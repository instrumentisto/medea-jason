//! Executor of [`Future`]s for the Dart environment.

mod task;

use std::{
    ptr,
    sync::{Arc, atomic, atomic::AtomicI64},
};

use dart_sys::{
    _Dart_CObject__bindgen_ty_1, Dart_CObject,
    Dart_CObject_Type_Dart_CObject_kInt64, Dart_Port,
};

pub use self::task::Task;
use crate::{api::propagate_panic, platform::utils::dart_api};

/// Runs a Rust [`Future`] on the current thread.
pub fn spawn(fut: impl Future<Output = ()> + 'static) {
    Task::spawn(Box::pin(fut));
}

/// Atomic variant of a [`Dart_Port`].
type AtomicDartPort = AtomicI64;

/// [`Dart_Port`] used to send [`Task`]'s poll commands so Dart will poll Rust
/// [`Future`]s.
///
/// Must be initialized with the [`rust_executor_init()`] function during FFI
/// initialization.
static WAKE_PORT: AtomicDartPort = AtomicI64::new(0);

/// Initializes Dart-driven async [`Task`] executor.
///
/// On a Dart side you should continuously read channel to get [`Task`]s
/// addresses for polling.
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_executor_init(wake_port: Dart_Port) {
    WAKE_PORT.store(wake_port, atomic::Ordering::Release);
}

/// Polls the provided [`Task`].
///
/// # Safety
///
/// Valid [`Task`] pointer must be provided.
///
/// # Panics
///
/// If called not on the same thread where the [`Task`] was originally created.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_executor_poll_task(task: ptr::NonNull<Task>) {
    propagate_panic(move || unsafe { Arc::from_raw(task.as_ptr()).poll() });
}

/// Commands an external Dart executor to poll the provided [`Task`].
///
/// Sends command that contains the provided [`Task`] to the configured
/// [`WAKE_PORT`]. When received, Dart must poll it by calling the
/// [`rust_executor_poll_task()`] function.
///
/// # Panics
///
/// If Dart-driven async [`Task`] executor is not initialized.
fn task_wake(task: Arc<Task>) {
    let wake_port = WAKE_PORT.load(atomic::Ordering::Acquire);
    assert!(wake_port > 0, "`WAKE_PORT` address must be initialized");
    let task = Arc::into_raw(task);

    let mut task_addr = Dart_CObject {
        type_: Dart_CObject_Type_Dart_CObject_kInt64,
        value: _Dart_CObject__bindgen_ty_1 { as_int64: task as i64 },
    };

    let enqueued =
        unsafe { dart_api::post_c_object(wake_port, &raw mut task_addr) };
    if !enqueued {
        log::warn!("Could not send message to Dart's native port");
        unsafe {
            drop(Arc::from_raw(task));
        }
    }
}
