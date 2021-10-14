use std::any::Any;

use dart_sys::Dart_Handle;

use crate::platform::dart::error::Error;

// TODO(evdokimovs): Remove err_message and err_name fields.
#[repr(C)]
pub struct DartResult {
    pub is_ok: i8,
    pub ok: *const dyn Any,
    pub cause: Dart_Handle,
}

impl<T: 'static> From<DartResult> for Result<&T, Error> {
    fn from(from: DartResult) -> Self {
        if from.is_ok == 1 {
            Ok(unsafe { from.ok.as_ref().unwrap().downcast_ref().unwrap() })
        } else {
            Err(Error::from(from.cause))
        }
    }
}

#[repr(C)]
pub struct VoidDartResult {
    pub is_ok: i8,
    pub err_name: *const libc::c_char,
    pub err_message: *const libc::c_char,
    pub cause: Dart_Handle,
}

impl From<VoidDartResult> for Result<(), Error> {
    fn from(from: VoidDartResult) -> Self {
        if from.is_ok == 1 {
            Ok(())
        } else {
            Err(Error::from(from.cause))
        }
    }
}
