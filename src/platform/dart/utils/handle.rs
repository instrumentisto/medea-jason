//! Definition and implementation of the wrapper around [`Dart_Handle`] which
//! manages lifetimes of the [`Dart_PersistentHandle`].

use std::{fmt, os::raw::c_char, ptr, rc::Rc};

use dart_sys::{Dart_Handle, Dart_PersistentHandle};

use crate::{
    api::{c_str_into_string, free_dart_native_string},
    platform::dart::utils::dart_api::{
        Dart_DeletePersistentHandle_DL_Trampolined,
        Dart_HandleFromPersistent_DL_Trampolined,
        Dart_NewPersistentHandle_DL_Trampolined,
    },
};

/// Pointer to an extern function that returns string representation
type RuntimeTypeFunction = extern "C" fn(Dart_Handle) -> ptr::NonNull<c_char>;

/// Pointer to an extern function that returns message of the provided Dart
/// error.
type ToStringFunction = extern "C" fn(Dart_Handle) -> ptr::NonNull<c_char>;

/// Stores pointer to the [`RuntimeTypeFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut RUNTIME_TYPE_FUNCTION: Option<RuntimeTypeFunction> = None;

/// Stores pointer to the [`ToStringFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut TO_STRING_FUNCTION: Option<ToStringFunction> = None;

/// Registers the provided [`RuntimeTypeFunction`] as [`RUNTIME_TYPE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Object__runtimeType__toString(
    f: RuntimeTypeFunction,
) {
    RUNTIME_TYPE_FUNCTION = Some(f);
}

/// Registers the provided [`ToStringFunction`] as [`TO_STRING_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Object__toString(f: ToStringFunction) {
    TO_STRING_FUNCTION = Some(f);
}

/// Reference-counting based [`Dart_Handle`] wrapper that takes care of its
/// lifetime management.
#[derive(Clone, Debug, PartialEq)]
pub struct DartHandle(Rc<Dart_PersistentHandle>);

impl DartHandle {
    /// Wraps provided [`Dart_Handle`] into this [`DartHandle`].
    ///
    /// Takes ownership of a provided [`Dart_Handle`] so it wont get freed by
    /// Dart VM.
    pub fn new(handle: Dart_Handle) -> Self {
        Self(Rc::new(unsafe {
            Dart_NewPersistentHandle_DL_Trampolined(handle)
        }))
    }

    /// Returns the underlying [`Dart_Handle`].
    #[must_use]
    pub fn get(&self) -> Dart_Handle {
        // SAFETY: We don't expose the inner `Dart_PersistentHandle` anywhere,
        //         so we're sure that it's valid at this point.
        unsafe { Dart_HandleFromPersistent_DL_Trampolined(*self.0) }
    }

    /// Returns runtime Dart type of this [`DartHandle`].
    #[must_use]
    pub fn name(&self) -> String {
        unsafe {
            let raw = RUNTIME_TYPE_FUNCTION.unwrap()(self.get());
            let name = c_str_into_string(raw);
            free_dart_native_string(raw);

            name
        }
    }
}

impl fmt::Display for DartHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let raw = TO_STRING_FUNCTION.unwrap()(self.get());
            let to_string = c_str_into_string(raw);
            free_dart_native_string(raw);

            write!(f, "{}", to_string)
        }
    }
}

impl From<Dart_Handle> for DartHandle {
    fn from(handle: Dart_Handle) -> Self {
        Self::new(handle)
    }
}

impl Drop for DartHandle {
    #[inline]
    fn drop(&mut self) {
        if Rc::strong_count(&self.0) == 1 {
            unsafe {
                Dart_DeletePersistentHandle_DL_Trampolined(*self.0);
            }
        }
    }
}
