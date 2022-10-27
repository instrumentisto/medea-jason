//! External [`Jason`] API.
pub mod err;

cfg_if::cfg_if! {
    if #[cfg(target_family = "wasm")] {
        mod wasm;
        pub use self::wasm::*;
    } else {
        pub mod dart;
        pub use self::dart::*;
    }
}
