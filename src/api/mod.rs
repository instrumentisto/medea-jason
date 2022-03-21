//! External [`Jason`] API.

pub mod err;

cfg_if::cfg_if! {
    if #[cfg(not(target_family = "wasm"))] {
        mod dart;
        pub use self::dart::*;
    } else {
        mod wasm;
        pub use self::wasm::*;
    }
}
