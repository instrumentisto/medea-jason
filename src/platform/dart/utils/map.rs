//! Definitions and implementation of the Rust side representation of the Dart
//! side Map.

use std::{os::raw::c_char, ptr};

use dart_sys::Dart_Handle;

use crate::{
    api::string_into_c_str, platform::dart::utils::handle::DartHandle,
};

/// Rust representation of the Dart side Map.
pub struct DartMap(DartHandle);

impl From<DartMap> for Value {
    fn from(from: DartMap) -> Self {
        Self::Map(from)
    }
}

impl From<String> for Value {
    fn from(from: String) -> Self {
        Self::String(from)
    }
}

impl From<i32> for Value {
    fn from(from: i32) -> Self {
        Self::Int(from)
    }
}

impl From<DartMap> for Dart_Handle {
    fn from(from: DartMap) -> Self {
        from.0.get()
    }
}

/// Pointer to an extern function that returns [`Dart_Handle`] to the newly
/// created Dart `Map`.
type NewFunction = extern "C" fn() -> Dart_Handle;

/// Stores pointer to the [`NewFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut NEW_FUNCTION: Option<NewFunction> = None;

/// Registers the provided [`NewFunction`] as [`NEW_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_DartMap__new(f: NewFunction) {
    NEW_FUNCTION = Some(f);
}

/// Pointer to an extern function that sets provided [`Dart_Handle`] with a
/// provided [`c_char`] key to the provided [`Dart_Handle`] `Map`.
type SetFunction =
    extern "C" fn(Dart_Handle, ptr::NonNull<c_char>, Dart_Handle);

/// Stores pointer to the [`SetFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut SET_FUNCTION: Option<SetFunction> = None;

/// Registers the provided [`SetFunction`] as [`SET_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_DartMap__set(f: SetFunction) {
    SET_FUNCTION = Some(f);
}

/// Pointer to an extern function that removes value with a provided [`c_char`]
/// key from the provided [`Dart_Handle`] `Map`.
type RemoveFunction = extern "C" fn(Dart_Handle, ptr::NonNull<c_char>);

/// Stores pointer to the [`RemoveFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut REMOVE_FUNCTION: Option<RemoveFunction> = None;

/// Registers the provided [`RemoveFunction`] as [`REMOVE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_DartMap__remove(f: RemoveFunction) {
    REMOVE_FUNCTION = Some(f);
}

impl Default for DartMap {
    fn default() -> Self {
        Self::new()
    }
}

impl DartMap {
    /// Returns new Dart `Map`.
    #[must_use]
    pub fn new() -> Self {
        Self(DartHandle::new(unsafe { NEW_FUNCTION.unwrap()() }))
    }

    /// Sets provided [`Value`] to the provided `key`.
    pub fn set(&self, key: String, value: Value) {
        unsafe {
            SET_FUNCTION.unwrap()(
                self.0.get(),
                string_into_c_str(key),
                value.into(),
            );
        }
    }

    /// Removes [`Value`] with a provided `key` from this [`DartMap`].
    pub fn remove(&self, key: String) {
        unsafe {
            REMOVE_FUNCTION.unwrap()(self.0.get(), string_into_c_str(key));
        }
    }
}

/// Pointer to an extern function that return [`Dart_Handle`] to the Dart
/// `String` created from the provided [`c_char`].
type NewStringFunction = extern "C" fn(ptr::NonNull<c_char>) -> Dart_Handle;

/// Stores pointer to the [`NewStringFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut NEW_STRING_FUNCTION: Option<NewStringFunction> = None;

/// Registers the provided [`NewStringFunction`] as [`NEW_STRING_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_String__new(f: NewStringFunction) {
    NEW_STRING_FUNCTION = Some(f);
}

/// Pointer to an extern function that return [`Dart_Handle`] to the Dart `int`
/// created from the provided [`i32`].
type NewIntFunction = extern "C" fn(i32) -> Dart_Handle;

/// Stores pointer to the [`NewIntFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut NEW_INT_FUNCTION: Option<NewIntFunction> = None;

/// Registers the provided [`NewIntFunction`] as [`NEW_INT_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Int__new(f: NewIntFunction) {
    NEW_INT_FUNCTION = Some(f);
}

/// All possible values of the [`DartMap`].
pub enum Value {
    /// Another [`DartMap`].
    Map(DartMap),

    /// [`String`] value.
    String(String),

    /// [`i32] value.
    Int(i32),
}

impl From<Value> for Dart_Handle {
    fn from(from: Value) -> Self {
        match from {
            Value::Map(h) => h.0.get(),
            Value::String(s) => unsafe {
                NEW_STRING_FUNCTION.unwrap()(string_into_c_str(s))
            },
            Value::Int(i) => unsafe { NEW_INT_FUNCTION.unwrap()(i) },
        }
    }
}
