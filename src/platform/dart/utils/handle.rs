//! Wrapper around [`Dart_Handle`] managing lifetimes of a
//! [`Dart_PersistentHandle`].

use std::{fmt, rc::Rc};

use dart_sys::{Dart_Handle, Dart_PersistentHandle};
use medea_macro::dart_bridge;

use crate::platform::utils::{
    c_str_into_string, dart_api, dart_string_into_rust,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DartHandle(Rc<Dart_PersistentHandle>);

impl DartHandle {
    /// Wraps the provided [`Dart_Handle`].
    ///
    /// Takes ownership of the provided [`Dart_Handle`] so it won't get freed by
    /// Dart VM.
    ///
    /// # Safety
    ///
    /// The provided [`Dart_Handle`] should be non-`null` and correct.
    ///
    /// # Panics
    ///
    /// If the provided [`Dart_Handle`] represents a Dart error, which is an
    /// unexpected situation.
    #[must_use]
    pub unsafe fn new(handle: Dart_Handle) -> Self {
        if dart_api::is_error(handle) {
            let pointer = dart_api::get_error(handle);
            let err_msg = c_str_into_string(pointer.as_ref().unwrap().into());
            panic!("Unexpected Dart error: {err_msg}")
        }
        Self(Rc::new(dart_api::new_persistent_handle(handle)))
    }

    /// Returns the underlying [`Dart_Handle`].
    #[must_use]
    pub fn get(&self) -> Dart_Handle {
        // SAFETY: We don't expose the inner `Dart_PersistentHandle` anywhere,
        //         so we're sure that it's valid at this point.
        unsafe { dart_api::handle_from_persistent(*self.0) }
    }

    /// Returns string representation of a runtime Dart type behind this
    /// [`DartHandle`].
    #[must_use]
    pub fn name(&self) -> String {
        unsafe { dart_string_into_rust(handle::runtime_type(self.get())) }
    }
}

impl fmt::Display for DartHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string =
            unsafe { dart_string_into_rust(handle::to_string(self.get())) };

        write!(f, "{string}")
    }
}

impl Drop for DartHandle {
    fn drop(&mut self) {
        if let Some(handle) = Rc::get_mut(&mut self.0) {
            unsafe {
                dart_api::delete_persistent_handle(*handle);
            }
        }
    }
}
