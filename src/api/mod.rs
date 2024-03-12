//! External [`Jason`] API.

pub mod err;

#[cfg(not(target_family = "wasm"))]
pub mod dart;
#[cfg(not(target_family = "wasm"))]
pub use self::dart::*;

#[cfg(target_family = "wasm")]
mod wasm;
#[cfg(target_family = "wasm")]
pub use self::wasm::*;
