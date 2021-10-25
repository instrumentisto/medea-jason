use std::{
    ptr, os::raw::c_char
};

use dart_sys::Dart_Handle;

use crate::api::{c_str_into_string, free_dart_native_string};

/// Pointer to an extern function that returns string representation
type NameFunction = extern "C" fn(Dart_Handle) -> ptr::NonNull<c_char>;

/// Pointer to an extern function that returns message of the provided Dart
/// error.
type MessageFunction = extern "C" fn(Dart_Handle) -> ptr::NonNull<c_char>;

/// Stores pointer to the [`NameFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut NAME_FUNCTION: Option<NameFunction> = None;

/// Stores pointer to the [`MessageFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut MESSAGE_FUNCTION: Option<MessageFunction> = None;

/// Registers the provided [`NameFunction`] as [`NAME_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Object__runtimeType__toString(f: NameFunction) {
    NAME_FUNCTION = Some(f);
}

/// Registers the provided [`MessageFunction`] as [`MESSAGE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Object__toString(f: MessageFunction) {
    MESSAGE_FUNCTION = Some(f);
}

trait DartObject {
    unsafe fn name(&self) -> String;
    unsafe fn to_string(&self) -> String;
}

impl DartObject for Dart_Handle {
    unsafe fn name(&self) -> String {
        let raw = NAME_FUNCTION.unwrap()(self.0.get());
        let name = c_str_into_string(raw);
        free_dart_native_string(raw);

        name
    }

    unsafe fn to_string(&self) -> String {
        let raw = MESSAGE_FUNCTION.unwrap()(self.0.get());
        let to_string = c_str_into_string(raw);
        free_dart_native_string(raw);

        to_string
    }
}
