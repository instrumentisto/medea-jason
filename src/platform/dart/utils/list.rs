//! Rust side representation of a Dart side [`List`].
//!
//! [`List`]: https://api.dart.dev/stable/dart-core/List-class.html

use derive_more::From;
use medea_macro::dart_bridge;

use crate::platform::dart::utils::{
    handle::DartHandle, NonNullDartValueArgExt,
};

#[dart_bridge("flutter/lib/src/native/ffi/list.g.dart")]
mod list {
    use std::ptr;

    use dart_sys::Dart_Handle;

    use crate::{api::DartValueArg, platform::dart::utils::handle::DartHandle};

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
    }
}

/// Rust side representation of a Dart side [`List`].
///
/// [`List`]: https://api.dart.dev/stable/dart-core/List-class.html
#[derive(From)]
pub struct DartList(DartHandle);

impl DartList {
    /// Returns an element by the provided `index` from this [`DartList`].
    #[must_use]
    pub fn get(&self, index: usize) -> Option<DartHandle> {
        #[allow(clippy::cast_possible_truncation)]
        unsafe { list::get(self.0.get(), index as u32).unbox() }
            .try_into()
            .unwrap()
    }

    /// Returns [`length`] of this [`DartList`].
    ///
    /// [`length`]: https://api.dart.dev/stable/dart-core/List/length.html
    #[must_use]
    pub fn length(&self) -> usize {
        unsafe { list::length(self.0.get()) as usize }
    }
}

impl<T: From<DartHandle>> From<DartList> for Vec<T> {
    fn from(list: DartList) -> Self {
        let len = list.length();
        let mut out = Vec::with_capacity(len);
        for i in 0..len {
            let val = list.get(i).unwrap();
            out.push(val.into());
        }
        out
    }
}
