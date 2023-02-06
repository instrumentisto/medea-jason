use dart_sys::{Dart_Handle, Dart_NewPersistentHandle_DL};
use flutter_rust_bridge::DartOpaque;

use crate::api::ForeignClass;

/// Creates a new [`DartOpaque`].
#[allow(clippy::expect_used)]
pub unsafe fn new_dart_opaque(handle: Dart_Handle) -> DartOpaque {
    DartOpaque::new_non_droppable(
        Dart_NewPersistentHandle_DL
            .expect("dart_api_dl has not been initialized")(handle)
        .cast(),
    )
}

impl<T> ForeignClass for Vec<T> {}
