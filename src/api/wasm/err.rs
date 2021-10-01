//! Implementations and definitions of errors which can be returned from API
//! functions.

use derive_more::{From, Into};
use wasm_bindgen::{
    convert::{FromWasmAbi, IntoWasmAbi},
    describe::WasmDescribe,
    prelude::*,
};

use crate::api::err::{
    EnumerateDevicesException, FormatException, InternalException,
    LocalMediaInitException, MediaSettingsUpdateException,
    MediaStateTransitionException, RpcClientException, StateError,
};

/// Wrapper around [`JsValue`] which represents a JS error.
#[derive(Into, From)]
pub struct Error(JsValue);

/// This implementation allows us to use [`Error`] as a return type in functions
/// exported to JS.
impl WasmDescribe for Error {
    fn describe() {
        JsValue::describe();
    }
}

/// This implementation allows us to use [`Error`] as a return type in functions
/// exported to JS.
impl IntoWasmAbi for Error {
    type Abi = <JsValue as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        self.0.into_abi()
    }
}

impl FromWasmAbi for Error {
    type Abi = <JsValue as FromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        Self(FromWasmAbi::from_abi(js))
    }
}

/// Implements `From<T> for Error where T: Into<JsValue>` for specified `T`.
macro_rules! impl_from_into_jsval_for_error {
    ($arg:ty) => {
        impl From<$arg> for Error {
            #[inline]
            fn from(err: $arg) -> Self {
                Error(err.into())
            }
        }
    };
}

impl_from_into_jsval_for_error!(StateError);
impl_from_into_jsval_for_error!(EnumerateDevicesException);
impl_from_into_jsval_for_error!(LocalMediaInitException);
impl_from_into_jsval_for_error!(RpcClientException);
impl_from_into_jsval_for_error!(InternalException);
impl_from_into_jsval_for_error!(FormatException);
impl_from_into_jsval_for_error!(MediaStateTransitionException);
impl_from_into_jsval_for_error!(MediaSettingsUpdateException);
