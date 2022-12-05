use std::{
    cell::RefCell,
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::platform::utils::dart_api::Dart_NewPersistentHandle_DL_Trampolined;
use dart_sys::Dart_Handle;
use derive_more::Deref;

#[derive(Debug)]
pub struct MyDartFuture(Dart_Handle);

impl MyDartFuture {
    pub fn new(handle: Dart_Handle) -> Self {
        Self(handle)
    }

    pub fn address(&self) -> usize {
        self.0 as _
    }

    pub fn into_raw(self) -> Dart_Handle {
        self.0
    }
}

#[derive(Deref, Debug)]
pub struct ApiWrapVec<T>(RefCell<Vec<T>>);
impl<T> ApiWrapVec<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self(RefCell::new(vec))
    }
}

impl<T> UnwindSafe for ApiWrapVec<T> {}
impl<T> RefUnwindSafe for ApiWrapVec<T> {}

#[no_mangle]
pub unsafe extern "C" fn handle_to_persistent_address(
    handle: Dart_Handle,
) -> usize {
    Dart_NewPersistentHandle_DL_Trampolined(handle) as _
}

#[no_mangle]
pub unsafe extern "C" fn handle_to_address(handle: Dart_Handle) -> usize {
    handle as _
}

#[no_mangle]
pub unsafe extern "C" fn address_to_handle(handle: usize) -> Dart_Handle {
    handle as _
}
