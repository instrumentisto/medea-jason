#![doc = include_str!("../README.md")]
#![deny(
    macro_use_extern_crate,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    trivial_casts,
    trivial_numeric_casts
)]
#![forbid(non_ascii_idents, unsafe_code)]
#![warn(
    clippy::as_conversions,
    clippy::branches_sharing_code,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::decimal_literal_representation,
    clippy::else_if_without_else,
    clippy::empty_line_after_outer_attr,
    clippy::equatable_if_let,
    clippy::exit,
    clippy::expect_used,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast,
    clippy::fn_to_numeric_cast_any,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::index_refutable_slice,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::option_if_let_else,
    clippy::panic_in_result_fn,
    clippy::pedantic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::shadow_unrelated,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::trailing_empty_array,
    clippy::trivial_regex,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm,
    future_incompatible,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

#[cfg(feature = "grpc")]
pub mod grpc;

use std::{
    collections::HashMap, fmt, fmt::Debug, future::Future, pin::Pin, sync::Arc,
};

use async_trait::async_trait;
use derive_more::{Display, Error, From};
use either::Either;
use time::PrimitiveDateTime;

pub use self::{
    endpoint::{Endpoint, WebRtcPlay, WebRtcPublish},
    member::Member,
    room::Room,
};

#[async_trait]
pub trait ControlApi {
    async fn create_room(&self, spec: Room) -> Result<Sids, ErrorResponse>;

    async fn apply_room(&self, spec: Room) -> Result<Sids, ErrorResponse>;

    async fn create_room_member(
        &self,
        parent_id: Fid<ToRoom>,
        spec: Member,
    ) -> Result<Sids, ErrorResponse>;

    async fn apply_room_member(
        &self,
        fid: Fid<ToRoom>,
        spec: Member,
    ) -> Result<Sids, ErrorResponse>;

    async fn create_room_endpoint(
        &self,
        parent_id: Fid<ToMember>,
        spec: Endpoint,
    ) -> Result<Sids, ErrorResponse>;

    async fn apply_room_endpoint(
        &self,
        parent_id: Fid<ToMember>,
        spec: Endpoint,
    ) -> Result<Sids, ErrorResponse>;

    async fn delete_elements(
        &self,
        fids: Vec<StatefulFid>,
    ) -> Result<Sids, ErrorResponse>;

    async fn get(
        &self,
        fids: Vec<StatefulFid>,
    ) -> Result<Elements, ErrorResponse>;

    async fn healthz(&self, ping: Ping) -> Result<Pong, ErrorResponse>;
}

#[async_trait]
pub trait Callback<OnJoin, OnLeave> {
    async fn on_join(fid: Fid<ToMember>, value: OnJoin, at: PrimitiveDateTime);

    async fn on_leave(
        fid: Fid<ToMember>,
        value: OnLeave,
        reason: Reason,
        at: PrimitiveDateTime,
    );
}

pub type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

/// Factory for a [`CallbackClient`]s.
pub trait CallbackClientFactory<OnJoin, OnLeave> {
    /// Creates [`CallbackClient`] basing on provided [`CallbackUrl`].
    fn build(
        url: Either<OnJoin, OnLeave>,
    ) -> LocalBoxFuture<'static, CallbackResult<Arc<dyn CallbackClient>>>;
}

/// Abstraction of a Control API callback client.
#[async_trait(?Send)]
pub trait CallbackClient: Debug + Send + Sync {
    /// Sends provided [`CallbackRequest`].
    async fn send(&self, request: CallbackRequest) -> CallbackResult;
}

/// Control API callback.
///
/// Used for sending callbacks with [`CallbackClient::send`].
///
/// [`CallbackClient::send`]:
/// crate::api::control::callback::clients::CallbackClient::send
#[derive(Debug)]
pub struct CallbackRequest {
    /// FID (Full ID) of element with which event was occurred.
    pub fid: StatefulFid,

    /// [`CallbackEvent`] which occurred.
    pub event: CallbackEvent,

    /// Time at which event occurred.
    pub at: PrimitiveDateTime,
}

/// All callbacks which can happen.
#[derive(Clone, Copy, Debug, From)]
pub enum CallbackEvent {
    OnJoin(OnJoinEvent),
    OnLeave(OnLeaveEvent),
}

