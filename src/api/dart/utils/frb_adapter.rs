use dart_sys::Dart_Handle;
use derive_more::Deref;
use flutter_rust_bridge::DartOpaque;
use std::{
    cell::RefCell,
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    api::ForeignClass,
    platform::utils::dart_api::Dart_NewPersistentHandle_DL_Trampolined,
};

/// Creates a new [`DartOpaque`].
pub unsafe fn new_dart_opaque(handle: Dart_Handle) -> DartOpaque {
    DartOpaque::new_non_droppable(Dart_NewPersistentHandle_DL_Trampolined(
        handle,
    ))
}

#[derive(Deref, Debug)]
/// Panic-unsafe wrapper [`RefCell`].
/// Used in `flutter_rust_bridge` API.
pub struct ApiWrap<T>(RefCell<T>);

impl<T> ApiWrap<T> {
    /// Creates a new [`ApiWrap`].
    pub const unsafe fn new(data: T) -> Self {
        Self(RefCell::new(data))
    }

    /// Returns inner data.
    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }
}

impl<T> UnwindSafe for ApiWrap<T> {}
impl<T> RefUnwindSafe for ApiWrap<T> {}
impl<T> ForeignClass for ApiWrap<T> {}
