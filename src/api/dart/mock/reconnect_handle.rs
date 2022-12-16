#![allow(
    clippy::missing_errors_doc,
    clippy::unused_async,
    missing_copy_implementations
)]

use dart_sys::Dart_Handle;
use futures::future;
use tracerr::{Trace, Traced};

use crate::{
    api::err::{RpcClientException, RpcClientExceptionKind},
    platform::{
        self,
        utils::{
            dart_future::{DartFuture, IntoDartFuture},
            result::DartResult,
        },
    },
    rpc::{ReconnectError, ReconnectHandle as CoreReconnectHandle},
};

use crate::api::dart::utils::DartError;

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

#[no_mangle]
pub unsafe extern "C" fn returns_rpc_client_exception(
    cause: Dart_Handle,
) -> DartResult {
    let err = RpcClientException::new(
        RpcClientExceptionKind::ConnectionLost,
        "RpcClientException::ConnectionLost",
        Some(platform::Error::from_handle(cause)),
        Trace::new(vec![tracerr::new_frame!()]),
    );

    DartError::from(err).into()
}

#[no_mangle]
pub unsafe extern "C" fn returns_future_rpc_client_exception(
    cause: Dart_Handle,
) -> DartFuture<Result<(), DartError>> {
    let err = RpcClientException::new(
        RpcClientExceptionKind::SessionFinished,
        "RpcClientException::SessionFinished",
        Some(platform::Error::from_handle(cause)),
        Trace::new(vec![tracerr::new_frame!()]),
    );

    future::err(err.into()).into_dart_future()
}
