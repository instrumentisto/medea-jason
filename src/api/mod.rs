//! External [`Jason`] API.

pub mod err;
mod shared;

#[cfg(not(target_family = "wasm"))]
pub mod dart;
#[cfg(not(target_family = "wasm"))]
pub use self::dart::*;

#[cfg(target_family = "wasm")]
mod wasm;
pub use shared::*;

#[cfg(target_family = "wasm")]
pub use self::wasm::*;
