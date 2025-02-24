//! Miscellaneous utility structs and functions.

#[macro_use]
mod errors;

pub mod component;
mod resettable_delay;

use derive_more::with_trait::From;
use futures::future::{self, AbortHandle};
use medea_reactive::Guarded;

#[doc(inline)]
pub use self::{
    component::{AsProtoState, Component, SynchronizableState, Updatable},
    errors::{Caused, JsonParseError},
    resettable_delay::{ResettableDelayHandle, resettable_delay_for},
};

/// Wrapper around [`AbortHandle`] which aborts [`Future`] on [`Drop`].
#[derive(Debug, From)]
pub struct TaskHandle(AbortHandle);

impl Drop for TaskHandle {
    fn drop(&mut self) {
        self.0.abort();
    }
}

/// Tries to upgrade [`Weak`] reference breaks cycle if upgrade fails.
macro_rules! upgrade_or_break {
    ($weak:tt) => {
        if let Some(this) = $weak.upgrade() {
            this
        } else {
            break;
        }
    };
}

/// Returns [`Future`] which will return the provided value being
/// [`Guarded::transpose()`]d.
///
/// Intended for use in [`StreamExt::filter_map()`].
///
/// [`StreamExt::filter_map()`]: futures::StreamExt::filter_map
pub fn transpose_guarded<T>(
    val: Guarded<Option<T>>,
) -> impl Future<Output = Option<Guarded<T>>> {
    future::ready(val.transpose())
}
