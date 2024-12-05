//! Helper functionality for passing [`String`]s through FFI boundaries.

use std::{
    cell::RefCell,
    ffi::{CStr, CString},
    os::raw::c_char,
    ptr,
};

use crate::api::propagate_panic;

/// Pointer to an extern function that frees the provided Dart native string.
type FreeDartNativeStringFunction = extern "C" fn(ptr::NonNull<c_char>);

thread_local! {
    /// Stores a pointer to the [`FreeDartNativeStringFunction`] extern
    /// function.
    ///
    /// Must be initialized by Dart during FFI initialization phase.
    static FREE_DART_NATIVE_STRING: RefCell<
        Option<FreeDartNativeStringFunction>
    > = RefCell::default();
}

/// Constructs a Rust [`String`] from the provided raw C string.
///
/// # Panics
///
/// If the provided C string UTF-8 validation fails.
///
/// # Safety
///
/// Same as for [`CStr::from_ptr()`].
#[must_use]
pub unsafe fn c_str_into_string(string: ptr::NonNull<c_char>) -> String {
    unsafe { CStr::from_ptr(string.as_ptr()) }
        .to_str()
        .unwrap()
        .to_owned()
}

/// Leaks the given [`String`] returning a raw C string that can be passed
/// through FFI boundaries.
///
/// The pointer (returned by this function) must be returned to Rust and
/// reconstituted via [`CString::from_raw()`] for proper deallocating.
///
/// # Panics
///
/// If the provided [`String`] contains an internal `0x0` byte.
#[must_use]
pub fn string_into_c_str(string: String) -> ptr::NonNull<c_char> {
    ptr::NonNull::new(CString::new(string).unwrap().into_raw()).unwrap()
}

/// Converts the provided C-string received from Dart into a Rust [`String`].
///
/// # Safety
///
/// The provided value must represent a valid C-string (`Pointer<Utf8>`
/// allocated on Dart-side).
#[must_use]
pub unsafe fn dart_string_into_rust(
    dart_string: ptr::NonNull<c_char>,
) -> String {
    let rust_string = unsafe { c_str_into_string(dart_string) };
    unsafe {
        free_dart_native_string(dart_string);
    }

    rust_string
}

/// Retakes ownership over a [`CString`] previously transferred to Dart via
/// [`CString::into_raw()`].
///
/// # Safety
///
/// Same as for [`CString::from_raw()`].
#[no_mangle]
pub unsafe extern "C" fn String_free(s: ptr::NonNull<c_char>) {
    propagate_panic(move || {
        drop(unsafe { CString::from_raw(s.as_ptr()) });
    });
}

/// Registers the provided [`FreeDartNativeStringFunction`] as
/// [`FREE_DART_NATIVE_STRING`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_free_dart_native_string(
    f: FreeDartNativeStringFunction,
) {
    FREE_DART_NATIVE_STRING.set(Some(f));
}

/// Calls Dart to release memory allocated for the provided native string.
///
/// Should be used when Dart cannot release memory in place, e.g when Rust calls
/// a Dart function returning a native string.
///
/// # Safety
///
/// `FREE_DART_NATIVE_STRING` function must be registered and the provided
/// pointer must be a valid native string.
pub unsafe fn free_dart_native_string(s: ptr::NonNull<c_char>) {
    FREE_DART_NATIVE_STRING.with_borrow(|f| (f.unwrap())(s));
}
