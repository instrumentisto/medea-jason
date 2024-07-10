//! Rust side representation of a Dart side [`Map`].
//!
//! [`Map`]: https://api.dart.dev/stable/dart-core/Map-class.html

use dart_sys::Dart_Handle;
use medea_macro::dart_bridge;

use crate::{
    api::DartValue,
    platform::dart::utils::{handle::DartHandle, string_into_c_str},
};

#[dart_bridge("flutter/lib/src/native/ffi/map.g.dart")]
mod map {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::{api::DartValue, platform::Error};

    extern "C" {
        /// Initializes a new empty [`Map`].
        ///
        /// [`Map`]: https://api.dart.dev/stable/dart-core/Map-class.html
        pub fn init() -> Result<Dart_Handle, Error>;

        /// Sets the provided `value` under the provided `key` to the provided
        /// [`Map`].
        ///
        /// [`Map`]: https://api.dart.dev/stable/dart-core/Map-class.html
        pub fn set(
            map: Dart_Handle,
            key: ptr::NonNull<c_char>,
            value: DartValue,
        ) -> Result<(), Error>;
    }
}

/// Rust representation of a Dart side [`Map`].
///
/// [`Map`]: https://api.dart.dev/stable/dart-core/Map-class.html
#[derive(Debug)]
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
    /// Creates a new empty [`DartMap`].
    #[must_use]
    pub fn new() -> Self {
        let map = unsafe { map::init() }.unwrap();
        Self(unsafe { DartHandle::new(map) })
    }

    /// Sets the provided `value` under the provided `key` to this [`DartMap`].
    pub fn set(&mut self, key: String, value: DartValue) {
        unsafe { map::set(self.0.get(), string_into_c_str(key), value) }
            .unwrap();
    }

    /// Returns the underlying [`Dart_Handle`] of this [`DartMap`].
    #[must_use]
    pub fn as_handle(&self) -> Dart_Handle {
        self.0.get()
    }
}
