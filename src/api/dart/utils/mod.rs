//! Miscellaneous utility structs and functions.

mod err;

use dart_sys::Dart_Handle;
use flutter_rust_bridge::DartOpaque;

use crate::platform::utils::dart_api;

pub use self::err::{new_panic_error, DartError};

/// Creates a new [`DartOpaque`] value out of the provided [`Dart_Handle`].
pub unsafe fn new_dart_opaque(handle: Dart_Handle) -> DartOpaque {
    // let h = unsafe { dart_api::new_persistent_handle(handle) };
    // // DartOpaque::new()
    // unsafe { DartOpaque::new_non_droppable(h.cast()) }
    todo!()
}
