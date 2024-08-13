//! Wrapper for Dart exceptions.

use dart_sys::Dart_Handle;
use derive_more::Display;

use super::utils::handle::DartHandle;

/// Wrapper for Dart exception thrown when calling Dart code.
#[derive(Clone, Debug, Display, Eq, PartialEq)]
#[display("DartPlatformError")]
pub struct Error(DartHandle);

impl Error {
    /// Creates a Dart [`Error`] out of the provided [`Dart_Handle`] from Dart
    /// side.
    ///
    /// # Safety
    ///
    /// The provided [`Dart_Handle`] should be non-`null` and point to the
    /// correct Dart exception.
    #[must_use]
    pub unsafe fn from_handle(h: Dart_Handle) -> Self {
        Self(unsafe { DartHandle::new(h) })
    }

    /// Returns a [`Dart_Handle`] to the underlying error.
    #[must_use]
    pub fn get_handle(&self) -> Dart_Handle {
        self.0.get()
    }

    /// Returns name of the underlying Dart exception.
    #[must_use]
    pub fn name(&self) -> String {
        self.0.name()
    }

    /// Returns message of the underlying Dart exception.
    #[must_use]
    pub fn message(&self) -> String {
        self.0.to_string()
    }
}
