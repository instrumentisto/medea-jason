//! Definitions and implementation of the Rust side representation of the Dart
//! side Map.

use std::{os::raw::c_char, ptr};

use dart_sys::Dart_Handle;

use crate::{
    api::{string_into_c_str, DartValue},
    platform::dart::utils::handle::DartHandle,
};

/// Rust representation of the Dart side Map.
pub struct DartMap(DartHandle);

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
type SetFunction = extern "C" fn(Dart_Handle, ptr::NonNull<c_char>, DartValue);

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
    pub fn set(&self, key: String, value: DartValue) {
        unsafe {
            SET_FUNCTION.unwrap()(self.0.get(), string_into_c_str(key), value);
        }
    }

    /// Removes [`Value`] with a provided `key` from this [`DartMap`].
    pub fn remove(&self, key: String) {
        unsafe {
            REMOVE_FUNCTION.unwrap()(self.0.get(), string_into_c_str(key));
        }
    }

    /// Returns underlying [`Dart_Handle`] of this [`DartMap`].
    #[must_use]
    pub fn as_handle(&self) -> Dart_Handle {
        self.0.get()
    }
}
