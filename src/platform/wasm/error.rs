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
    /// Returns a brief description of the error if it's available or has been
    /// set.
    #[must_use]
    pub fn message(&self) -> String {
        self.0.message().into()
    }
}

impl From<JsValue> for Error {
    fn from(val: JsValue) -> Self {
        match val.dyn_into::<js_sys::Error>() {
            Ok(err) => Self(err),
            Err(val) => match val.as_string() {
                Some(msg) => Self(js_sys::Error::new(&msg)),
                None => Self(js_sys::Error::new(&format!("{val:?}"))),
            },
        }
    }
}

/// This implementation allows us to use [`Error`] as a return type in functions
/// exported to JS.
impl WasmDescribe for Error {
    fn describe() {
        js_sys::Error::describe();
    }
}

/// This implementation allows us to use [`Error`] as a return type in functions
/// exported to JS.
impl IntoWasmAbi for Error {
    type Abi = <js_sys::Error as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        self.0.into_abi()
    }
}

/// This implementation allows us to use [`Error`] as a return type in functions
/// exported to JS.
impl OptionIntoWasmAbi for Error {
    fn none() -> u32 {
        js_sys::Error::none()
    }
}
