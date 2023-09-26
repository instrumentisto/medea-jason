/// Request with a fired callback event and its meta information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    /// FID (Full ID) of the media `Element`, the occurred event is related to.
    #[prost(string, tag = "1")]
    pub fid: ::prost::alloc::string::String,
    /// Datetime when the event occurred.
    #[prost(string, tag = "2")]
    pub at: ::prost::alloc::string::String,
    /// Occurred event.
    #[prost(oneof = "request::Event", tags = "3, 4")]
    pub event: ::core::option::Option<request::Event>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    /// Occurred event.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag = "3")]
        OnJoin(super::OnJoin),
        #[prost(message, tag = "4")]
        OnLeave(super::OnLeave),
    }
}
/// Empty response of the `Callback` service.
///
/// We don't use `google.protobuf.Empty` to be able to add some fields (if
/// necessary) in the future.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {}
/// Event notifying about a `Member` joining a `Room`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnJoin {}
/// Event notifying about a `Member` leaving its `Room`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnLeave {
    /// Reason of why the `Member` leaves.
    #[prost(enumeration = "on_leave::Reason", tag = "1")]
    pub reason: i32,
}
/// Nested message and enum types in `OnLeave`.
pub mod on_leave {
    /// Possible reasons of why a `Member` leaves its `Room`.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Reason {
        /// `Member` was disconnected normally.
        Disconnected = 0,
        /// Connection with the `Member` was lost.
        Lost = 1,
        /// `Member` was forcibly disconnected by a media server.
        Kicked = 2,
        /// Media server was shut down.
        Shutdown = 3,
    }
    impl Reason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reason::Disconnected => "DISCONNECTED",
                Reason::Lost => "LOST",
                Reason::Kicked => "KICKED",
                Reason::Shutdown => "SHUTDOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DISCONNECTED" => Some(Self::Disconnected),
                "LOST" => Some(Self::Lost),
                "KICKED" => Some(Self::Kicked),
                "SHUTDOWN" => Some(Self::Shutdown),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod callback_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for receiving callbacks from a media server.
    #[derive(Debug, Clone)]
    pub struct CallbackClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CallbackClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CallbackClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CallbackClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CallbackClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Fires when a certain callback event happens on a media server.
        pub async fn on_event(
            &mut self,
            request: impl tonic::IntoRequest<super::Request>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/callback.Callback/OnEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("callback.Callback", "OnEvent"));
            self.inner.unary(req, path, codec).await
        }
    }
}
