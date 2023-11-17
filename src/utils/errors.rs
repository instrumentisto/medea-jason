//! Helpers for application errors.

use std::rc::Rc;

use derive_more::{Display, From};

pub use medea_macro::Caused;

/// Representation of an error caused by FFI side.
pub trait Caused {
    /// Type of a wrapper for the FFI error.
    type Error;

    /// Returns the FFI error if represents the cause.
    fn cause(self) -> Option<Self::Error>;
}

/// Wrapper for [`serde_json::Error`] that provides [`Clone`], [`Debug`],
/// [`Display`] implementations.
///
/// [`Debug`]: std::fmt::Debug
/// [`Display`]: std::fmt::Display
#[derive(Clone, Debug, Display, From)]
#[from(forward)]
pub struct JsonParseError(Rc<serde_json::Error>);

impl PartialEq for JsonParseError {
    fn eq(&self, other: &Self) -> bool {
        self.0.line() == other.0.line()
            && self.0.column() == other.0.column()
            && self.0.classify() == other.0.classify()
    }
}
