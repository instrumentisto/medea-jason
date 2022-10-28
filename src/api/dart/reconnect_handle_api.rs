use flutter_rust_bridge::{Opaque, SyncReturn};

use crate::api::dart::utils::{ArgumentError, IntoDartFuture};

use super::{media_manager_handle_api::MyDartFuture, utils::DartError};

#[cfg(feature = "mockable")]
pub use self::mock::ReconnectHandle;
#[cfg(not(feature = "mockable"))]
pub use crate::rpc::ReconnectHandle;

/// Tries to reconnect a [`Room`] after the provided delay in milliseconds.
///
/// If the [`Room`] is already reconnecting then new reconnection attempt won't
/// be performed. Instead, it will wait for the first reconnection attempt
/// result and use it here..
///
/// [`Room`]: crate::room::Room
pub fn reconnect_handle_reconnect_with_delay(
    reconnect_handle: Opaque<ReconnectHandle>,
    delay_ms: i64,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let reconnect_handle = ReconnectHandle::clone(&reconnect_handle);
    SyncReturn(Opaque::new(
        async move {
            #[allow(clippy::map_err_ignore)]
            let delay_ms = u32::try_from(delay_ms).map_err(|_| {
                ArgumentError::new(delay_ms, "delayMs", "Expected u32")
            })?;

            reconnect_handle.reconnect_with_delay(delay_ms).await?;
            Ok::<_, DartError>(())
        }
        .into_my_dart_future(),
    ))
}

/// Tries to reconnect a [`Room`] in a loop with a growing backoff delay.
///
/// The first attempt will be performed immediately, and the second attempt will
/// be performed after `starting_delay_ms`.
///
/// Delay between reconnection attempts won't be greater than
/// `max_delay_ms`.
///
/// After each reconnection attempt, delay between reconnections will be
/// multiplied by the given `multiplier` until it reaches `max_delay_ms`.
///
/// If `multiplier` is a negative number then it will be considered as `0.0`.
/// reconnect_handle might cause a busy loop, so it's not recommended.
///
/// Max elapsed time can be limited with an optional `max_elapsed_time_ms`
/// argument.
///
/// If the [`Room`] is already reconnecting then new reconnection attempt won't
/// be performed. Instead, it will wait for the first reconnection attempt
/// result and use it here.
///
/// [`Room`]: crate::room::Room
pub fn reconnect_handle_reconnect_with_backoff(
    reconnect_handle: Opaque<ReconnectHandle>,
    starting_delay: i64,
    multiplier: f64,
    max_delay: u32,
    max_elapsed_time_ms: Option<u32>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    let reconnect_handle = ReconnectHandle::clone(&reconnect_handle);

    SyncReturn(Opaque::new(
        async move {
            #[allow(clippy::map_err_ignore)]
            let starting_delay =
                u32::try_from(starting_delay).map_err(|_| {
                    ArgumentError::new(
                        starting_delay,
                        "startingDelayMs",
                        "Expected u32",
                    )
                })?;
            #[allow(clippy::map_err_ignore)]
            let max_elapsed_time_ms = max_elapsed_time_ms
                .map(|v| {
                    #[allow(clippy::map_err_ignore)]
                    u32::try_from(v).map_err(|_| {
                        ArgumentError::new(
                            v,
                            "maxElapsedTimeMs",
                            "Expected u32",
                        )
                    })
                })
                .transpose()?;

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
        .into_my_dart_future(),
    ))
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(
        clippy::missing_errors_doc,
        clippy::unused_async,
        missing_copy_implementations
    )]

    use dart_sys::Dart_Handle;
    use futures::future;
    use tracerr::{Trace, Traced};

    use crate::{
        api::{
            dart::utils::{
                DartError, DartFuture, DartResult, IntoDartFuture as _,
            },
            err::{RpcClientException, RpcClientExceptionKind},
        },
        platform,
        rpc::{ReconnectError, ReconnectHandle as CoreReconnectHandle},
    };

    #[derive(Clone, Debug)]
    pub struct ReconnectHandle(pub u8);

    impl From<CoreReconnectHandle> for ReconnectHandle {
        fn from(_: CoreReconnectHandle) -> Self {
            Self(0)
        }
    }

    impl ReconnectHandle {
        pub async fn reconnect_with_delay(
            &self,
            _delay_ms: u32,
        ) -> Result<(), Traced<ReconnectError>> {
            Ok(())
        }

        pub async fn reconnect_with_backoff(
            &self,
            _starting_delay_ms: u32,
            _multiplier: f64,
            _max_delay: u32,
            _max_elapsed_time_ms: Option<u32>,
        ) -> Result<(), Traced<ReconnectError>> {
            Ok(())
        }
    }

    pub fn returns_rpc_client_exception(cause: Dart_Handle) -> DartResult {
        let err = RpcClientException::new(
            RpcClientExceptionKind::ConnectionLost,
            "RpcClientException::ConnectionLost",
            Some(platform::Error::from_handle(cause)),
            Trace::new(vec![tracerr::new_frame!()]),
        );

        DartError::from(err).into()
    }

    pub fn returns_future_rpc_client_exception(
        cause: Dart_Handle,
    ) -> SyncReturn<Opaque<MyDartFuture>> {
        let err = RpcClientException::new(
            RpcClientExceptionKind::SessionFinished,
            "RpcClientException::SessionFinished",
            Some(platform::Error::from_handle(cause)),
            Trace::new(vec![tracerr::new_frame!()]),
        );

        future::err(err.into()).into_my_dart_future()
    }
}
