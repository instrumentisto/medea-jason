//! More convenient wrapper for [`js_sys::Error`].

use derive_more::Display;
use wasm_bindgen::{
    convert::{IntoWasmAbi, OptionIntoWasmAbi},
    describe::WasmDescribe,
    JsCast, JsValue,
};

/// Wrapper for JS value which returned from JS side as error.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "{}", "String::from(_0.to_string())")]
pub struct Error(js_sys::Error);

impl Error {
    /// Returns a brief description of the error if one is available or has been
    /// set.
    #[inline]
    #[must_use]
    pub fn message(&self) -> String {
        self.0.message().into()
    }
}

impl From<JsValue> for Error {
    #[inline]
    fn from(val: JsValue) -> Self {
        match val.dyn_into::<js_sys::Error>() {
            Ok(err) => Error(err),
            Err(val) => match val.as_string() {
                Some(msg) => Error(js_sys::Error::new(&msg)),
                None => Error(js_sys::Error::new(&format!("{:?}", val))),
            },
        }
    }
}

/// This implementation allows us to use [`Error`] as return type in functions exported to JS.
impl WasmDescribe for Error {
    fn describe() {
        js_sys::Error::describe();
    }
}

/// This implementation allows us to use [`Error`] as return type in functions exported to JS.
impl IntoWasmAbi for Error {
    type Abi = <js_sys::Error as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.0.into_abi()
    }
}

/// This implementation allows us to use [`Error`] as return type in functions exported to JS.
impl OptionIntoWasmAbi for Error {
    #[inline]
    fn none() -> u32 {
        js_sys::Error::none()
    }
}
