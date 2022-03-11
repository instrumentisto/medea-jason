//! Wrapper around [`Dart_Handle`] managing lifetimes of a
//! [`Dart_PersistentHandle`].

use std::{fmt, rc::Rc};

use dart_sys::{Dart_Handle, Dart_PersistentHandle};
use medea_macro::dart_bridge;

use crate::{
    api::{c_str_into_string, free_dart_native_string},
    platform::{
        dart::utils::dart_api::{
            Dart_DeletePersistentHandle_DL_Trampolined,
            Dart_HandleFromPersistent_DL_Trampolined,
            Dart_NewPersistentHandle_DL_Trampolined,
        },
        utils::dart_api::{
            Dart_GetError_DL_Trampolined, Dart_IsApiError_DL_Trampolined,
        },
    },
};

#[dart_bridge("flutter/lib/src/native/platform/object.g.dart")]
mod handle {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    extern "C" {
        /// Returns a string representation of a Dart type behind the provided
        /// [`Dart_Handle`].
        pub fn runtime_type(handle: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns a message of the provided Dart error.
        pub fn to_string(handle: Dart_Handle) -> ptr::NonNull<c_char>;
    }
}

/// Reference-counting based [`Dart_Handle`] wrapper taking care of its
/// lifetime management.
#[derive(Clone, Debug, PartialEq)]
pub struct DartHandle(Rc<Dart_PersistentHandle>);

impl DartHandle {
    /// Wraps the provided [`Dart_Handle`].
    ///
    /// Takes ownership of the provided [`Dart_Handle`] so it won't get freed by
    /// Dart VM.
    #[must_use]
    pub fn new(handle: Dart_Handle) -> Self {
        // TODO(alexlapa): not sure about Dart_IsApiError
        //                 why not Dart_IsError?
        if unsafe { Dart_IsApiError_DL_Trampolined(handle) } {
            let err_msg = unsafe {
                c_str_into_string(Dart_GetError_DL_Trampolined(handle))
            };
            panic!("Unexpected Dart error: {}", err_msg)
        }
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

    /// Returns string representation of a runtime Dart type behind this
    /// [`DartHandle`].
    #[must_use]
    pub fn name(&self) -> String {
        unsafe {
            let raw = handle::runtime_type(self.get());
            let name = c_str_into_string(raw);
            free_dart_native_string(raw);

            name
        }
    }
}

impl fmt::Display for DartHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let raw = handle::to_string(self.get());
            let to_string = c_str_into_string(raw);
            free_dart_native_string(raw);

            write!(f, "{to_string}")
        }
    }
}

impl From<Dart_Handle> for DartHandle {
    fn from(handle: Dart_Handle) -> Self {
        Self::new(handle)
    }
}

impl Drop for DartHandle {
    fn drop(&mut self) {
        if let Some(handle) = Rc::get_mut(&mut self.0) {
            unsafe {
                Dart_DeletePersistentHandle_DL_Trampolined(*handle);
            }
        }
    }
}
