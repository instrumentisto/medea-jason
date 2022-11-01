use std::marker::PhantomData;

use dart_sys::Dart_Handle;

use crate::platform::utils::dart_api::Dart_NewPersistentHandle_DL_Trampolined;

pub struct MyDartFuture {
    pub handle: Dart_Handle,
}

impl MyDartFuture {
    fn get(self) -> Dart_Handle {
        self.handle
    }
}


#[no_mangle]
pub unsafe extern "C" fn handle2int(handle: Dart_Handle) -> usize {
    Dart_NewPersistentHandle_DL_Trampolined(handle) as _
}
// int to opaque handle

#[no_mangle]
pub unsafe extern "C" fn int2handle(handle: usize) -> Dart_Handle {
    handle as _
}


// раст вызывает дарт чтобы тот вызвал frb который создаст dart_Handle чтобы вернуть его в дарт а после скастить к Opaque