/// Event for `on_leave` `Member` callback.
#[derive(Clone, Copy, Debug)]
pub struct OnLeaveEvent {
    /// Reason of why `Member` was lost.
    pub reason: OnLeaveReason,
}

impl OnLeaveEvent {
    #[must_use]
    pub const fn new(reason: OnLeaveReason) -> Self {
        Self { reason }
    }
}

/// Reason of why `Member` was lost.
#[derive(Clone, Copy, Debug)]
pub enum OnLeaveReason {
    /// `Member` was normally disconnected.
    Disconnected,

    /// Connection with `Member` was lost.
    LostConnection,

    /// `Member` was forcibly disconnected by server.
    Kicked,

    /// Server is shutting down.
    ServerShutdown,
}

/// `on_join` `Member` callback for Control API.
#[derive(Clone, Copy, Debug)]
pub struct OnJoinEvent;

/// Shortcut for [`Result`] of methods in this module.
pub type CallbackResult<T = ()> = Result<T, CallbackClientError>;

/// Error of sending [`CallbackRequest`] by [`CallbackClient`].
#[derive(Debug, Display, Error, From)]
pub enum CallbackClientError {
    /// [`tonic`] failed to send [`CallbackRequest`].
    #[display(fmt = "gRPC request failed: {}", _0)]
    Request(#[error(not(source))] tonic::Status),

    /// Error while creating a new [`CallbackClient`].
    #[display(fmt = "Failed to initialize gRPC callback client: {}", _0)]
    Initialization(tonic::transport::Error),
}

pub struct Ping(pub u32);

pub struct Pong(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Reason {
    /// Member was normally disconnected.
    Disconnected,
    /// Connection with Member was lost.
    LostConnection,
    /// Member was forcibly disconnected by server.
    Kicked,
    /// Medea media server is shutting down.
    ServerShutdown,
}

/// Serialized to protobuf `Element`s which will be returned from [`Get`] on
/// success result.
pub type Elements = HashMap<StatefulFid, Element>;

#[derive(Clone, Debug)]
pub enum Element {
    Member(Member),
    Room(Room),
    Endpoint(Endpoint),
}

/// FID (full ID, or `fid` in Control API specs) is a composition of
/// media elements IDs, which refers to some media element on a whole server
/// uniquely.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Fid<T> {
    pub state: T,
}

impl Fid<ToRoom> {
    #[doc = "Create new reference in [`ToRoom`] state."]
    #[must_use]
    pub const fn new(room_id: room::Id) -> Self {
        Self {
            state: ToRoom(room_id),
        }
    }
}

impl Fid<ToMember> {
    /// Create new reference in [`ToMember`] state.
    ///
    /// [`ToMember`]: crate::api::control::refs::ToMember
    #[must_use]
    pub const fn new(room_id: room::Id, member_id: member::Id) -> Self {
        Self {
            state: ToMember(room_id, member_id),
        }
    }
}

impl Fid<ToEndpoint> {
    /// Creates new reference in [`ToEndpoint`] state.
    ///
    /// [`ToEndpoint`]: crate::api::control::refs::ToEndpoint
    #[must_use]
    pub const fn new(
        room_id: room::Id,
        member_id: member::Id,
        endpoint_id: endpoint::Id,
    ) -> Self {
        Self {
            state: ToEndpoint(room_id, member_id, endpoint_id),
        }
    }
}

impl fmt::Display for Fid<ToRoom> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.state.0)
    }
}

impl fmt::Display for Fid<ToMember> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.state.0, self.state.1)
    }
}

impl fmt::Display for Fid<ToEndpoint> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}/{}", self.state.0, self.state.1, self.state.2)
    }
}

impl From<Fid<ToMember>> for Fid<ToRoom> {
    fn from(id: Fid<ToMember>) -> Self {
        Self::new(id.state.0)
    }
}

/// Enum for storing [`Fid`]s in all states.
#[derive(Clone, Debug, Display, Eq, From, Hash, PartialEq)]
pub enum StatefulFid {
    Room(Fid<ToRoom>),
    Member(Fid<ToMember>),
    Endpoint(Fid<ToEndpoint>),
}

/// State of reference which points to [`Room`].
///
/// [`Room`]: crate::signalling::room::Room
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ToRoom(pub room::Id);

