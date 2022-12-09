use dart_sys::Dart_Handle;
use derive_more::Deref;
use flutter_rust_bridge::DartOpaque;
use std::{
    cell::RefCell,
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::platform::utils::dart_api::Dart_NewPersistentHandle_DL_Trampolined;

pub unsafe fn new_dart_opaque(handle: Dart_Handle) -> DartOpaque {
    DartOpaque::new_non_dropable(Dart_NewPersistentHandle_DL_Trampolined(
        handle,
    ))
}

#[derive(Deref, Debug)]
pub struct ApiWrap<T>(RefCell<T>);

impl<T> ApiWrap<T> {
    pub fn new(data: T) -> Self {
        data.into()
    }

    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }
}
impl<T> From<T> for ApiWrap<T> {
    fn from(data: T) -> Self {
        Self(RefCell::new(data))
    }
}

impl<T> UnwindSafe for ApiWrap<T> {}
impl<T> RefUnwindSafe for ApiWrap<T> {}
