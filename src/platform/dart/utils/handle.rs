//! Definition and implementation of the wrapper around [`Dart_Handle`] which
//! manages lifetimes of the [`Dart_PersistentHandle`].

use std::rc::Rc;

use dart_sys::{Dart_Handle, Dart_PersistentHandle};

use crate::platform::dart::utils::dart_api::{
    Dart_DeletePersistentHandle_DL_Trampolined,
    Dart_HandleFromPersistent_DL_Trampolined,
    Dart_NewPersistentHandle_DL_Trampolined,
};

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
        Self(Rc::new(Inner(unsafe {
            Dart_NewPersistentHandle_DL_Trampolined(handle)
        })))
    }

    /// Returns the underlying [`Dart_Handle`].
    #[must_use]
    pub fn get(&self) -> Dart_Handle {
        // SAFETY: We don't expose the inner `Dart_PersistentHandle` anywhere,
        //         so we're sure that it's valid at this point.
        unsafe { Dart_HandleFromPersistent_DL_Trampolined(self.0 .0) }
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
