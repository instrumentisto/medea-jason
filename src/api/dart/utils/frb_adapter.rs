use dart_sys::Dart_Handle;
use flutter_rust_bridge::DartOpaque;

use crate::{api::ForeignClass, platform::utils::dart_api};

/// Creates a new [`DartOpaque`].
pub unsafe fn new_dart_opaque(handle: Dart_Handle) -> DartOpaque {
    DartOpaque::new_non_droppable(
        dart_api::new_persistent_handle(handle).cast(),
    )
}

impl<T> ForeignClass for Vec<T> {}