impl From<String> for ToRoom {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

/// State of reference which points to [`Member`].
///
/// [`Member`]: crate::signalling::elements::Member
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ToMember(pub room::Id, pub member::Id);

/// State of reference which points to [`Endpoint`].
///
/// [`Endpoint`]: crate::signalling::elements::endpoints::Endpoint
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ToEndpoint(pub room::Id, pub member::Id, pub endpoint::Id);

/// Type alias for success [`CreateResponse`]'s sids.
///
/// [`CreateResponse`]: medea_control_api_proto::grpc::api::CreateResponse
pub type Sids = HashMap<member::Id, member::Sid>;

pub mod room {
    use std::collections::HashMap;

    use derive_more::{Display, From, Into};
    use serde::{Deserialize, Serialize};

    use super::{member, Member};

    /// ID of a `Room`.
    #[derive(
        Clone,
        Debug,
        Display,
        Serialize,
        Deserialize,
        Eq,
        From,
        Hash,
        Into,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub struct Id(pub String);

    /// [Control API]'s `Room` element specification.
    ///
    /// Newtype for [`RootElement::Room`].
    ///
    /// [Control API]: https://tinyurl.com/yxsqplq7
    #[derive(Clone, Debug)]
    pub struct Room {
        pub id: Id,
        pub pipeline: HashMap<member::Id, Member>,
    }
}

pub mod member {
    use std::{collections::HashMap, fmt, time::Duration};

    use derive_more::{Display, From, Into};
    use serde::{Deserialize, Serialize};

    use super::{endpoint, room, Endpoint};

    /// ID of a `Member`.
    #[derive(
        Clone,
        Debug,
        Display,
        Serialize,
        Deserialize,
        Eq,
        From,
        Hash,
        Into,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub struct Id(pub String);

    /// Newtype for [`RoomElement::Member`] variant.
    #[derive(Clone, Debug)]
    pub struct Member {
        pub id: Id,

        /// Spec of this `Member`.
        pub pipeline: HashMap<endpoint::Id, Endpoint>,

        /// Credentials to authorize `Member` with.
        pub credentials: Credentials,

        /// URL to which `OnJoin` Control API callback will be sent.
        pub on_join: Option<String>,

        /// URL to which `OnLeave` Control API callback will be sent.
        pub on_leave: Option<String>,

        /// Timeout of receiving heartbeat messages from the `Member` via
        /// Client API.
        ///
        /// Once reached, the `Member` is considered being idle.
        pub idle_timeout: Option<Duration>,

        /// Timeout of the `Member` reconnecting via Client API.
        ///
        /// Once reached, the `Member` is considered disconnected.
        pub reconnect_timeout: Option<Duration>,

        /// Interval of sending `Ping`s to the `Member` via Client API.
        pub ping_interval: Option<Duration>,
    }

    /// URI used by `Member`s to connect to a media server via Client API.
    #[derive(Clone, Debug)]
    pub struct Sid {
        /// Public URL of HTTP server to establish WebSocket connection with.
        pub public_url: PublicUrl,

        /// [`RoomId`] of the `Room` the `Member` participates in.
        pub room_id: room::Id,

        pub member_id: Id,

        /// [`Credential`] of the `Member` to authorize his connection with.
        pub credentials: Credentials,
    }

    impl fmt::Display for Sid {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}/{}/{}",
                self.public_url, self.room_id, self.member_id,
            )?;
            if let Credentials::Plain(plain) = &self.credentials {
                write!(f, "?token={plain}")?;
            }
            Ok(())
        }
    }

    /// Public URL of HTTP server. Address for exposed [Client API].
    /// It's assumed that HTTP server can be reached via this URL externally.
    ///
    /// This address is returned from [Control API] in `sids` field and [Jason]
    /// uses this address to start its session.
    ///
    /// [Client API]: https://tinyurl.com/yx9thsnr
    /// [Control API]: https://tinyurl.com/yxsqplq7
    /// [Jason]: https://github.com/instrumentisto/medea-jason
    #[derive(Clone, Debug, Display, Deserialize, Serialize, From)]
    pub struct PublicUrl(pub String);

    /// Credentials of the `Member` element.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
    #[serde(rename_all = "lowercase")]
    pub enum Credentials {
        /// [Argon2] hash of the `Member` credential.
        ///
        /// [Argon2]: https://en.wikipedia.org/wiki/Argon2
        Hash(String),

        /// Plain text `Member` credentials.
        Plain(String),
    }

    impl Credentials {
        /// Length of [`Credentials`].
        const LEN: usize = 32;
    }

    impl Default for Credentials {
        fn default() -> Self {
            use rand::Rng as _;

            Self::Plain(
                rand::thread_rng()
                    .sample_iter(&rand::distributions::Alphanumeric)
                    .take(Self::LEN)
                    .map(char::from)
                    .collect(),
            )
        }
    }
}

