//! Executor of [`Future`]s for the Dart environment.

mod task;

use std::{future::Future, ptr, rc::Rc, thread};

use dart_sys::{
    Dart_CObject, Dart_CObject_Type_Dart_CObject_kInt64, Dart_Port,
    _Dart_CObject__bindgen_ty_1,
};
use sync_unsafe_cell::SyncUnsafeCell;

use crate::{
    api::propagate_panic,
    platform::{utils::dart_api, DART_MAIN_THREAD},
};

pub use self::task::Task;

/// Runs a Rust [`Future`] on the current thread.
pub fn spawn(fut: impl Future<Output = ()> + 'static) {
    Task::spawn(Box::pin(fut));
}

/// [`Dart_Port`] used to send [`Task`]'s poll commands so Dart will poll Rust
/// [`Future`]s.
///
/// Must be initialized with the [`rust_executor_init()`] function during FFI
/// initialization.
static WAKE_PORT: SyncUnsafeCell<Option<i64>> = SyncUnsafeCell::new(None);

/// Initializes Dart-driven async [`Task`] executor.
///
/// On a Dart side you should continuously read channel to get [`Task`]s
/// addresses for polling.
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn rust_executor_init(wake_port: Dart_Port) {
    unsafe {
        *WAKE_PORT.get() = Some(wake_port);
    }
}

/// Polls the provided [`Task`].
///
/// # Safety
///
/// Valid [`Task`] pointer must be provided.
#[no_mangle]
pub unsafe extern "C" fn rust_executor_poll_task(task: ptr::NonNull<Task>) {
    propagate_panic(move || {
        _ = unsafe { Rc::from_raw(task.as_ptr()).poll() };
    });
}

/// Commands an external Dart executor to poll the provided [`Task`].
///
/// Sends command that contains the provided [`Task`] to the configured
/// [`WAKE_PORT`]. When received, Dart must poll it by calling the
/// [`rust_executor_poll_task()`] function.
fn task_wake(task: Rc<Task>) {
    assert_eq!(
        Some(thread::current().id()),
        *DART_MAIN_THREAD.lock().unwrap(),
        "Futures executor must be run on a single thread"
    );

    let wake_port = unsafe { *WAKE_PORT.get() }.unwrap();
    let task = Rc::into_raw(task);

    let mut task_addr = Dart_CObject {
        type_: Dart_CObject_Type_Dart_CObject_kInt64,
        value: _Dart_CObject__bindgen_ty_1 {
            as_int64: task as i64,
        },
    };

    let enqueued =
        unsafe { dart_api::post_c_object(wake_port, &mut task_addr) };
    if !enqueued {
        log::warn!("Could not send message to Dart's native port");
        unsafe {
            drop(Rc::from_raw(task));
        }
    }
}
