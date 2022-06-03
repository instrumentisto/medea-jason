//! gRPC [`ControlApi`] and [`CallbackApi`] servers.

use std::collections::HashMap;

use async_trait::async_trait;
use derive_more::{Display, Error, From};
use tonic::codegen::{Body, Bytes};

use crate::{
    callback::Request as CallbackRequest,
    control::Request as ControlRequest,
    grpc::{
        api::{
            self as control_proto,
            control_api_server::ControlApi as GrpcControlApi,
        },
        callback as callback_proto, CallbackClient, ProtobufError,
    },
    CallbackApi, ControlApi, Ping,
};

/// [`Box`]ed [`Error`] with [`Send`] and [`Sync`].
///
/// [`Error`]: std::error::Error
type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[async_trait]
impl<T> GrpcControlApi for T
where
    T: ControlApi + Send + Sync + 'static,
    T::Error: From<ProtobufError> + Into<control_proto::Error>,
{
    async fn create(
        &self,
        req: tonic::Request<control_proto::CreateRequest>,
    ) -> Result<tonic::Response<control_proto::CreateResponse>, tonic::Status>
    {
        let fut = async {
            let req = ControlRequest::try_from(req.into_inner())?;
            self.create(req).await
        };

        Ok(tonic::Response::new(match fut.await {
            Ok(sids) => control_proto::CreateResponse {
                sid: sids
                    .into_iter()
                    .map(|(id, sid)| (id.to_string(), sid.to_string()))
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
        req: tonic::Request<control_proto::IdRequest>,
    ) -> Result<tonic::Response<control_proto::Response>, tonic::Status> {
        let ids = req
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
            Ok(_) => control_proto::Response { error: None },
            Err(e) => control_proto::Response {
                error: Some(e.into()),
            },
        }))
    }

    async fn get(
        &self,
        req: tonic::Request<control_proto::IdRequest>,
    ) -> Result<tonic::Response<control_proto::GetResponse>, tonic::Status>
    {
        let ids = req
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
                    .map(|(id, el)| (id.to_string(), el.into()))
                    .collect(),
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
        req: tonic::Request<control_proto::ApplyRequest>,
    ) -> Result<tonic::Response<control_proto::CreateResponse>, tonic::Status>
    {
        let result = async {
            let req = ControlRequest::try_from(req.into_inner())?;
            self.apply(req).await
        };

        Ok(tonic::Response::new(match result.await {
            Ok(sids) => control_proto::CreateResponse {
                sid: sids
                    .into_iter()
                    .map(|(id, sid)| (id.to_string(), sid.to_string()))
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
        self.healthz(Ping(request.into_inner().nonce))
            .await
            .map(|pong| {
                tonic::Response::new(control_proto::Pong { nonce: pong.0 })
            })
            .map_err(|e| {
                let e = e.into();
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
impl<T> CallbackApi for CallbackClient<T>
where
    T: Clone + tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync,
    T::Future: Send,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    type Error = CallbackClientError;

    async fn on_event(&self, req: CallbackRequest) -> Result<(), Self::Error> {
        // It's ok to `.clone()` here.
        // https://docs.rs/tonic/latest/tonic/client/index.html#concurrent-usage
        let mut this = self.clone();

        Self::on_event(&mut this, callback_proto::Request::from(req))
            .await
            .map(drop)
            .map_err(Into::into)
    }
}

/// [`CallbackClient`] error.
#[derive(Debug, Display, From, Error)]
pub enum CallbackClientError {
    /// gRPC server errored.
    #[display(fmt = "gRPC server errored: {}", _0)]
    Tonic(tonic::Status),

    /// Failed to convert from protobuf.
    #[display(fmt = "Failed to convert from protobuf: {}", _0)]
    TryFromProtobufError(ProtobufError),
}
