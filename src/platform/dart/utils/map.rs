//! Rust side representation of a Dart side [`Map`].
//!
//! [`Map`]: https://api.dart.dev/stable/dart-core/Map-class.html

use std::ffi::CString;

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
        /// Initializes a new empty [`Map`].
        ///
        /// [`Map`]: https://api.dart.dev/stable/dart-core/Map-class.html
        pub fn init() -> Dart_Handle;

        /// Sets the provided `value` under the provided `key` to the provided
        /// [`Map`].
        ///
        /// [`Map`]: https://api.dart.dev/stable/dart-core/Map-class.html
        pub fn set(
            map: Dart_Handle,
            key: ptr::NonNull<c_char>,
            value: DartValue,
        );
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
        Self(unsafe { DartHandle::new(map::init()) })
    }

    /// Sets the provided `value` under the provided `key` to this [`DartMap`].
    pub fn set(&mut self, key: String, value: DartValue) {
        let key_c_str = string_into_c_str(key);
        unsafe {
            map::set(self.0.get(), key_c_str, value);
            drop(CString::from_raw(key_c_str.as_ptr()));
        }
    }

    /// Returns the underlying [`Dart_Handle`] of this [`DartMap`].
    #[must_use]
    pub fn as_handle(&self) -> Dart_Handle {
        self.0.get()
    }
}