pub mod endpoint {
    use derive_more::{Display, From, Into};
    use serde::Deserialize;

    pub use self::{web_rtc_play::WebRtcPlay, web_rtc_publish::WebRtcPublish};

    /// ID of `Endpoint`.
    #[derive(
        Clone, Debug, Deserialize, Display, Eq, From, Hash, Into, PartialEq,
    )]
    pub struct Id(pub String);

    impl From<web_rtc_publish::Id> for Id {
        fn from(id: web_rtc_publish::Id) -> Self {
            id.0.into()
        }
    }

    impl From<web_rtc_play::Id> for Id {
        fn from(id: web_rtc_play::Id) -> Self {
            id.0.into()
        }
    }

    impl From<Id> for web_rtc_publish::Id {
        fn from(id: Id) -> Self {
            id.0.into()
        }
    }

    impl From<Id> for web_rtc_play::Id {
        fn from(id: Id) -> Self {
            id.0.into()
        }
    }

    /// Media element that one or more media data streams flow through.
    #[derive(Clone, Debug, From)]
    pub enum Endpoint {
        /// [`WebRtcPublishEndpoint`] element.
        WebRtcPublish(WebRtcPublish),

        /// [`WebRtcPlayEndpoint`] element.
        WebRtcPlay(WebRtcPlay),
    }

    pub mod web_rtc_publish {
        use derive_more::{Display, From, Into};
        use serde::Deserialize;
        use smart_default::SmartDefault;

        /// ID of [`WebRtcPublishEndpoint`].
        #[derive(
            Clone, Debug, Deserialize, Display, Eq, From, Hash, Into, PartialEq,
        )]
        pub struct Id(pub String);

        /// Media element which is able to publish media data for another client
        /// via WebRTC.
        #[derive(Clone, Deserialize, Debug)]
        pub struct WebRtcPublish {
            pub id: Id,

            /// Peer-to-peer mode of this [`WebRtcPublishEndpoint`].
            pub p2p: P2pMode,

            /// Option to relay all media through a TURN server forcibly.
            #[serde(default)]
            pub force_relay: bool,

            /// Settings for the audio media type of the
            /// [`WebRtcPublishEndpoint`].
            #[serde(default)]
            pub audio_settings: AudioSettings,

            /// Settings for the video media type of the
            /// [`WebRtcPublishEndpoint`].
            #[serde(default)]
            pub video_settings: VideoSettings,
        }

        /// Peer-to-peer mode of [`WebRtcPublishEndpoint`].
        #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
        #[repr(i32)]
        pub enum P2pMode {
            /// Never connect peer-to-peer.
            Never = 0,

            /// Connect peer-to-peer if it possible.
            IfPossible = 1,

            /// Always connect peer-to-peer.
            Always = 2,
        }

        /// Settings for the audio media type of the [`WebRtcPublishEndpoint`].
        #[derive(Clone, Copy, Debug, Default, Deserialize)]
        pub struct AudioSettings {
            /// Publishing policy of the audio media type in the
            /// [`WebRtcPublishEndpoint`].
            #[serde(default)]
            pub publish_policy: PublishPolicy,
        }

        /// Settings for the video media type of the [`WebRtcPublishEndpoint`].
        #[derive(Clone, Copy, Debug, Default, Deserialize)]
        pub struct VideoSettings {
            /// Publishing policy of the video media type in the
            /// [`WebRtcPublishEndpoint`].
            #[serde(default)]
            pub publish_policy: PublishPolicy,
        }

        /// Publishing policy of the video or audio media type in the
        /// [`WebRtcPublishEndpoint`].
        #[derive(
            Clone, Copy, Debug, Deserialize, Eq, PartialEq, SmartDefault,
        )]
        pub enum PublishPolicy {
            /// Specified media type __may__ be published.
            ///
            /// Media server will try to initialize publishing, but won't
            /// produce any errors if user application will fail to
            /// or choose not to acquire required track. Media
            /// server will approve user request to stop and
            /// restart publishing specified media type.
            #[default]
            Optional = 0,

            /// Specified media type __must__ be published.
            ///
            /// Media server will try to initialize publishing. If required
            /// media track could not be acquired, then an error
            /// will be thrown. Media server will deny all requests
            /// to stop publishing.
            Required = 1,

            /// Media type __must__ not be published.
            ///
            /// Media server will not try to initialize publishing.
            Disabled = 2,
        }
    }

    pub mod web_rtc_play {
        use derive_more::{Display, From, Into};
        use serde::{de, de::Visitor, Deserialize, Deserializer};
        use std::fmt;

        use crate::{endpoint::web_rtc_publish, member, room};

        /// ID of [`WebRtcPublishEndpoint`].
        #[derive(
            Clone, Debug, Deserialize, Display, Eq, From, Hash, Into, PartialEq,
        )]
        pub struct Id(pub String);

        /// Media element which is able to play media data for client via
        /// WebRTC.
        #[derive(Clone, Deserialize, Debug)]
        pub struct WebRtcPlay {
            pub id: Id,

            /// Source URI in format
            /// `local://{room_id}/{member_id}/{endpoint_id}`.
            pub src: SrcUri,

            /// Option to relay all media through a TURN server forcibly.
            #[serde(default)]
            pub force_relay: bool,
        }

        /// Special URI with pattern
        /// `local://{room_id}/{member_id}/{endpoint_id}`.
        /// This uri can pointing only to [`WebRtcPublishEndpoint`].
        ///
        /// Note that [`SrcUri`] is parsing with [`LocalUri`] parser.
        /// Actually difference between [`SrcUri`] and [`LocalUri`]
        /// in endpoint ID's type. In [`SrcUri`] it [`WebRtcPublishId`], and in
        /// [`LocalUri`] it [`EndpointId`]. Also [`SrcUri`] can be deserialized
        /// with [`serde`].
        ///
        /// Atm used only in [Control API] specs.
        ///
        /// [`WebRtcPublishEndpoint`]:
        /// crate::api::control::endpoints::WebRtcPublishEndpoint
        /// [Control API]: https://tinyurl.com/yxsqplq7
        /// [`EndpointId`]: crate::api::control::EndpointId
        #[derive(Clone, Debug)]
        pub struct SrcUri {
            /// ID of [`Room`].
            ///
            /// [`Room`]: crate::signalling::room::Room
            pub room_id: room::Id,

            /// ID of [`MemberSpec`].
            ///
            /// [`MemberSpec`]: crate::api::control::member::MemberSpec
            pub member_id: member::Id,

            /// ID of [`WebRtcPublishEndpoint`].
            ///
            /// [`WebRtcPublishEndpoint`]:
            /// crate::api::control::endpoints::WebRtcPublishEndpoint
            pub endpoint_id: web_rtc_publish::Id,
        }

        /// [Serde] deserializer for [`SrcUri`].
        ///
        /// Deserializes URIs with pattern:
        /// `local://room_id/member_id/publish_endpoint_id`.
        ///
        /// [Serde]: serde
        impl<'de> Deserialize<'de> for SrcUri {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                /// [`Visitor`] implementation for [`Deserialize`]ing
                /// [`SrcUri`].
                struct SrcUriVisitor;

                impl<'de> Visitor<'de> for SrcUriVisitor {
                    type Value = SrcUri;

                    fn expecting(
                        &self,
                        f: &mut fmt::Formatter<'_>,
                    ) -> fmt::Result {
                        f.write_str(
                            "URI in format: \
                             local://room_id/member_id/endpoint_id",
                        )
                    }

                    fn visit_str<E>(self, value: &str) -> Result<SrcUri, E>
                    where
                        E: de::Error,
                    {
                        match SrcUri::try_from(value.to_owned()) {
                            Ok(src_uri) => Ok(src_uri),
                            Err(e) => Err(de::Error::custom(e)),
                        }
                    }
                }

                deserializer.deserialize_identifier(SrcUriVisitor)
            }
        }

        impl fmt::Display for SrcUri {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "local://{}/{}/{}",
                    self.room_id, self.member_id, self.endpoint_id
                )
            }
        }
    }
}

