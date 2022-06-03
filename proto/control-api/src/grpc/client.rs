//! gRPC [`ControlApi`] and [`CallbackApi`] clients.

use async_trait::async_trait;
use derive_more::{Display, Error, From};
use tonic::codegen::{Body, Bytes};

use crate::{
    callback::Request as CallbackRequest,
    control::{ParseFidError, Request as ControlRequest},
    grpc::{
        api::{self as control_proto},
        callback::{
            self as callback_proto,
            callback_server::Callback as GrpcCallbackApi,
        },
        ControlApiClient, ProtobufError,
    },
    member,
    member::ParseSidError,
    CallbackApi, ControlApi, Elements, Fid, Ping, Pong,
};

/// [`Box`]ed [`Error`] with [`Send`] and [`Sync`].
///
/// [`Error`]: std::error::Error
type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[async_trait]
impl<T> GrpcCallbackApi for T
where
    T: CallbackApi + Send + Sync + 'static,
    T::Error: From<ProtobufError>,
    tonic::Status: From<T::Error>,
{
    async fn on_event(
        &self,
        req: tonic::Request<callback_proto::Request>,
    ) -> Result<tonic::Response<callback_proto::Response>, tonic::Status> {
        let req = CallbackRequest::try_from(req.into_inner())
            .map_err(T::Error::from)?;
        self.on_event(req)
            .await
            .map(|_| tonic::Response::new(callback_proto::Response {}))
            .map_err(Into::into)
    }
}

#[async_trait]
impl<T> ControlApi for ControlApiClient<T>
where
    T: Clone + tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync,
    T::Future: Send,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    type Error = ControlClientError;

    async fn create(
        &self,
        req: ControlRequest,
    ) -> Result<member::Sids, Self::Error> {
        // It's ok to `.clone()` here.
        // https://docs.rs/tonic/latest/tonic/client/index.html#concurrent-usage
        let mut this = self.clone();
        let resp =
            Self::create(&mut this, control_proto::CreateRequest::from(req))
                .await?
                .into_inner();

        if let Some(e) = resp.error {
            return Err(e.into());
        }

        resp.sid
            .into_iter()
            .map(|(id, sid)| {
                Ok((member::Id::from(id), sid.parse::<member::Sid>()?))
            })
            .collect()
    }

    async fn apply(
        &self,
        req: ControlRequest,
    ) -> Result<member::Sids, Self::Error> {
        // It's ok to `.clone()` here.
        // https://docs.rs/tonic/latest/tonic/client/index.html#concurrent-usage
        let mut this = self.clone();
        let resp =
            Self::apply(&mut this, control_proto::ApplyRequest::from(req))
                .await?
                .into_inner();

        if let Some(e) = resp.error {
            return Err(e.into());
        }

        resp.sid
            .into_iter()
            .map(|(id, sid)| {
                Ok((member::Id::from(id), sid.parse::<member::Sid>()?))
            })
            .collect()
    }

    async fn delete(&self, fids: &[Fid]) -> Result<(), Self::Error> {
        // It's ok to `.clone()` here.
        // https://docs.rs/tonic/latest/tonic/client/index.html#concurrent-usage
        let mut this = self.clone();
        let resp = Self::delete(
            &mut this,
            control_proto::IdRequest {
                fid: fids.iter().map(ToString::to_string).collect(),
            },
        )
        .await?
        .into_inner();

        if let Some(e) = resp.error {
            return Err(e.into());
        }

        Ok(())
    }

    async fn get(&self, fids: &[Fid]) -> Result<Elements, Self::Error> {
        // It's ok to `.clone()` here.
        // https://docs.rs/tonic/latest/tonic/client/index.html#concurrent-usage
        let mut this = self.clone();
        let resp = Self::get(
            &mut this,
            control_proto::IdRequest {
                fid: fids.iter().map(ToString::to_string).collect(),
            },
        )
        .await?
        .into_inner();

        if let Some(e) = resp.error {
            return Err(e.into());
        }

        resp.elements
            .into_iter()
            .map(|(fid, el)| Ok((fid.parse()?, el.try_into()?)))
            .collect()
    }

    async fn healthz(&self, ping: Ping) -> Result<Pong, Self::Error> {
        // It's ok to `.clone()` here.
        // https://docs.rs/tonic/latest/tonic/client/index.html#concurrent-usage
        let mut this = self.clone();
        Self::healthz(&mut this, control_proto::Ping::from(ping))
            .await
            .map(|resp| Pong(resp.into_inner().nonce))
            .map_err(Into::into)
    }
}

/// [`ControlApiClient`] error.
#[derive(Debug, Display, From, Error)]
pub enum ControlClientError {
    /// Failed to parse [`member::Sid`].
    #[display(fmt = "Failed to parse Sid: {}", _0)]
    ParseSidError(ParseSidError),

    /// Failed to parse [`Fid`].
    #[display(fmt = "Failed to parse Fid: {}", _0)]
    ParseFidError(ParseFidError),

    /// gRPC server errored.
    #[display(fmt = "gRPC server errored: {}", _0)]
    Tonic(tonic::Status),

    /// Failed to convert from protobuf.
    #[display(fmt = "Failed to convert from protobuf: {}", _0)]
    TryFromProtobufError(ProtobufError),

    /// [`ControlApi`] errored.
    #[display(fmt = "ControlApi errored: {:?}", _0)] // TODO
    ControlError(#[error(not(source))] control_proto::Error),
}
