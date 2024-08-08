use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    ptr,
};

use derive_more::From;
use flutter_rust_bridge::{frb, DartOpaque};

use crate::{
    api::{Error, Error as DartError, ForeignClass},
    platform::utils::dart_future::IntoDartFuture,
    rpc as core,
};

#[derive(Debug, From)]
#[frb(opaque)]
pub struct ReconnectHandle(core::ReconnectHandle);

impl ReconnectHandle {
    /// Tries to reconnect a [`Room`] after the provided delay in milliseconds.
    ///
    /// If the [`Room`] is already reconnecting then new reconnection attempt
    /// won't be performed. Instead, it will wait for the first reconnection
    /// attempt result and use it here.
    ///
    /// [`Room`]: room::Room
    #[frb(sync)]
    #[must_use]
    pub fn reconnect_with_delay(&self, delay_ms: u32) -> DartOpaque {
        let reconnect_handle = self.0.clone();

        async move {
            reconnect_handle.reconnect_with_delay(delay_ms).await?;
            Ok::<_, Error>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }

    /// Tries to reconnect a [`Room`] in a loop with a growing backoff delay.
    ///
    /// The first attempt will be performed immediately, and the second attempt
    /// will be performed after `starting_delay_ms`.
    ///
    /// Delay between reconnection attempts won't be greater than
    /// `max_delay_ms`.
    ///
    /// After each reconnection attempt, delay between reconnections will be
    /// multiplied by the given `multiplier` until it reaches `max_delay_ms`.
    ///
    /// If `multiplier` is a negative number then it will be considered as
    /// `0.0`. This might cause a busy loop, so it's not recommended.
    ///
    /// Max elapsed time can be limited with an optional `max_elapsed_time_ms`
    /// argument.
    ///
    /// If the [`Room`] is already reconnecting then new reconnection attempt
    /// won't be performed. Instead, it will wait for the first reconnection
    /// attempt result and use it here.
    ///
    /// [`Room`]: room::Room
    #[frb(sync)]
    #[must_use]
    pub fn reconnect_with_backoff(
        &self,
        starting_delay: u32,
        multiplier: f64,
        max_delay: u32,
        max_elapsed_time_ms: Option<u32>,
    ) -> DartOpaque {
        let reconnect_handle = self.0.clone();

        async move {
            reconnect_handle
                .reconnect_with_backoff(
                    starting_delay,
                    multiplier,
                    max_delay,
                    max_elapsed_time_ms,
                )
                .await?;
            Ok::<_, DartError>(())
        }
        .into_dart_future()
        .into_dart_opaque()
    }
}

impl ForeignClass for ReconnectHandle {}
impl RefUnwindSafe for ReconnectHandle {}
impl UnwindSafe for ReconnectHandle {}
unsafe impl Send for ReconnectHandle {}
unsafe impl Sync for ReconnectHandle {}

/// Returns the [`ReconnectHandle`] from the [`ForeignClass`] address.
#[frb(sync)]
#[must_use]
pub fn reconnect_handle_from_ptr(ptr: usize) -> ReconnectHandle {
    unsafe { ReconnectHandle::from_ptr(ptr::NonNull::new(ptr as _).unwrap()) }
}