/// Medea's Control API error response.
#[derive(Debug)]
pub struct ErrorResponse {
    /// [`ErrorCode`] which will be returned with code and message.
    error_code: ErrorCode,

    /// Element ID where some error happened. May be empty.
    element_id: Option<String>,

    /// All [`ErrorCode`]s have [`Display`] implementation. And this
    /// implementation will be used if this field is [`None`]. But
    /// sometimes we want to add some error explanation. Then we set this
    /// field to [`Some`] and this text will be added to
    /// [`Display`] implementation's text.
    ///
    /// By default this field should be [`None`].
    ///
    /// For providing error explanation use [`ErrorResponse::with_explanation`]
    /// method.
    ///
    /// [`Display`]: std::fmt::Display
    explanation: Option<String>,
}

impl ErrorResponse {
    /// New [`ErrorResponse`] with [`ErrorCode`] and element ID.
    pub fn new<T: ToString>(error_code: ErrorCode, element_id: &T) -> Self {
        Self {
            error_code,
            element_id: Some(element_id.to_string()),
            explanation: None,
        }
    }

    /// New [`ErrorResponse`] only with [`ErrorCode`].
    #[must_use]
    pub const fn without_id(error_code: ErrorCode) -> Self {
        Self {
            error_code,
            element_id: None,
            explanation: None,
        }
    }

