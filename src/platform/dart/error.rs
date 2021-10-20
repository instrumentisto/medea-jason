//! Wrapper for Dart exceptions.

use std::{os::raw::c_char, ptr, rc::Rc};

use dart_sys::{Dart_Handle, Dart_PersistentHandle};
use derive_more::Display;

use crate::api::c_str_into_string;

use super::utils::{
    dart_api::{
        Dart_DeletePersistentHandle_DL_Trampolined,
        Dart_HandleFromPersistent_DL_Trampolined,
        Dart_NewPersistentHandle_DL_Trampolined,
    },
    handle::DartHandle,
};

/// Wrapper for Dart exception thrown when calling Dart code.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "DartPlatformError")]
pub struct Error(Rc<Dart_PersistentHandle>);

impl Error {
    /// Returns a [`Dart_Handle`] to the underlying error.
    #[inline]
    #[must_use]
    pub fn get_handle(&self) -> Dart_Handle {
        // SAFETY: We don't expose the inner `Dart_PersistentHandle` anywhere,
        //         so we're sure that it's valid at this point.
        unsafe { Dart_HandleFromPersistent_DL_Trampolined(*self.0) }
    }
}

impl From<Dart_Handle> for Error {
    #[inline]
    fn from(err: Dart_Handle) -> Self {
        Self(Rc::new(unsafe {
            Dart_NewPersistentHandle_DL_Trampolined(err)
        }))
    }
}

impl Drop for Error {
    #[inline]
    fn drop(&mut self) {
        if Rc::strong_count(&self.0) == 1 {
            unsafe {
                Dart_DeletePersistentHandle_DL_Trampolined(*self.0);
            }
        }
    }
}

/// Pointer to an extern function that returns name of the provided Dart error.
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
pub unsafe extern "C" fn register_DartError__name(f: NameFunction) {
    NAME_FUNCTION = Some(f);
}

/// Registers the provided [`MessageFunction`] as [`MESSAGE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_DartError__message(f: MessageFunction) {
    MESSAGE_FUNCTION = Some(f);
}
