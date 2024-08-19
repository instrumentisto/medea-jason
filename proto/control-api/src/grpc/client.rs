//! [`ControlApi`] client and [`CallbackApi`] server [gRPC] implementations.
//!
//! [gRPC]: https://grpc.io

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
            callback_server::Callback as GrpcCallbackService,
        },
        ControlApiClient, ProtobufError,
    },
    member,
    member::ParseSidError,
    CallbackApi, ControlApi, Elements, Fid, Ping, Pong,
};

/// [`Box`]ed [`Error`] with [`Send`] and [`Sync`].
type StdError = Box<dyn Error + Send + Sync + 'static>;

#[async_trait]
impl<T: ?Sized> GrpcCallbackService for T
where
    T: CallbackApi + Send + Sync + 'static,
    T::Error: From<ProtobufError>,
    tonic::Status: From<T::Error>,
{
    async fn on_event(
        &self,
        request: tonic::Request<callback_proto::Request>,
    ) -> Result<tonic::Response<callback_proto::Response>, tonic::Status> {
        let req = CallbackRequest::try_from(request.into_inner())
            .map_err(T::Error::from)?;
        self.on_event(req)
            .await
            .map(|()| tonic::Response::new(callback_proto::Response {}))
            .map_err(Into::into)
    }
}

#[async_trait]
impl<T> ControlApi for ControlApiClient<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + Clone + Send + Sync,
    T::Future: Send,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Send,
    StdError: From<<T::ResponseBody as Body>::Error>,
{
    type Error = ControlApiClientError;

    async fn create(
        &self,
        request: ControlRequest,
    ) -> Result<member::Sids, Self::Error> {
        // It's OK to `.clone()` `tonic::client`:
        // https://docs.rs/tonic/latest/tonic/client/index.html#concurrent-usage
        let mut this = self.clone();

        let resp = Self::create(
            &mut this,
            control_proto::CreateRequest::from(request),
        )
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
        request: ControlRequest,
    ) -> Result<member::Sids, Self::Error> {
        // It's OK to `.clone()` `tonic::client`:
        // https://docs.rs/tonic/latest/tonic/client/index.html#concurrent-usage
        let mut this = self.clone();

        let resp =
            Self::apply(&mut this, control_proto::ApplyRequest::from(request))
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
        // It's OK to `.clone()` `tonic::client`:
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
        // It's OK to `.clone()` `tonic::client`:
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
        // It's OK to `.clone()` `tonic::client`:
        // https://docs.rs/tonic/latest/tonic/client/index.html#concurrent-usage
        let mut this = self.clone();

        Ok(Self::healthz(&mut this, control_proto::Ping::from(ping))
            .await?
            .into_inner()
            .into())
    }
}

/// Possible errors of [`ControlApiClient`].
#[derive(Debug, Display, From, Error)]
pub enum ControlApiClientError {
    /// Failed to parse [`member::Sid`].
    #[display("Invalid SID: {_0}")]
    InvalidSid(ParseSidError),

    /// Failed to parse [`Fid`].
    #[display("Invalid FID: {_0}")]
    InvalidFid(ParseFidError),

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

    /// [`ControlApi`] server implementation errored.
    #[display("Control API server errored: {_0:?}")]
    ControlError(#[error(not(source))] control_proto::Error),
}
