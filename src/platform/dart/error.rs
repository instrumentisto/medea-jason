//! Wrapper for Dart exceptions.

use std::{os::raw::c_char, ptr};

use dart_sys::Dart_Handle;
use derive_more::Display;

use crate::api::c_str_into_string;

use super::utils::handle::DartHandle;

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
pub unsafe extern "C" fn register_Error__name(f: NameFunction) {
    NAME_FUNCTION = Some(f);
}

/// Registers the provided [`MessageFunction`] as [`MESSAGE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Error__message(f: MessageFunction) {
    MESSAGE_FUNCTION = Some(f);
}

/// Wrapper for Dart exception thrown when calling Dart code.
#[derive(Clone, Display, Debug, PartialEq)]
#[display(fmt = "DartPlatformError")]
pub struct Error(DartHandle);

impl Error {
    /// Returns a [`Dart_Handle`] to the underlying error.
    #[must_use]
    pub fn get_handle(&self) -> Dart_Handle {
        self.0.get()
    }

    /// Returns name of the underlying Dart exception.
    #[must_use]
    pub fn name(&self) -> String {
        unsafe { c_str_into_string(NAME_FUNCTION.unwrap()(self.0.get())) }
    }

    /// Returns message of the underlying Dart exception.
    #[must_use]
    pub fn message(&self) -> String {
        unsafe { c_str_into_string(MESSAGE_FUNCTION.unwrap()(self.0.get())) }
    }
}

impl From<Dart_Handle> for Error {
    fn from(from: Dart_Handle) -> Self {
        Self(DartHandle::new(from))
    }
}
