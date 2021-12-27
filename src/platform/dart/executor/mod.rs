//! Executor of [`Future`]s for the Dart environment.

mod task;

use std::{future::Future, ptr, rc::Rc};

use dart_sys::{Dart_CObject, Dart_CObjectValue, Dart_CObject_Type, Dart_Port};

use crate::platform::dart::utils::dart_api::Dart_PostCObject_DL_Trampolined;

use self::task::Task;

/// Runs a Rust [`Future`] on the current thread.
pub fn spawn(fut: impl Future<Output = ()> + 'static) {
    Task::spawn(Box::pin(fut));
}

/// A [`Dart_Port`] used to send [`Task`]'s poll commands so Dart will poll Rust
/// [`Future`]s.
///
/// Must be initialized with the [`rust_executor_init()`] function during FFI
/// initialization.
static mut WAKE_PORT: Option<Dart_Port> = None;

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
    WAKE_PORT = Some(wake_port);
}

/// Polls an incomplete [`Task`].
///
/// # Safety
///
/// Valid [`Task`] pointer must be provided. Must not be called if the provided
/// [`Task`] has been dropped (with the [`rust_executor_drop_task()`] function).
///
/// [`Pending`]: std::task::Poll::Pending
/// [`Ready`]: std::task::Poll::Ready
#[no_mangle]
pub unsafe extern "C" fn rust_executor_poll_task(task: ptr::NonNull<Task>) {
    let task = Rc::from_raw(task.as_ptr()).poll();
}

/// Commands an external Dart executor to poll the provided [`Task`].
///
/// Sends command that contains the provided [`Task`] to the configured
/// [`WAKE_PORT`]. When received, Dart must poll it by calling the
/// [`rust_executor_poll_task()`] function.
fn task_wake(task: Rc<Task>) {
    let wake_port = unsafe { WAKE_PORT }.unwrap();
    let task = Rc::into_raw(task);

    let mut task_addr = Dart_CObject {
        type_: Dart_CObject_Type::Int64,
        value: Dart_CObjectValue {
            as_int64: task as i64,
        },
    };

    let enqueued =
        unsafe { Dart_PostCObject_DL_Trampolined(wake_port, &mut task_addr) };
    if !enqueued {
        log::warn!("Could not send message to Dart's native port");
        unsafe {
            drop(Rc::from_raw(task));
        }
    }
}
