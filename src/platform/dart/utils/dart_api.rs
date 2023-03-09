//! Functionality for calling [`Dart DL API`] from Rust.
//!
//! [`Dart DL API`]: https://tinyurl.com/32e7fudh

use dart_sys::{
    Dart_CObject, Dart_DeletePersistentHandle_DL, Dart_FinalizableHandle,
    Dart_GetError_DL, Dart_Handle, Dart_HandleFinalizer,
    Dart_HandleFromPersistent_DL, Dart_InitializeApiDL, Dart_IsError_DL,
    Dart_NewFinalizableHandle_DL, Dart_NewPersistentHandle_DL,
    Dart_PersistentHandle, Dart_Port_DL, Dart_PostCObject_DL,
    Dart_PropagateError_DL,
};

/// Initializes usage of Dynamically Linked Dart API.
///
/// # Safety
///
/// Intended to be called ONLY with [`NativeApi.initializeApiDLData`][1] from
/// Dart.
///
/// [1]: https://api.dart.dev/dart-ffi/NativeApi/initializeApiDLData.html
pub unsafe fn initialize_api(data: *mut core::ffi::c_void) -> isize {
    Dart_InitializeApiDL(data)
}

/// Allocates a [`Dart_PersistentHandle`] for provided [`Dart_Handle`].
///
/// [`Dart_PersistentHandle`]s have the lifetime of the current isolate
/// unless they are explicitly deallocated.
///
/// # Safety
///
/// [`initialize_api`] must be called before this function.
pub unsafe fn new_persistent_handle(
    object: Dart_Handle,
) -> Dart_PersistentHandle {
    #[allow(clippy::expect_used)]
    Dart_NewPersistentHandle_DL.expect("dart_api_dl has not been initialized")(
        object,
    )
}

/// Allocates a [`Dart_Handle`] in the current scope from the given
/// [`Dart_PersistentHandle`].
///
/// This doesn't affect the provided [`Dart_PersistentHandle`]'s lifetime.
///
/// # Safety
///
/// [`initialize_api`] must be called before this function.
pub unsafe fn handle_from_persistent(
    object: Dart_PersistentHandle,
) -> Dart_Handle {
    #[allow(clippy::expect_used)]
    Dart_HandleFromPersistent_DL.expect("dart_api_dl has not been initialized")(
        object,
    )
}

/// Deallocates the provided [`Dart_PersistentHandle`].
///
/// # Safety
///
/// [`initialize_api`] must be called before this function.
pub unsafe fn delete_persistent_handle(object: Dart_Handle) {
    #[allow(clippy::expect_used)]
    Dart_DeletePersistentHandle_DL
        .expect("dart_api_dl has not been initialized")(object);
}

/// Posts a `message` on some port. It will contain a [`Dart_CObject`]
/// object graph rooted in the `message`.
///
/// While the `message` is being sent the state of the graph of
/// [`Dart_CObject`] structures rooted in the `message` should not be
/// accessed, as the message generation will make temporary modifications to
/// the data. When the message has been sent the graph will be fully
/// restored.
///
/// If `true` is returned, the `message` was enqueued, and finalizers for
/// external typed data will eventually run, even if the receiving isolate
/// shuts down before processing the `message`. If `false` is returned, the
/// `message` was not enqueued and ownership of external typed data in the
/// `message` remains with the caller.
///
/// # Safety
///
/// [`initialize_api`] must be called before this function.
pub unsafe fn post_c_object(
    port_id: Dart_Port_DL,
    message: *mut Dart_CObject,
) -> bool {
    #[allow(clippy::expect_used)]
    Dart_PostCObject_DL.expect("dart_api_dl has not been initialized")(
        port_id, message,
    )
}

/// Allocates a finalizable handle for an object.
///
/// This handle has the lifetime of the current isolate group unless the
/// object pointed to by the handle is garbage collected, in this case the
/// VM automatically deletes the handle after invoking the callback
/// associated with the handle.
///
/// Once finalizable handle is collected by GC, the provided `callback` is
/// called.
///
/// `peer` argument will be provided to the `callback` on finalize.
///
/// `external_allocation_size` is a size of the `peer` which can be
/// calculated via [`mem::size_of()`].
///
/// # Safety
///
/// [`initialize_api`] must be called before this function.
///
/// [`mem::size_of()`]: std::mem::size_of
pub unsafe fn new_finalizable_handle(
    object: Dart_Handle,
    peer: *mut ::core::ffi::c_void,
    external_allocation_size: isize,
    callback: Dart_HandleFinalizer,
) -> Dart_FinalizableHandle {
    #[allow(clippy::expect_used)]
    Dart_NewFinalizableHandle_DL.expect("dart_api_dl has not been initialized")(
        object,
        peer,
        external_allocation_size,
        callback,
    )
}

/// Checks whether the provided [`Dart_Handle`] represents a Dart error.
///
/// Should be called on the current isolate.
///
/// # Safety
///
/// [`initialize_api`] must be called before this function.
pub unsafe fn is_error(handle: Dart_Handle) -> bool {
    #[allow(clippy::expect_used)]
    Dart_IsError_DL.expect("dart_api_dl has not been initialized")(handle)
}

/// Returns the error message from the provided Dart error handle.
///
/// Should be called on the current isolate.
///
/// Returns a C string containing a Dart error message if the provided
/// `object` represents a Dart error, or an empty C string ("") otherwise.
///
/// # Safety
///
/// [`initialize_api`] must be called before this function.
pub unsafe fn get_error(handle: Dart_Handle) -> *const core::ffi::c_char {
    #[allow(clippy::expect_used)]
    Dart_GetError_DL.expect("dart_api_dl has not been initialized")(handle)
}

/// Propagates the given Dart error to the Dart side.
///
/// If the provided [`Dart_Handle`] is an unhandled exception error, then it
/// will be rethrown in the standard way: walking up Dart frames until an
/// appropriate `catch` block is found, than executing `finally` blocks, and so
/// on.
///
/// # Safety
///
/// Intended to be called ONLY with [`NativeApi.initializeApiDLData`][1] from
/// Dart.
///
/// [`initialize_api`] must be called before this function.
///
/// [1]: https://api.dart.dev/dart-ffi/NativeApi/initializeApiDLData.html
pub unsafe fn propagate_error(handle: Dart_Handle) {
    #[allow(clippy::expect_used)]
    Dart_PropagateError_DL.expect("dart_api_dl has not been initialized")(
        handle,
    );
}
