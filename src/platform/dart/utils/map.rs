//! Definitions and implementation of the Rust side representation of the Dart
//! side Map.

use dart_sys::Dart_Handle;
use medea_macro::dart_bridge;

use crate::{
    api::{string_into_c_str, DartValue},
    platform::dart::utils::handle::DartHandle,
};

#[dart_bridge("flutter/lib/src/native/ffi/map.g.dart")]
mod map {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::api::DartValue;

    extern "C" {
        /// Returns [`Dart_Handle`] to the newly created Dart `Map`.
        pub fn init() -> Dart_Handle;

        /// Sets provided [`Dart_Handle`] with a provided [`c_char`] key to the
        /// provided [`Dart_Handle`] `Map`.
        pub fn set(
            map: Dart_Handle,
            key: ptr::NonNull<c_char>,
            value: DartValue,
        );
    }
}

/// Rust representation of the Dart side Map.
pub struct DartMap(DartHandle);

impl From<DartMap> for Dart_Handle {
    fn from(from: DartMap) -> Self {
        from.0.get()
    }
}

impl Default for DartMap {
    fn default() -> Self {
        Self::new()
    }
}

impl DartMap {
    /// Returns new Dart `Map`.
    #[must_use]
    pub fn new() -> Self {
        Self(DartHandle::new(unsafe { map::init() }))
    }

    /// Sets provided [`Value`] to the provided `key`.
    pub fn set(&self, key: String, value: DartValue) {
        unsafe {
            map::set(self.0.get(), string_into_c_str(key), value);
        }
    }

    /// Returns underlying [`Dart_Handle`] of this [`DartMap`].
    #[must_use]
    pub fn as_handle(&self) -> Dart_Handle {
        self.0.get()
    }
}
