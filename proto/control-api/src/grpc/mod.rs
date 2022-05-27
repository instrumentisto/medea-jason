//! [gRPC]-based [Control API] for [Medea].
//!
//! [gRPC]: https://grpc.io
//! [Medea]: https://github.com/instrumentisto/medea
//! [Control API]: https://tinyurl.com/yxsqplq7

mod conversions;

#[allow(
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#[rustfmt::skip]
pub mod api;
#[allow(
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#[rustfmt::skip]
pub mod callback;

use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::Mutex;
use tonic::{transport::Channel, Response, Status};

use crate::{
    callback::Request as CallbackRequest,
    control,
    grpc::{
        api::{
            self as proto, control_api_server::ControlApi as GrpcControlApi,
        },
        callback::callback_client,
    },
    ControlApi, Ping,
};

pub use self::conversions::{
    CallbackUrl, CallbackUrlParseError, TryFromProtobufError,
};

/// gRPC [`CallbackClient`] for sending [`Request`]s.
///
/// [`CallbackClient`]: crate::CallbackClient
/// [`Request`]: CallbackRequest
#[derive(Debug)]
pub struct CallbackClient {
    client: Mutex<callback_client::CallbackClient<Channel>>,
}

impl CallbackClient {
    /// Returns gRPC [`CallbackClient`]s.
    ///
    /// For every [`CallbackUrl`] creates a unique connection and serves all
    /// [`Event`]s through it.
    ///
    /// [`CallbackClient`]: crate::CallbackClient
    /// [`Event`]: crate::callback::Event
    pub async fn connect(
        url: CallbackUrl,
    ) -> Result<Self, tonic::transport::Error> {
        let client =
            callback_client::CallbackClient::connect(url.http_addr()).await?;
        Ok(Self {
            client: Mutex::new(client),
        })
    }
}

#[async_trait(?Send)]
impl crate::CallbackApi for CallbackClient {
    type Error = Status;

    async fn fire_event(
        &self,
        request: CallbackRequest,
    ) -> Result<(), Self::Error> {
        let mut guard = self.client.lock().await;
        let _ = guard.on_event(tonic::Request::new(request.into())).await?;
        Ok(())

        // let url = CallbackUrl::try_from(request.url.clone())?;
        // let read_guard = self.clients.read().await;
        //
        // let mut client = if let Some(client) = read_guard.get(&url) {
        //     client.clone()
        // } else {
        //     drop(read_guard);
        //     let mut write_guard = self.clients.write().await;
        //     if let Some(client) = write_guard.get(&url) {
        //         client.clone()
        //     } else {
        //         let client =
        //             callback_client::CallbackClient::connect(url.http_addr())
        //                 .await
        //                 .map_err(|e| ErrorResponse::unexpected(&e))?;
        //         drop(write_guard.insert(url, client.clone()));
        //         client
        //     }
        // };
        //
        // drop(
        //     client
        //         .on_event(tonic::Request::new(request.into()))
        //         .await
        //         .map_err(|e| ErrorResponse::unexpected(&e))?,
        // );
        // Ok(())
    }
}

#[async_trait]
impl<T> GrpcControlApi for T
where
    T: ControlApi + Send + Sync + 'static,
    T::Error: Into<proto::Error> + From<TryFromProtobufError>,
{
    async fn create(
        &self,
        req: tonic::Request<proto::CreateRequest>,
    ) -> Result<Response<proto::CreateResponse>, Status> {
        let fut = async {
            let req = control::Request::try_from(req.into_inner())?;
            self.create(req).await
        };

        Ok(Response::new(match fut.await {
            Ok(sids) => proto::CreateResponse {
                sid: sids
                    .into_iter()
                    .map(|(id, sid)| (id.to_string(), sid.to_string()))
                    .collect(),
                error: None,
            },
            Err(e) => proto::CreateResponse {
                sid: HashMap::new(),
                error: Some(e.into()),
            },
        }))
    }

    async fn delete(
        &self,
        req: tonic::Request<proto::IdRequest>,
    ) -> Result<Response<proto::Response>, Status> {
        let ids = req
            .into_inner()
            .fid
            .into_iter()
            .map(|fid| fid.parse().map_err(TryFromProtobufError::from))
            .collect::<Result<Vec<_>, _>>();

        let result = match ids {
            Ok(ids) => self.delete_elements(ids).await,
            Err(e) => Err(e.into()),
        };

        Ok(Response::new(match result {
            Ok(_) => proto::Response { error: None },
            Err(e) => proto::Response {
                error: Some(e.into()),
            },
        }))
    }

    async fn get(
        &self,
        req: tonic::Request<proto::IdRequest>,
    ) -> Result<Response<proto::GetResponse>, Status> {
        let ids = req
            .into_inner()
            .fid
            .into_iter()
            .map(|fid| fid.parse().map_err(TryFromProtobufError::from))
            .collect::<Result<Vec<_>, _>>();

        let result = match ids {
            Ok(ids) => self.get_elements(ids).await,
            Err(e) => Err(e.into()),
        };

        Ok(Response::new(match result {
            Ok(elements) => proto::GetResponse {
                elements: elements
                    .into_iter()
                    .map(|(id, el)| (id.to_string(), el.into()))
                    .collect(),
                error: None,
            },
            Err(e) => proto::GetResponse {
                elements: HashMap::new(),
                error: Some(e.into()),
            },
        }))
    }

    async fn apply(
        &self,
        req: tonic::Request<proto::ApplyRequest>,
    ) -> Result<Response<proto::CreateResponse>, Status> {
        let result = async {
            let req = control::Request::try_from(req.into_inner())?;
            self.apply(req).await
        };

        Ok(Response::new(match result.await {
            Ok(sids) => proto::CreateResponse {
                sid: sids
                    .into_iter()
                    .map(|(id, sid)| (id.to_string(), sid.to_string()))
                    .collect(),
                error: None,
            },
            Err(e) => proto::CreateResponse {
                sid: HashMap::new(),
                error: Some(e.into()),
            },
        }))
    }

    async fn healthz(
        &self,
        request: tonic::Request<proto::Ping>,
    ) -> Result<Response<proto::Pong>, Status> {
        self.healthz(Ping(request.into_inner().nonce))
            .await
            .map(|pong| Response::new(proto::Pong { nonce: pong.0 }))
            .map_err(|e| {
                let e = e.into();
                let message = [&e.doc, &e.element, &e.text].into_iter().fold(
                    e.code.to_string(),
                    |mut acc, s| {
                        if !s.is_empty() {
                            acc.push_str(": ");
                            acc.push_str(s)
                        }
                        acc
                    },
                );

                Status::unknown(message)
            })
    }
}
