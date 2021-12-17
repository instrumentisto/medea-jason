//! Definitions and implementations of the Rust side representation of the Dart
//! side `List`s.

use std::convert::TryInto;

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
        /// Returns element with a provided index from the provided
        /// [`Dart_Handle`] `List`.
        pub fn get(
            list: Dart_Handle,
            index: i32,
        ) -> ptr::NonNull<DartValueArg<Option<DartHandle>>>;

        /// Returns length of the Dart side `List`.
        pub fn length(list: Dart_Handle) -> i32;
    }
}

/// Rust side representation of the Dart side `List`s.
#[derive(From)]
pub struct DartList(DartHandle);

impl DartList {
    /// Gets [`DartHandle`] from the underlying Dart `List` with a provided
    /// index.
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap
    )]
    #[must_use]
    pub fn get(&self, i: usize) -> Option<DartHandle> {
        unsafe { list::get(self.0.get(), i as i32).unbox() }
            .try_into()
            .unwrap()
    }

    /// Returns length of the underlying Dart `List`.
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap
    )]
    #[must_use]
    pub fn length(&self) -> usize {
        unsafe { list::length(self.0.get()) as usize }
    }
}

impl<T> From<DartList> for Vec<T>
where
    T: From<DartHandle>,
{
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
