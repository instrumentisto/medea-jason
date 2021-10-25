//! Wrapper for Dart exceptions.

use dart_sys::Dart_Handle;
use derive_more::Display;

use super::utils::handle::DartHandle;

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
        unimplemented!()

    }

    /// Returns message of the underlying Dart exception.
    #[must_use]
    pub fn message(&self) -> String {
        unimplemented!()
    }
}

impl From<Dart_Handle> for Error {
    fn from(from: Dart_Handle) -> Self {
        Self(DartHandle::new(from))
    }
}
