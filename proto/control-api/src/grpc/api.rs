/// Request for creating a new `Element` on a media server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRequest {
    /// FID (Full ID) of the parent `Element` to create the provided `Element` in.
    #[prost(string, tag = "1")]
    pub parent_fid: ::prost::alloc::string::String,
    /// Spec of the created `Element`.
    #[prost(oneof = "create_request::El", tags = "2, 3, 4, 5")]
    pub el: ::core::option::Option<create_request::El>,
}
/// Nested message and enum types in `CreateRequest`.
pub mod create_request {
    /// Spec of the created `Element`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum El {
        #[prost(message, tag = "2")]
        Member(super::Member),
        #[prost(message, tag = "3")]
        Room(super::Room),
        #[prost(message, tag = "4")]
        WebrtcPlay(super::WebRtcPlayEndpoint),
        #[prost(message, tag = "5")]
        WebrtcPub(super::WebRtcPublishEndpoint),
    }
}
/// Request with many FIDs (Full IDs) of `Element`s.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdRequest {
    /// List of `Element`s FIDs.
    #[prost(string, repeated, tag = "1")]
    pub fid: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for applying a spec to an exiting `Element` on a media server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyRequest {
    /// FID (full ID) of the parent `Element` to apply the provided spec to.
    #[prost(string, tag = "1")]
    pub parent_fid: ::prost::alloc::string::String,
    /// Spec of the `Element` to be applied.
    #[prost(oneof = "apply_request::El", tags = "2, 3, 4, 5")]
    pub el: ::core::option::Option<apply_request::El>,
}
/// Nested message and enum types in `ApplyRequest`.
pub mod apply_request {
    /// Spec of the `Element` to be applied.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum El {
        #[prost(message, tag = "2")]
        Member(super::Member),
        #[prost(message, tag = "3")]
        Room(super::Room),
        #[prost(message, tag = "4")]
        WebrtcPlay(super::WebRtcPlayEndpoint),
        #[prost(message, tag = "5")]
        WebrtcPub(super::WebRtcPublishEndpoint),
    }
}
/// Response which doesn't return anything on success, but is fallible with an
/// `Error`.
///
/// If operation fails then an `Error` will be returned.
/// The response is considered successful only if it doesn't contain an `Error`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// Error of this `Response`.
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<Error>,
}
/// Response of `ControlApi.Create` RPC method.
///
/// If operation fails then an `Error` will be returned.
/// The response is considered successful only if it doesn't contain an `Error`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResponse {
    /// Hashmap with IDs (key) and URIs (value) of `Element`s, which should be used
    /// by clients to connect to a media server via Client API.
    ///
    /// Returned only if this `CreateResponse` is successful.
    #[prost(map = "string, string", tag = "1")]
    pub sid: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Error of this `CreateResponse`.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<Error>,
}
/// Response of `ControlApi.Get` RPC method.
///
/// If operation fails then an `Error` will be returned.
/// The response is considered successful only if it doesn't contain an `Error`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    /// Hashmap with IDs (key) and specs (value) of the requested `Elements`.
    ///
    /// Returned only if this `GetResponse` is successful.
    #[prost(map = "string, message", tag = "1")]
    pub elements: ::std::collections::HashMap<::prost::alloc::string::String, Element>,
    /// Error of this `GetResponse`.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<Error>,
}
/// Error of a failed request.
///
/// If an `Error` is not returned then a request is considered as successful.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// Concrete unique code of this `Error`.
    #[prost(uint32, tag = "1")]
    pub code: u32,
    /// Human-readable text description of this `Error`.
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    /// Link to online documentation of this `Error`.
    ///
    /// Optional field.
    #[prost(string, tag = "3")]
    pub doc: ::prost::alloc::string::String,
    /// FID (Full ID) of the `Element` that this `Error` is related to.
    /// Some `Error`s are not related to any `Element` and so have this field
    /// empty.
    ///
    /// Optional field.
    #[prost(string, tag = "4")]
    pub element: ::prost::alloc::string::String,
}
/// Possible media elements forming a media pipeline.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Element {
    #[prost(oneof = "element::El", tags = "1, 2, 3, 4")]
    pub el: ::core::option::Option<element::El>,
}
/// Nested message and enum types in `Element`.
pub mod element {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum El {
        #[prost(message, tag = "1")]
        Member(super::Member),
        #[prost(message, tag = "2")]
        Room(super::Room),
        #[prost(message, tag = "3")]
        WebrtcPlay(super::WebRtcPlayEndpoint),
        #[prost(message, tag = "4")]
        WebrtcPub(super::WebRtcPublishEndpoint),
    }
}
/// Media element representing a single space where multiple `Member`s can
/// interact with each other.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Room {
    /// ID of this `Room`.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Media pipeline representing this `Room`.
    #[prost(map = "string, message", tag = "2")]
    pub pipeline: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        room::Element,
    >,
}
/// Nested message and enum types in `Room`.
pub mod room {
    /// Possible media elements forming a `Room` pipeline.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Element {
        #[prost(oneof = "element::El", tags = "1, 2, 3")]
        pub el: ::core::option::Option<element::El>,
    }
    /// Nested message and enum types in `Element`.
    pub mod element {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum El {
            #[prost(message, tag = "1")]
            Member(super::super::Member),
            #[prost(message, tag = "2")]
            WebrtcPlay(super::super::WebRtcPlayEndpoint),
            #[prost(message, tag = "3")]
            WebrtcPub(super::super::WebRtcPublishEndpoint),
        }
    }
}
/// Media element representing a client authorized to participate in some bigger
/// media pipeline (`Room`, for example).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Member {
    /// ID of this `Member`.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// URL of the callback to fire when this `Member` establishes a persistent
    /// connection with a media server via Client API.
    #[prost(string, tag = "2")]
    pub on_join: ::prost::alloc::string::String,
    /// URL of the callback to fire when this `Member` finishes a persistent
    /// connection with a media server via Client API.
    #[prost(string, tag = "3")]
    pub on_leave: ::prost::alloc::string::String,
    /// URL of the callback to fire when this \[`Member`\] start traffic
    /// with a media server via \[Client API\].
    #[prost(string, tag = "4")]
    pub on_start: ::prost::alloc::string::String,
    /// URL of the callback to fire when this \[`Member`\] stopped traffic
    /// with a media server via \[Client API\].
    #[prost(string, tag = "5")]
    pub on_stop: ::prost::alloc::string::String,
    /// Timeout of receiving heartbeat messages from this `Member` via Client API.
    /// Once reached, this `Member` is considered being idle.
    #[prost(message, optional, tag = "8")]
    pub idle_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Timeout of reconnecting this `Member` via Client API.
    /// Once reached, this `Member` is considered disconnected.
    #[prost(message, optional, tag = "9")]
    pub reconnect_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Interval of pinging with heartbeat messages this `Member` via Client API
    /// by a media server.
    /// If empty then the default interval of a media server is used, if
    /// configured.
    #[prost(message, optional, tag = "10")]
    pub ping_interval: ::core::option::Option<::prost_types::Duration>,
    /// Media pipeline representing this `Member`.
    #[prost(map = "string, message", tag = "11")]
    pub pipeline: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        member::Element,
    >,
    /// Credentials to authenticate this `Member` in Client API with.
    ///
    /// Plain and hashed credentials are supported. If no credentials provided,
    /// then random plain string will be generated. If no authentication is
    /// required then empty plain string can be used.
    ///
    /// Hashed variant only supports Argon2 hash at the moment.
    /// `Member` sid won't contain a `token` query parameter if hashed credentials
    /// are used, so it should be appended manually on a client side.
    #[prost(oneof = "member::Credentials", tags = "6, 7")]
    pub credentials: ::core::option::Option<member::Credentials>,
}
/// Nested message and enum types in `Member`.
pub mod member {
    /// Elements which Member's pipeline can contain.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Element {
        #[prost(oneof = "element::El", tags = "1, 2")]
        pub el: ::core::option::Option<element::El>,
    }
    /// Nested message and enum types in `Element`.
    pub mod element {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum El {
            #[prost(message, tag = "1")]
            WebrtcPlay(super::super::WebRtcPlayEndpoint),
            #[prost(message, tag = "2")]
            WebrtcPub(super::super::WebRtcPublishEndpoint),
        }
    }
    /// Credentials to authenticate this `Member` in Client API with.
    ///
    /// Plain and hashed credentials are supported. If no credentials provided,
    /// then random plain string will be generated. If no authentication is
    /// required then empty plain string can be used.
    ///
    /// Hashed variant only supports Argon2 hash at the moment.
    /// `Member` sid won't contain a `token` query parameter if hashed credentials
    /// are used, so it should be appended manually on a client side.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Credentials {
        /// Argon2 hash of credentials.
        #[prost(string, tag = "6")]
        Hash(::prost::alloc::string::String),
        /// Plain text credentials.
        #[prost(string, tag = "7")]
        Plain(::prost::alloc::string::String),
    }
}
/// Media element receiving media data from a client via WebRTC (allows to
/// publish media data).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebRtcPublishEndpoint {
    /// ID of this `WebRtcPublishEndpoint`.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Peer-to-peer mode of this `WebRtcPublishEndpoint`.
    #[prost(enumeration = "web_rtc_publish_endpoint::P2p", tag = "2")]
    pub p2p: i32,
    /// Callback firing when a client starts publishing media data.
    #[prost(string, tag = "3")]
    pub on_start: ::prost::alloc::string::String,
    /// Callback firing when a client stops publishing media data.
    #[prost(string, tag = "4")]
    pub on_stop: ::prost::alloc::string::String,
    /// Indicator whether to relay all media data through a TURN server forcibly.
    #[prost(bool, tag = "5")]
    pub force_relay: bool,
    /// Settings for the audio media type of this `WebRtcPublishEndpoint`.
    #[prost(message, optional, tag = "6")]
    pub audio_settings: ::core::option::Option<web_rtc_publish_endpoint::AudioSettings>,
    /// Settings for the video media type of this `WebRtcPublishEndpoint`.
    #[prost(message, optional, tag = "7")]
    pub video_settings: ::core::option::Option<web_rtc_publish_endpoint::VideoSettings>,
}
/// Nested message and enum types in `WebRtcPublishEndpoint`.
pub mod web_rtc_publish_endpoint {
    /// Audio media type settings of a `WebRtcPublishEndpoint`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AudioSettings {
        /// Policy to publish the audio media type with.
        #[prost(enumeration = "PublishPolicy", tag = "1")]
        pub publish_policy: i32,
    }
    /// Video media type settings of `WebRtcPublishEndpoint`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VideoSettings {
        /// Policy to publish the video media type with.
        #[prost(enumeration = "PublishPolicy", tag = "1")]
        pub publish_policy: i32,
    }
    /// Policy of how a video or an audio media type can be published in a
    /// `WebRtcPublishEndpoint`.
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
    pub enum PublishPolicy {
        /// Media type MAY be published.
        ///
        /// Media server will try to initialize publishing, but won't produce any
        /// errors if user application fails to (or chooses not to) acquire the
        /// required media track. Media server will approve user requests to stop and
        /// to restart publishing the specified media type.
        Optional = 0,
        /// Media type MUST be published.
        ///
        /// Media server will try to initialize publishing, and if the required media
        /// track cannot be acquired, then an error will be thrown. Media server will
        /// deny all requests to stop publishing.
        Required = 1,
        /// Media type MUST NOT be published.
        ///
        /// Media server will not try to initialize publishing.
        Disabled = 2,
    }
    impl PublishPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PublishPolicy::Optional => "OPTIONAL",
                PublishPolicy::Required => "REQUIRED",
                PublishPolicy::Disabled => "DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPTIONAL" => Some(Self::Optional),
                "REQUIRED" => Some(Self::Required),
                "DISABLED" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
    /// Possible peer-to-peer modes of WebRTC interaction in a
    /// `WebRtcPublishEndpoint`.
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
    pub enum P2p {
        /// Never use peer-to-peer connections and always send media data through a
        /// media server.
        Never = 0,
        /// Use peer-to-peer connections directly if it's possible, otherwise send
        /// media data through a media server.
        IfPossible = 1,
        /// Send media data via peer-to-peer connections only, and never through a
        /// media server.
        Always = 2,
    }
    impl P2p {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                P2p::Never => "NEVER",
                P2p::IfPossible => "IF_POSSIBLE",
                P2p::Always => "ALWAYS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NEVER" => Some(Self::Never),
                "IF_POSSIBLE" => Some(Self::IfPossible),
                "ALWAYS" => Some(Self::Always),
                _ => None,
            }
        }
    }
}
/// Media element playing media data for a client via WebRTC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebRtcPlayEndpoint {
    /// ID of this `WebRtcPlayEndpoint`.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Source to play media data from.
    #[prost(string, tag = "2")]
    pub src: ::prost::alloc::string::String,
    /// Callback firing when a client starts playing media data from the source.
    #[prost(string, tag = "3")]
    pub on_start: ::prost::alloc::string::String,
    /// Callback firing when a client stops playing media data from the source.
    #[prost(string, tag = "4")]
    pub on_stop: ::prost::alloc::string::String,
    /// Indicator whether to relay all media data through a TURN server forcibly.
    #[prost(bool, tag = "5")]
    pub force_relay: bool,
}
/// Ping message received by a media server periodically for probing its
/// healthiness.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {
    /// Each new `Ping` should increment its nonce, starting with 0.
    #[prost(uint32, tag = "1")]
    pub nonce: u32,
}
/// Pong message sent by a media server in response to a received `Ping` message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pong {
    /// / Nonce of the answered `Ping` message.
    #[prost(uint32, tag = "1")]
    pub nonce: u32,
}
/// Generated client implementations.
pub mod control_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service allowing to control a media server dynamically, by creating, updating
    /// and destroying pipelines of media `Element`s on it.
    #[derive(Debug, Clone)]
    pub struct ControlApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ControlApiClient<tonic::transport::Channel> {
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
    impl<T> ControlApiClient<T>
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
        ) -> ControlApiClient<InterceptedService<T, F>>
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
            ControlApiClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new `Element` on the media server.
        ///
        /// Non-idempotent. Errors if an `Element` with such ID already exists.
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/api.ControlApi/Create");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.ControlApi", "Create"));
            self.inner.unary(req, path, codec).await
        }
        /// Removes `Element`s from the media server.
        /// Allows referring multiple `Element`s on the last two levels of a FID.
        ///
        /// Idempotent. If no `Element`s with such FIDs exist, then succeeds.
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::IdRequest>,
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
            let path = http::uri::PathAndQuery::from_static("/api.ControlApi/Delete");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.ControlApi", "Delete"));
            self.inner.unary(req, path, codec).await
        }
        /// Lookups `Element`s by their FIDs on the media server.
        /// If no FIDs are specified, then returns all the current `Element`s on the
        /// media server.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::IdRequest>,
        ) -> std::result::Result<tonic::Response<super::GetResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/api.ControlApi/Get");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.ControlApi", "Get"));
            self.inner.unary(req, path, codec).await
        }
        /// Applies changes to an existing `Element` on the media server, or creates a
        /// new one in case there is no `Element` with such ID.
        ///
        /// Idempotent. If no `Element` with such ID exists, then it will be created,
        /// otherwise it will be reconfigured. `Element`s that exist on the same
        /// hierarchy level, but are not specified in the provided spec, will be
        /// removed.
        pub async fn apply(
            &mut self,
            request: impl tonic::IntoRequest<super::ApplyRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/api.ControlApi/Apply");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.ControlApi", "Apply"));
            self.inner.unary(req, path, codec).await
        }
        /// Checks healthiness of the media server.
        /// Caller should assert that the returned `Pong` has the same nonce as the
        /// sent `Ping`.
        pub async fn healthz(
            &mut self,
            request: impl tonic::IntoRequest<super::Ping>,
        ) -> std::result::Result<tonic::Response<super::Pong>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/api.ControlApi/Healthz");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.ControlApi", "Healthz"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod control_api_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ControlApiServer.
    #[async_trait]
    pub trait ControlApi: Send + Sync + 'static {
        /// Creates a new `Element` on the media server.
        ///
        /// Non-idempotent. Errors if an `Element` with such ID already exists.
        async fn create(
            &self,
            request: tonic::Request<super::CreateRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateResponse>, tonic::Status>;
        /// Removes `Element`s from the media server.
        /// Allows referring multiple `Element`s on the last two levels of a FID.
        ///
        /// Idempotent. If no `Element`s with such FIDs exist, then succeeds.
        async fn delete(
            &self,
            request: tonic::Request<super::IdRequest>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status>;
        /// Lookups `Element`s by their FIDs on the media server.
        /// If no FIDs are specified, then returns all the current `Element`s on the
        /// media server.
        async fn get(
            &self,
            request: tonic::Request<super::IdRequest>,
        ) -> std::result::Result<tonic::Response<super::GetResponse>, tonic::Status>;
        /// Applies changes to an existing `Element` on the media server, or creates a
        /// new one in case there is no `Element` with such ID.
        ///
        /// Idempotent. If no `Element` with such ID exists, then it will be created,
        /// otherwise it will be reconfigured. `Element`s that exist on the same
        /// hierarchy level, but are not specified in the provided spec, will be
        /// removed.
        async fn apply(
            &self,
            request: tonic::Request<super::ApplyRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateResponse>, tonic::Status>;
        /// Checks healthiness of the media server.
        /// Caller should assert that the returned `Pong` has the same nonce as the
        /// sent `Ping`.
        async fn healthz(
            &self,
            request: tonic::Request<super::Ping>,
        ) -> std::result::Result<tonic::Response<super::Pong>, tonic::Status>;
    }
    /// Service allowing to control a media server dynamically, by creating, updating
    /// and destroying pipelines of media `Element`s on it.
    #[derive(Debug)]
    pub struct ControlApiServer<T: ControlApi> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ControlApi> ControlApiServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ControlApiServer<T>
    where
        T: ControlApi,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api.ControlApi/Create" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSvc<T: ControlApi>(pub Arc<T>);
                    impl<T: ControlApi> tonic::server::UnaryService<super::CreateRequest>
                    for CreateSvc<T> {
                        type Response = super::CreateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlApi>::create(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.ControlApi/Delete" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSvc<T: ControlApi>(pub Arc<T>);
                    impl<T: ControlApi> tonic::server::UnaryService<super::IdRequest>
                    for DeleteSvc<T> {
                        type Response = super::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlApi>::delete(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.ControlApi/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: ControlApi>(pub Arc<T>);
                    impl<T: ControlApi> tonic::server::UnaryService<super::IdRequest>
                    for GetSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlApi>::get(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.ControlApi/Apply" => {
                    #[allow(non_camel_case_types)]
                    struct ApplySvc<T: ControlApi>(pub Arc<T>);
                    impl<T: ControlApi> tonic::server::UnaryService<super::ApplyRequest>
                    for ApplySvc<T> {
                        type Response = super::CreateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ApplyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlApi>::apply(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ApplySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.ControlApi/Healthz" => {
                    #[allow(non_camel_case_types)]
                    struct HealthzSvc<T: ControlApi>(pub Arc<T>);
                    impl<T: ControlApi> tonic::server::UnaryService<super::Ping>
                    for HealthzSvc<T> {
                        type Response = super::Pong;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Ping>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlApi>::healthz(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HealthzSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ControlApi> Clone for ControlApiServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ControlApi> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ControlApi> tonic::server::NamedService for ControlApiServer<T> {
        const NAME: &'static str = "api.ControlApi";
    }
}
