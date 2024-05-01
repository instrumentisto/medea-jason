//! Rust side representation of a Dart side [`List`].
//!
//! [`List`]: https://api.dart.dev/stable/dart-core/List-class.html

use dart_sys::Dart_Handle;
use derive_more::From;
use medea_macro::dart_bridge;

use crate::{
    api::DartValue,
    platform::dart::utils::{handle::DartHandle, NonNullDartValueArgExt},
};

#[dart_bridge("flutter/lib/src/native/ffi/list.g.dart")]
mod list {
    use std::ptr;

    use dart_sys::Dart_Handle;

    use crate::{
        api::{DartValue, DartValueArg},
        platform::dart::utils::handle::DartHandle,
    };

    extern "C" {
        /// Returns an element by the provided `index` from the provided
        /// [`List`].
        ///
        /// [`List`]: https://api.dart.dev/stable/dart-core/List-class.html
        pub fn get(
            list: Dart_Handle,
            index: u32,
        ) -> ptr::NonNull<DartValueArg<Option<DartHandle>>>;

        /// Returns [`length`] of the provided [`List`].
        ///
        /// [`length`]: https://api.dart.dev/stable/dart-core/List/length.html
        /// [`List`]: https://api.dart.dev/stable/dart-core/List-class.html
        pub fn length(list: Dart_Handle) -> u32;

        pub fn init() -> Dart_Handle;

        pub fn add(map: Dart_Handle, value: DartValue);
    }
}

/// Rust side representation of a Dart side [`List`].
///
/// [`List`]: https://api.dart.dev/stable/dart-core/List-class.html
#[derive(Debug, From)]
pub struct DartList(DartHandle);

impl DartList {
    /// Creates a new empty [`DartList`].
    #[must_use]
    pub fn new() -> Self {
        let map = unsafe { list::init() };
        Self(unsafe { DartHandle::new(map) })
    }

    pub fn add(&mut self, value: DartValue) {
        unsafe {
            list::add(self.0.get(), value);
        }
    }

    /// Returns an element by the provided `index` from this [`DartList`].
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<DartHandle> {
        #[allow(clippy::cast_possible_truncation)]
        let item_ptr = unsafe { list::get(self.0.get(), index as u32) };
        unsafe { item_ptr.unbox() }.try_into().unwrap()
    }

    /// Returns [`length`] of this [`DartList`].
    ///
    /// [`length`]: https://api.dart.dev/stable/dart-core/List/length.html
    #[must_use]
    pub fn length(&self) -> usize {
        unsafe { list::length(self.0.get()) as usize }
    }

    /// Returns the underlying [`Dart_Handle`] of this [`DartList`].
    #[must_use]
    pub fn as_handle(&self) -> Dart_Handle {
        self.0.get()
    }
}

impl<T: From<DartHandle>> From<DartList> for Vec<T> {
    fn from(list: DartList) -> Self {
        let len = list.length();
        let mut out = Self::with_capacity(len);
        for i in 0..len {
            if let Some(v) = list.get(i) {
                out.push(v.into());
            }
        }
        out
    }
}

impl Default for DartList {
    fn default() -> Self {
        Self::new()
    }
}
