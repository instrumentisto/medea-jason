//! Multiplatform Dart runtime specific utility structs and functions.

pub mod callback;
pub mod completer;
pub mod dart_api;
pub mod dart_future;
pub mod function;
pub mod handle;

#[doc(inline)]
pub use self::{completer::Completer, function::Function};
