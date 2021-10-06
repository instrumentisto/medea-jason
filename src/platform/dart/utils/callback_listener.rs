//! Definitions and implementations of the Dart callback listeners.

use std::{os::raw::c_char, ptr};

use dart_sys::Dart_Handle;

use crate::{
    api::c_str_into_string, platform::dart::utils::handle::DartHandle,
};

/// Pointer to an extern function that returns a [`Dart_Handle`] to a newly
/// created Dart callback with a `void` as argument which will call Rust side
/// callback when Dart side callback will be fired.
type VoidCallbackFunction = extern "C" fn(*mut VoidCallback) -> Dart_Handle;

/// Stores pointer to the [`VoidCallbackFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut VOID_CALLBACK_FUNCTION: Option<VoidCallbackFunction> = None;

/// Registers the provided [`VoidCallbackFunction`] as
/// [`VOID_CALLBACK_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_VoidCallback__callback(
    f: VoidCallbackFunction,
) {
    VOID_CALLBACK_FUNCTION = Some(f);
}

/// Listener for the Dart callback with `void` as argument.
pub struct VoidCallback(Box<dyn FnOnce()>);

impl VoidCallback {
    /// Returns [`Dart_Handle`] to the Dart callback which will call provided
    /// `f` closure when it will be called on Dart side.
    pub fn callback<F>(f: F) -> Dart_Handle
    where
        F: FnOnce() + 'static,
    {
        let this = Self(Box::new(f));
        unsafe {
            VOID_CALLBACK_FUNCTION.unwrap()(Box::into_raw(Box::new(this)))
        }
    }
}

/// Notifies underlying closure of the [`VoidCallback`] about Dart side callback
/// firing.
#[no_mangle]
pub unsafe extern "C" fn VoidCallback__call(cb: *mut VoidCallback) {
    let cb = Box::from_raw(cb);
    cb.0();
}

/// Passes provided `val` argument to the [`StringCallback`]'s underlying
/// closure.
///
/// This function is used for notifying Rust side about Dart side callback
/// firing. All arguments of the fired Dart callback should be passed to this
/// function.
#[no_mangle]
pub unsafe extern "C" fn StringCallback__call(
    cb: *const StringCallback,
    val: ptr::NonNull<c_char>,
) {
    let s = c_str_into_string(val);
    let cb = cb.as_ref().unwrap();
    cb.0(s);
}

/// Pointer to an extern function that returns a [`Dart_Handle`] to a newly
/// created Dart callback with a `String` as argument which will call Rust side
/// callback when Dart side callback will be fired.
type StringCallbackFunction =
    extern "C" fn(*const StringCallback) -> Dart_Handle;

/// Stores pointer to the [`StringCallbackFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut STRING_CALLBACK_FUNCTION: Option<StringCallbackFunction> = None;

/// Registers the provided [`StringCallbackFunction`] as
/// [`STRING_CALLBACK_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_StringCallback__callback(
    f: StringCallbackFunction,
) {
    STRING_CALLBACK_FUNCTION = Some(f);
}

/// Listener for the Dart callback with `String` as argument.
pub struct StringCallback(Box<dyn Fn(String)>);

impl StringCallback {
    /// Returns [`Dart_Handle`] to the Dart callback which will call provided
    /// `f` closure when it will be called on Dart side.
    ///
    /// Argument with which Dart side callback will be called will be passed to
    /// the provided `f` closure.
    pub fn callback<F>(f: F) -> DartHandle
    where
        F: Fn(String) + 'static,
    {
        let this = Self(Box::new(f));
        unsafe {
            DartHandle::new(STRING_CALLBACK_FUNCTION.unwrap()(Box::into_raw(
                Box::new(this),
            )))
        }
    }
}

/// Pointer to an extern function that returns a [`Dart_Handle`] to a newly
/// created Dart callback with a mutable pointer to the [`Dart_Handle`] as
/// argument which will call Rust side callback when Dart side callback will be
/// fired.
type HandleMutCallbackFunction =
    extern "C" fn(*mut HandleMutCallback) -> Dart_Handle;

/// Stores pointer to the [`HandleMutCallbackFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut HANDLE_MUT_CALLBACK_FUNCTION: Option<HandleMutCallbackFunction> =
    None;

/// Registers the provided [`HandleMutCallbackFunction`] as
/// [`HANDLE_MUT_CALLBACK_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_HandleMutCallback__callback(
    f: HandleMutCallbackFunction,
) {
    HANDLE_MUT_CALLBACK_FUNCTION = Some(f);
}

/// Passes provided `val` argument to the [`HandleMutCallback`]'s underlying
/// closure.
///
/// This function is used for notifying Rust side about Dart side callback
/// firing. All arguments of the fired Dart callback should be passed to this
/// function.
#[no_mangle]
pub unsafe extern "C" fn HandleMutCallback__call(
    cb: *mut HandleMutCallback,
    val: Dart_Handle,
) {
    (*cb).0(val);
}

/// Listener for the Dart callback with a mutable pointer to [`Dart_Handle`] as
/// argument.
pub struct HandleMutCallback(Box<dyn FnMut(Dart_Handle)>);

impl HandleMutCallback {
    /// Returns [`Dart_Handle`] to the Dart callback which will call provided
    /// `f` closure when it will be called on Dart side.
    ///
    /// Argument with which Dart side callback will be called will be passed to
    /// the provided `f` closure.
    pub fn callback<F>(f: F) -> Dart_Handle
    where
        F: FnMut(Dart_Handle) + 'static,
    {
        let this = Self(Box::new(f));
        unsafe {
            HANDLE_MUT_CALLBACK_FUNCTION.unwrap()(Box::into_raw(Box::new(this)))
        }
    }
}

