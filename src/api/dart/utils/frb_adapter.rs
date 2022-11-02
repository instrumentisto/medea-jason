use crate::platform::utils::dart_api::Dart_NewPersistentHandle_DL_Trampolined;
use dart_sys::Dart_Handle;

#[derive(Debug)]
pub struct MyDartFuture(Dart_Handle);

impl MyDartFuture {
    pub fn new(handle: Dart_Handle) -> Self {
        Self(handle)
    }

    pub fn address(&self) -> usize {
        self.0 as _
    }
}

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
