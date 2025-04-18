//! [`ControlApi`] server and [`CallbackApi`] client [gRPC] implementations.
//!
//! [gRPC]: https://grpc.io

use std::collections::HashMap;

use async_trait::async_trait;
use derive_more::with_trait::{Display, Error, From};
use tonic::codegen::{Body, Bytes};

use crate::{
    CallbackApi, ControlApi,
    callback::Request as CallbackRequest,
    control::Request as ControlRequest,
    grpc::{
        CallbackApiClient, ProtobufError,
        api::{
            self as control_proto,
            control_api_server::ControlApi as GrpcControlApiService,
        },
        callback as callback_proto,
    },
};

/// [`Box`]ed [`Error`] with [`Send`] and [`Sync`].
type StdError = Box<dyn Error + Send + Sync + 'static>;

#[async_trait]
impl<T: ?Sized> GrpcControlApiService for T
where
    T: ControlApi + Send + Sync + 'static,
    T::Error: From<ProtobufError>,
    control_proto::Error: From<T::Error>,
{
    async fn create(
        &self,
        request: tonic::Request<control_proto::CreateRequest>,
    ) -> Result<tonic::Response<control_proto::CreateResponse>, tonic::Status>
    {
        let fut = async {
            self.create(ControlRequest::try_from(request.into_inner())?).await
        };

        Ok(tonic::Response::new(match fut.await {
            Ok(sids) => control_proto::CreateResponse {
                sid: sids
                    .into_iter()
                    .map(|(id, sid)| (id.to_string(), sid.to_uri_string()))
                    .collect(),
                error: None,
            },
            Err(e) => control_proto::CreateResponse {
                sid: HashMap::new(),
                error: Some(e.into()),
            },
        }))
    }

    async fn delete(
        &self,
        request: tonic::Request<control_proto::IdRequest>,
    ) -> Result<tonic::Response<control_proto::Response>, tonic::Status> {
        let ids = request
            .into_inner()
            .fid
            .into_iter()
            .map(|fid| fid.parse().map_err(ProtobufError::from))
            .collect::<Result<Vec<_>, _>>();

        let result = match ids {
            Ok(ids) => self.delete(&ids).await,
            Err(e) => Err(e.into()),
        };

        Ok(tonic::Response::new(match result {
            Ok(()) => control_proto::Response { error: None },
            Err(e) => control_proto::Response { error: Some(e.into()) },
        }))
    }

    async fn get(
        &self,
        request: tonic::Request<control_proto::IdRequest>,
    ) -> Result<tonic::Response<control_proto::GetResponse>, tonic::Status>
    {
        let ids = request
            .into_inner()
            .fid
            .into_iter()
            .map(|fid| fid.parse().map_err(ProtobufError::from))
            .collect::<Result<Vec<_>, _>>();

        let result = match ids {
            Ok(ids) => self.get(&ids).await,
            Err(e) => Err(e.into()),
        };

        Ok(tonic::Response::new(match result {
            Ok(elements) => control_proto::GetResponse {
                elements: elements
                    .into_iter()
                    .map(|(id, el)| {
                        let s = id.to_string();
                        (id, el).try_into().map(|proto| (s, proto))
                    })
                    .collect::<Result<_, _>>()?,
                error: None,
            },
            Err(e) => control_proto::GetResponse {
                elements: HashMap::new(),
                error: Some(e.into()),
            },
        }))
    }

    async fn apply(
        &self,
        request: tonic::Request<control_proto::ApplyRequest>,
    ) -> Result<tonic::Response<control_proto::CreateResponse>, tonic::Status>
    {
        let result = async {
            let req = ControlRequest::try_from(request.into_inner())?;
            self.apply(req).await
        };

        Ok(tonic::Response::new(match result.await {
            Ok(sids) => control_proto::CreateResponse {
                sid: sids
                    .into_iter()
                    .map(|(id, sid)| (id.to_string(), sid.to_uri_string()))
                    .collect(),
                error: None,
            },
            Err(e) => control_proto::CreateResponse {
                sid: HashMap::new(),
                error: Some(e.into()),
            },
        }))
    }

    async fn healthz(
        &self,
        request: tonic::Request<control_proto::Ping>,
    ) -> Result<tonic::Response<control_proto::Pong>, tonic::Status> {
        self.healthz(request.into_inner().into())
            .await
            .map(|pong| tonic::Response::new(pong.into()))
            .map_err(|e| {
                let e = control_proto::Error::from(e);
                let message = [&e.doc, &e.element, &e.text].into_iter().fold(
                    e.code.to_string(),
                    |mut acc, s| {
                        if !s.is_empty() {
                            acc.push_str(": ");
                            acc.push_str(s);
                        }
                        acc
                    },
                );
                tonic::Status::unknown(message)
            })
    }
}

#[async_trait]
impl<T> CallbackApi for CallbackApiClient<T>
where
    T: tonic::client::GrpcService<
            tonic::body::Body,
            Future: Send,
            ResponseBody: Body<Data = Bytes, Error: Into<StdError> + Send>
                              + Send
                              + 'static,
        > + Clone
        + Send
        + Sync,
{
    type Error = CallbackApiClientError;

    async fn on_event(
        &self,
        request: CallbackRequest,
    ) -> Result<(), Self::Error> {
        // It's OK to `.clone()` `tonic::client`:
        // https://docs.rs/tonic/latest/tonic/client/index.html#concurrent-usage
        let mut this = self.clone();

        Self::on_event(&mut this, callback_proto::Request::from(request))
            .await
            .map(drop)
            .map_err(Into::into)
    }
}

/// Possible errors of [`CallbackApiClient`].
#[derive(Debug, Display, From, Error)]
pub enum CallbackApiClientError {
    /// [gRPC] server errored.
    ///
    /// [gRPC]: https://grpc.io
    #[display("gRPC server errored: {_0}")]
    Tonic(tonic::Status),

    /// Failed to convert from [gRPC] response.
    ///
    /// [gRPC]: https://grpc.io
    #[display("Failed to convert from gRPC response: {_0}")]
    InvalidProtobuf(ProtobufError),
}