    /// [`ErrorResponse`] for all unexpected errors.
    ///
    /// Provide unexpected `Error` to this function.
    /// This error will be printed with [`Display`] implementation
    /// of provided `Error` as error explanation.
    ///
    /// [`Display`]: std::fmt::Display
    pub fn unexpected<B: ToString>(unknown_error: &B) -> Self {
        Self {
            error_code: ErrorCode::UnexpectedError,
            explanation: Some(unknown_error.to_string()),
            element_id: None,
        }
    }

    /// [`ErrorResponse`] with some additional info.
    ///
    /// With this method you can add additional text to error message of
    /// [`ErrorCode`].
    #[must_use]
    pub const fn with_explanation(
        error_code: ErrorCode,
        explanation: String,
        id: Option<String>,
    ) -> Self {
        Self {
            error_code,
            explanation: Some(explanation),
            element_id: id,
        }
    }
}

/// [Medea]'s [Control API] errors.
///
/// [Medea]: https://git.instrumentisto.com/streaming/medea
/// [Control API]: https://tinyurl.com/yxsqplq7
#[derive(Clone, Copy, Debug, Display)]
#[repr(u16)]
pub enum ErrorCode {
    /// Unimplemented API call.
    ///
    /// This code should be with additional text which explains what
    /// exactly unimplemented (you can do it with
    /// [`ErrorResponse::with_explanation`] function).
    ///
    /// Code: __1000__.
    #[display(fmt = "Unimplemented API call.")]
    UnimplementedCall = 1000,

    /// Request doesn't contain any elements.
    ///
    /// Code: __1001__.
    #[display(fmt = "Request doesn't contain any elements")]
    NoElement = 1001,

    /// Provided fid can't point to provided element.
    ///
    /// Code: __1002__.
    #[display(fmt = "Provided fid can't point to provided element")]
    ElementIdMismatch = 1002,

    /// Room not found.
    ///
    /// Code: __1003__.
    #[display(fmt = "Room not found.")]
    RoomNotFound = 1003,

    /// Member not found.
    ///
    /// Code: __1004__.
    #[display(fmt = "Member not found.")]
    MemberNotFound = 1004,

    /// Endpoint not found.
    ///
    /// Code: __1005__.
    #[display(fmt = "Endpoint not found.")]
    EndpointNotFound = 1005,

    /// Medea expects `Room` element in pipeline but received not him.
    ///
    /// Code: __1006__.
    #[display(fmt = "Expecting Room element but it's not.")]
    NotRoomInSpec = 1006,

    /// Medea expects `Member` element in pipeline but received not him.
    ///
    /// Code: __1007__.
    #[display(fmt = "Expected Member element but it's not.")]
    NotMemberInSpec = 1007,