/// Pointer to an extern function that returns a [`Dart_Handle`] to a newly
/// created Dart callback with a pointer to the [`Dart_Handle`] as argument
/// which will call Rust side callback when Dart side callback will be fired.
type HandleCallbackFunction = extern "C" fn(*mut HandleCallback) -> Dart_Handle;

/// Stores pointer to the [`HandleCallbackFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut HANDLE_CALLBACK_FUNCTION: Option<HandleCallbackFunction> = None;

/// Registers the provided [`HandleCallbackFunction`] as
/// [`HANDLE_CALLBACK_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_HandleCallback__callback(
    f: HandleCallbackFunction,
) {
    HANDLE_CALLBACK_FUNCTION = Some(f);
}

/// Passes provided `val` argument to the [`HandleCallback`]'s underlying
/// closure.
///
/// This function is used for notifying Rust side about Dart side callback
/// firing. All arguments of the fired Dart callback should be passed to this
/// function.
#[no_mangle]
pub unsafe extern "C" fn HandleCallback__call(
    cb: *mut HandleCallback,
    handle: Dart_Handle,
) {
    let cb = Box::from_raw(cb);
    cb.0(handle);
}

/// Listener for the Dart callback with a pointer to [`Dart_Handle`] as
/// argument.
pub struct HandleCallback(Box<dyn Fn(Dart_Handle)>);

impl HandleCallback {
    /// Returns [`Dart_Handle`] to the Dart callback which will call provided
    /// `f` closure when it will be called on Dart side.
    ///
    /// Argument with which Dart side callback will be called will be passed to
    /// the provided `f` closure.
    pub fn callback<F>(f: F) -> Dart_Handle
    where
        F: Fn(Dart_Handle) + 'static,
    {
        let this = Self(Box::new(f));
        unsafe {
            HANDLE_CALLBACK_FUNCTION.unwrap()(Box::into_raw(Box::new(this)))
        }
    }
}

/// Pointer to an extern function that returns a [`Dart_Handle`] to a newly
/// created Dart callback with a `Int32` as argument which will call Rust side
/// callback when Dart side callback will be fired.
type IntCallbackFunction = extern "C" fn(*mut IntCallback) -> Dart_Handle;

/// Stores pointer to the [`IntCallbackFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut INT_CALLBACK_FUNCTION: Option<IntCallbackFunction> = None;

/// Registers the provided [`IntCallbackFunction`] as [`INT_CALLBACK_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_IntCallback__callback(
    f: IntCallbackFunction,
) {
    INT_CALLBACK_FUNCTION = Some(f);
}

/// Passes provided `val` argument to the [`IntCallback`]'s underlying closure.
///
/// This function is used for notifying Rust side about Dart side callback
/// firing. All arguments of the fired Dart callback should be passed to this
/// function.
#[no_mangle]
pub unsafe extern "C" fn IntCallback__call(cb: *mut IntCallback, val: i32) {
    (*cb).0(val);
}

/// Listener for the Dart callback with `Int32` as argument.
pub struct IntCallback(Box<dyn FnMut(i32)>);

impl IntCallback {
    /// Returns [`Dart_Handle`] to the Dart callback which will call provided
    /// `f` closure when it will be called on Dart side.
    ///
    /// Argument with which Dart side callback will be called will be passed to
    /// the provided `f` closure.
    pub fn callback<F>(f: F) -> Dart_Handle
    where
        F: FnMut(i32) + 'static,
    {
        let this = Self(Box::new(f));
        unsafe { INT_CALLBACK_FUNCTION.unwrap()(Box::into_raw(Box::new(this))) }
    }
}

/// Pointer to an extern function that returns a [`Dart_Handle`] to a newly
/// created Dart callback with a two pointers to the [`Dart_Handle`]s as
/// argument which will call Rust side callback when Dart side callback will be
/// fired.
type TwoArgCallbackFunction = extern "C" fn(*mut TwoArgCallback) -> Dart_Handle;

/// Stores pointer to the [`TwoArgCallbackFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut TWO_ARG_CALLBACK_FUNCTION: Option<TwoArgCallbackFunction> = None;

/// Registers the provided [`TwoArgCallbackFunction`] as
/// [`TWO_ARG_CALLBACK_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_TwoArgCallback__callback(
    f: TwoArgCallbackFunction,
) {
    TWO_ARG_CALLBACK_FUNCTION = Some(f);
}

/// Passes provided `first` and `second` arguments to the [`TwoArgCallback`]'s
/// underlying closure.
///
/// This function is used for notifying Rust side about Dart side callback
/// firing. All arguments of the fired Dart callback should be passed to this
/// function.
#[no_mangle]
pub unsafe extern "C" fn TwoArgCallback__call(
    cb: *mut TwoArgCallback,
    first: Dart_Handle,
    second: Dart_Handle,
) {
    (*cb).0(first, second);
}

/// Listener for the Dart callback with a two pointers to [`Dart_Handle`]s as
/// arguments.
pub struct TwoArgCallback(Box<dyn FnMut(Dart_Handle, Dart_Handle)>);

impl TwoArgCallback {
    /// Returns [`Dart_Handle`] to the Dart callback which will call provided
    /// `f` closure when it will be called on Dart side.
    ///
    /// Arguments with which Dart side callback will be called will be passed to
    /// the provided `f` closure.
    pub fn callback<F>(f: F) -> Dart_Handle
    where
        F: FnMut(Dart_Handle, Dart_Handle) + 'static,
    {
        let this = Self(Box::new(f));
        unsafe {
            TWO_ARG_CALLBACK_FUNCTION.unwrap()(Box::into_raw(Box::new(this)))
        }
    }
}
