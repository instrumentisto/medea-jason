use std::marker::PhantomData;

use dart_sys::Dart_Handle;

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
    handle as _
}
// int to opaque handle

#[no_mangle]
pub unsafe extern "C" fn int2handle(handle: usize) -> Dart_Handle {
    handle as _
}