    /// Invalid source URI in [`WebRtcPlayEndpoint`].
    ///
    /// Code: __1008__.
    ///
    /// [`WebRtcPlayEndpoint`]:
    /// crate::signalling::elements::endpoints::webrtc::WebRtcPlayEndpoint
    #[display(fmt = "Invalid source URI in 'WebRtcPlayEndpoint'.")]
    InvalidSrcUri = 1008,

    /// Provided not source URI in [`WebRtcPlayEndpoint`].
    ///
    /// Code: __1009__.
    ///
    /// [`WebRtcPlayEndpoint`]:
    /// crate::signalling::elements::endpoints::webrtc::WebRtcPlayEndpoint
    #[display(fmt = "Provided not source URI in 'WebRtcPlayEndpoint'.")]
    NotSourceUri = 1009,

    /// Element's URI don't have `local://` prefix.
    ///
    /// Code: __1010__.
    #[display(fmt = "Element's URI don't have 'local://' prefix.")]
    ElementIdIsNotLocal = 1010,

    /// Provided element's FID/URI with too many paths.
    ///
    /// Code: __1011__.
    #[display(fmt = "You provided element's FID/URI with too many paths.")]
    ElementIdIsTooLong = 1011,

    /// Missing some fields in source URI of WebRtcPublishEndpoint.
    ///
    /// Code: __1012__.
    #[display(
        fmt = "Missing some fields in source URI of WebRtcPublishEndpoint."
    )]
    MissingFieldsInSrcUri = 1012,

    /// Empty element ID.
    ///
    /// Code: __1013__.
    #[display(fmt = "Provided empty element ID.")]
    EmptyElementId = 1013,

    /// Provided empty elements FIDs list.
    ///
    /// Code: __1014__.
    #[display(fmt = "Provided empty elements FIDs list.")]
    EmptyElementsList = 1014,

    /// Provided not the same Room IDs in elements IDs. Probably you try use
    /// `Delete` method for elements with different Room IDs
    ///
    /// Code: __1015__.
    ///
    /// [`RoomId`]: crate::api::control::room::Id
    #[display(fmt = "Provided not the same Room IDs in elements IDs. \
                     Probably you try use 'Delete' method for elements with \
                     different Room IDs")]
    ProvidedNotSameRoomIds = 1015,

    /// Room with provided fid already exists.
    ///
    /// Code: __1016__.
    #[display(fmt = "Room with provided FID already exists.")]
    RoomAlreadyExists = 1016,

    /// Member with provided FID already exists.
    ///
    /// Code: __1017__.
    #[display(fmt = "Member with provided FID already exists.")]
    MemberAlreadyExists = 1017,

    /// Endpoint with provided FID already exists.
    ///
    /// Code: __1018__.
    #[display(fmt = "Endpoint with provided FID already exists.")]
    EndpointAlreadyExists = 1018,

    /// Missing path in some reference to the Medea element.
    ///
    /// Code: __1019__.
    #[display(fmt = "Missing path in some reference to the Medea element.")]
    MissingPath = 1019,

    /// Missing host in callback URL.
    ///
    /// Code: __1020__.
    #[display(fmt = "Missing host in callback URL.")]
    MissingHostInCallbackUrl = 1020,

    /// Unsupported callback URL protocol.
    ///
    /// Code: __1021__.
    #[display(fmt = "Unsupported callback URL protocol.")]
    UnsupportedCallbackUrlProtocol = 1021,

    /// Invalid callback URL.
    ///
    /// Code: __1022__.
    #[display(fmt = "Invalid callback URL.")]
    InvalidCallbackUrl = 1022,

    /// Encountered negative duration.
    ///
    /// Code: __1023__.
    #[display(fmt = "Encountered negative duration")]
    NegativeDuration = 1023,

    /// Unexpected server error.
    ///
    /// Use this [`ErrorCode`] only with [`ErrorResponse::unexpected`]
    /// function. In error text with this code should be error message
    /// which explain what exactly goes wrong
    /// ([`ErrorResponse::unexpected`] do this).
    ///
    /// Code: __2000__.
    #[display(fmt = "Unexpected error happened.")]
    UnexpectedError = 2000,
}

impl From<ErrorCode> for u16 {
    #[allow(clippy::as_conversions)]
    fn from(code: ErrorCode) -> Self {
        code as Self
    }
}

impl From<ErrorCode> for u32 {
    fn from(code: ErrorCode) -> Self {
        u16::from(code).into()
    }
}
