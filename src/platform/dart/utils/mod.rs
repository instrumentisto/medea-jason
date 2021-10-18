//! Multiplatform Dart runtime specific utility structs and functions.

pub mod array;
pub mod callback_listener;
pub mod completer;
pub mod dart_api;
pub mod dart_future;
pub mod function;
pub mod handle;
pub mod map;
pub mod nullable;

#[doc(inline)]
pub use self::{completer::Completer, function::Function};
