#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
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
    clippy::default_union_representation,
    clippy::else_if_without_else,
    clippy::empty_drop,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::equatable_if_let,
    clippy::exit,
    clippy::expect_used,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::index_refutable_slice,
    clippy::iter_with_drain,
    clippy::large_include_file,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::only_used_in_recursion,
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
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::try_err,
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
    unused_tuple_struct_fields,
    variant_size_differences
)]
// TODO: Remove once annoying false positive is fixed:
//       https://github.com/rust-lang/rust-clippy/issues/6902
#![allow(clippy::use_self)]

pub mod state;
pub mod stats;

use std::collections::HashMap;

use derive_more::{Constructor, Display, From};
use medea_macro::dispatchable;
use serde::{Deserialize, Serialize};

use self::stats::RtcStat;

/// ID of a `Room`.
#[derive(
    Clone, Debug, Display, Serialize, Deserialize, Eq, From, Hash, PartialEq,
)]
#[from(forward)]
pub struct RoomId(pub String);

/// ID of a `Member`.
#[derive(
    Clone, Debug, Display, Serialize, Deserialize, Eq, From, Hash, PartialEq,
)]
#[from(forward)]
pub struct MemberId(pub String);

/// ID of a `Peer`.
#[cfg_attr(feature = "server", derive(Default))]
#[derive(
    Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize,
)]
pub struct PeerId(pub u32);

/// ID of a `MediaTrack`.
#[cfg_attr(feature = "server", derive(Default))]
#[derive(
    Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize,
)]
pub struct TrackId(pub u32);

/// Credential used for a `Member` authentication.
#[derive(
    Clone, Debug, Deserialize, Display, Eq, From, Hash, PartialEq, Serialize,
)]
#[from(forward)]
pub struct Credential(pub String);

#[cfg(feature = "server")]
/// Value that is able to be incremented by `1`.
pub trait Incrementable {
    /// Returns current value + 1.
    #[must_use]
    fn incr(&self) -> Self;
}

#[cfg(feature = "server")]
/// Implements [`Incrementable`] trait for a newtype with any numeric type.
macro_rules! impl_incrementable {
    ($name:ty) => {
        impl Incrementable for $name {
            fn incr(&self) -> Self {
                Self(self.0 + 1)
            }
        }
    };
}

#[cfg(feature = "server")]
impl_incrementable!(PeerId);
#[cfg(feature = "server")]
impl_incrementable!(TrackId);

#[allow(variant_size_differences)]
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
#[serde(tag = "msg", content = "data")]
/// Message sent by Media Server to Web Client.
pub enum ServerMsg {
    /// `ping` message that Media Server is expected to send to Web Client
    /// periodically for probing its aliveness.
    Ping(u32),

    /// Media Server notifies Web Client about happened facts and it reacts on
    /// them to reach the proper state.
    Event {
        /// ID of the `Room` that this [`Event`] is associated with.
        room_id: RoomId,

        /// Actual [`Event`] sent to Web Client.
        event: Event,
    },

    /// Media Server notifies Web Client about necessity to update its RPC
    /// settings.
    RpcSettings(RpcSettings),
}

#[allow(variant_size_differences)]
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "server", derive(Deserialize))]
#[derive(Clone, Debug, PartialEq)]
/// Message by Web Client to Media Server.
pub enum ClientMsg {
    /// `pong` message that Web Client answers with to Media Server in response
    /// to received [`ServerMsg::Ping`].
    Pong(u32),

    /// Request of Web Client to change its state on Media Server.
    Command {
        /// ID of the `Room` that this [`Command`] is associated with.
        room_id: RoomId,

        /// Actual [`Command`] sent to Media Server.
        command: Command,
    },
}

/// RPC settings of Web Client received from Media Server.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RpcSettings {
    /// Timeout of considering Web Client as lost by Media Server when it
    /// doesn't receive any [`ClientMsg::Pong`]s.
    ///
    /// Unit: millisecond.
    pub idle_timeout_ms: u32,

    /// Interval that Media Server sends [`ServerMsg::Ping`]s with.
    ///
    /// Unit: millisecond.
    pub ping_interval_ms: u32,
}

/// Possible commands sent by Web Client to Media Server.
#[dispatchable]
#[allow(unused_results)] // false positive: on `Deserialize`
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "server", derive(Deserialize))]
#[derive(Clone, Debug, PartialEq)]
#[serde(tag = "command", content = "data")]
pub enum Command {
    /// Request to join a `Room`.
    JoinRoom {
        /// ID of the `Member` who joins the `Room`.
        member_id: MemberId,

        /// [`Credential`] of the `Member` to authenticate with.
        credential: Credential,
    },

    /// Request to leave a `Room`.
    LeaveRoom {
        /// ID of the `Member` who leaves the `Room`.
        member_id: MemberId,
    },

    /// Web Client sends SDP Offer.
    MakeSdpOffer {
        /// ID of the `Peer` SDP Offer is sent for.
        peer_id: PeerId,

        /// SDP Offer of the `Peer`.
        sdp_offer: String,

        /// Associations between [`Track`] and transceiver's
        /// [media description][1].
        ///
        /// `mid` is basically an ID of [`m=<media>` section][1] in SDP.
        ///
        /// [1]: https://tools.ietf.org/html/rfc4566#section-5.14
        mids: HashMap<TrackId, String>,

        /// Statuses of the `Peer` transceivers.
        transceivers_statuses: HashMap<TrackId, bool>,
    },

    /// Web Client sends SDP Answer.
    MakeSdpAnswer {
        /// ID of the `Peer` SDP Answer is sent for.
        peer_id: PeerId,

        /// SDP Answer of the `Peer`.
        sdp_answer: String,

        /// Statuses of the `Peer` transceivers.
        transceivers_statuses: HashMap<TrackId, bool>,
    },

    /// Web Client sends an Ice Candidate.
    SetIceCandidate {
        /// ID of the `Peer` the Ice Candidate is sent for.
        peer_id: PeerId,

        /// [`IceCandidate`] sent by the `Peer`.
        candidate: IceCandidate,
    },

    /// Web Client sends Peer Connection metrics.
    AddPeerConnectionMetrics {
        /// ID of the `Peer` metrics are sent for.
        peer_id: PeerId,

        /// Metrics of the `Peer`.
        metrics: PeerMetrics,
    },

    /// Web Client asks permission to update [`Track`]s in the specified
    /// `Peer`. Media Server gives permission by sending
    /// [`Event::PeerUpdated`].
    UpdateTracks {
        /// ID of the `Peer` to update [`Track`]s in.
        peer_id: PeerId,

        /// Patches for updating the [`Track`]s.
        tracks_patches: Vec<TrackPatchCommand>,
    },

    /// Web Client asks Media Server to synchronize Client State with a
    /// Server State.
    SynchronizeMe {
        /// Whole Client State of the `Room`.
        state: state::Room,
    },
}

/// Web Client's `PeerConnection` metrics.
#[allow(variant_size_differences)]
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "server", derive(Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum PeerMetrics {
    /// `PeerConnection`'s ICE connection state.
    IceConnectionState(IceConnectionState),

    /// `PeerConnection`'s connection state.
    PeerConnectionState(PeerConnectionState),

    /// `PeerConnection`'s RTC stats.
    RtcStats(Vec<RtcStat>),
}

/// `PeerConnection`'s ICE connection state.
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "server", derive(Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IceConnectionState {
    /// ICE agent is gathering addresses or is waiting to be given remote
    /// candidates.
    New,

    /// ICE agent has been given one or more remote candidates and is checking
    /// pairs of local and remote candidates against one another to try to find
    /// a compatible match, but hasn't yet found a pair which will allow the
    /// `PeerConnection` to be made. It's possible that gathering of candidates
    /// is also still underway.
    Checking,

    /// Usable pairing of local and remote candidates has been found for all
    /// components of the connection, and the connection has been established.
    /// It's possible that gathering is still underway, and it's also possible
    /// that the ICE agent is still checking candidates against one another
    /// looking for a better connection to use.
    Connected,

    /// ICE agent has finished gathering candidates, has checked all pairs
    /// against one another, and has found a connection for all components.
    Completed,

    /// ICE candidate has checked all candidates pairs against one another and
    /// has failed to find compatible matches for all components of the
    /// connection. It is, however, possible that the ICE agent did find
    /// compatible connections for some components.
    Failed,

    /// Checks to ensure that components are still connected failed for at
    /// least one component of the `PeerConnection`. This is a less stringent
    /// test than [`IceConnectionState::Failed`] and may trigger intermittently
    /// and resolve just as spontaneously on less reliable networks, or during
    /// temporary disconnections. When the problem resolves, the connection may
    /// return to the [`IceConnectionState::Connected`] state.
    Disconnected,

    /// ICE agent for this `PeerConnection` has shut down and is no longer
    /// handling requests.
    Closed,
}

/// `PeerConnection`'s connection state.
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "server", derive(Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PeerConnectionState {
    /// At least one of the connection's ICE transports are in the
    /// [`IceConnectionState::New`] state, and none of them are in one
    /// of the following states: [`IceConnectionState::Checking`],
    /// [`IceConnectionState::Failed`], or
    /// [`IceConnectionState::Disconnected`], or all of the connection's
    /// transports are in the [`IceConnectionState::Closed`] state.
    New,

    /// One or more of the ICE transports are currently in the process of
    /// establishing a connection; that is, their [`IceConnectionState`] is
    /// either [`IceConnectionState::Checking`] or
    /// [`IceConnectionState::Connected`], and no transports are in the
    /// [`IceConnectionState::Failed`] state.
    Connecting,

    /// Every ICE transport used by the connection is either in use (state
    /// [`IceConnectionState::Connected`] or [`IceConnectionState::Completed`])
    /// or is closed ([`IceConnectionState::Closed`]); in addition,
    /// at least one transport is either [`IceConnectionState::Connected`] or
    /// [`IceConnectionState::Completed`].
    Connected,

    /// At least one of the ICE transports for the connection is in the
    /// [`IceConnectionState::Disconnected`] state and none of the other
    /// transports are in the state [`IceConnectionState::Failed`] or
    /// [`IceConnectionState::Checking`].
    Disconnected,

    /// One or more of the ICE transports on the connection is in the
    /// [`IceConnectionState::Failed`] state.
    Failed,

    /// The `PeerConnection` is closed.
    Closed,
}

impl From<IceConnectionState> for PeerConnectionState {
    fn from(ice_con_state: IceConnectionState) -> Self {
        use IceConnectionState as Ice;

        match ice_con_state {
            Ice::New => Self::New,
            Ice::Checking => Self::Connecting,
            Ice::Connected | Ice::Completed => Self::Connected,
            Ice::Failed => Self::Failed,
            Ice::Disconnected => Self::Disconnected,
            Ice::Closed => Self::Closed,
        }
    }
}

/// Reason of disconnecting Web Client from Media Server.
#[derive(
    Copy, Clone, Debug, Deserialize, Display, Eq, PartialEq, Serialize,
)]
pub enum CloseReason {
    /// Client session was finished on a server side.
    Finished,

    /// Old connection was closed due to a client reconnection.
    Reconnected,

    /// Connection has been inactive for a while and thus considered idle
    /// by a server.
    Idle,

    /// Establishing of connection with a server was rejected on server side.
    ///
    /// Most likely because of incorrect `Member` credentials.
    Rejected,

    /// Server internal error has occurred while connecting.
    ///
    /// This close reason is similar to 500 HTTP status code.
    InternalError,

    /// Client was evicted on the server side.
    Evicted,
}

/// Description which is sent in [Close] WebSocket frame from Media Server to
/// Web Client.
///
/// [Close]: https://tools.ietf.org/html/rfc6455#section-5.5.1
#[derive(
    Clone, Constructor, Copy, Debug, Deserialize, Eq, PartialEq, Serialize,
)]
pub struct CloseDescription {
    /// Reason of why WebSocket connection has been closed.
    pub reason: CloseReason,
}

/// Possible WebSocket messages sent from Media Server to Web Client.
#[dispatchable(self: & Self, async_trait(? Send))]
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
#[serde(tag = "event", content = "data")]
pub enum Event {
    /// Media Server notifies Web Client that a `Member` joined a `Room`.
    RoomJoined {
        /// ID of the `Member` who joined the `Room`.
        member_id: MemberId,
    },

    /// Media Server notifies Web Client that a `Member` left a `Room`.
    RoomLeft {
        /// [`CloseReason`] with which the `Member` left the `Room`.
        close_reason: CloseReason,
    },

    /// Media Server notifies Web Client about necessity of RTCPeerConnection
    /// creation.
    PeerCreated {
        /// ID of the `Peer` to create RTCPeerConnection for.
        peer_id: PeerId,

        /// [`NegotiationRole`] of the `Peer`.
        negotiation_role: NegotiationRole,

        /// [`Track`]s to create RTCPeerConnection with.
        tracks: Vec<Track>,

        /// [`IceServer`]s to create RTCPeerConnection with.
        ice_servers: Vec<IceServer>,

        /// Indicator whether the created RTCPeerConnection should be forced to
        /// use relay [`IceServer`]s only.
        force_relay: bool,
    },

    /// Media Server notifies Web Client about necessity to apply the specified
    /// SDP Answer to Web Client's RTCPeerConnection.
    SdpAnswerMade {
        /// ID of the `Peer` to apply SDP Answer to.
        peer_id: PeerId,

        /// SDP Answer to be applied.
        sdp_answer: String,
    },

    /// Media Server notifies Web Client that his SDP offer was applied.
    LocalDescriptionApplied {
        /// ID of the `Peer` which SDP offer was applied.
        peer_id: PeerId,

        /// SDP offer that was applied.
        sdp_offer: String,
    },

    /// Media Server notifies Web Client about necessity to apply the specified
    /// ICE Candidate.
    IceCandidateDiscovered {
        /// ID of the `Peer` to apply ICE Candidate to.
        peer_id: PeerId,

        /// ICE Candidate to be applied.
        candidate: IceCandidate,
    },

    /// Media Server notifies Web Client about necessity of RTCPeerConnection
    /// close.
    PeersRemoved {
        /// IDs of `Peer`s to be removed.
        peer_ids: Vec<PeerId>,
    },

    /// Media Server notifies about necessity to update [`Track`]s in a `Peer`.
    PeerUpdated {
        /// ID of the `Peer` to update [`Track`]s in.
        peer_id: PeerId,

        /// List of [`PeerUpdate`]s which should be applied.
        updates: Vec<PeerUpdate>,

        /// Negotiation role basing on which should be sent
        /// [`Command::MakeSdpOffer`] or [`Command::MakeSdpAnswer`].
        ///
        /// If [`None`] then no (re)negotiation should be done.
        negotiation_role: Option<NegotiationRole>,
    },

    /// Media Server notifies about connection quality score update.
    ConnectionQualityUpdated {
        /// Partner [`MemberId`] of the `Peer`.
        partner_member_id: MemberId,

        /// Estimated connection quality.
        quality_score: ConnectionQualityScore,
    },

    /// Media Server synchronizes Web Client state and reports the proper one.
    StateSynchronized {
        /// Proper state that should be assumed by Web Client.
        state: state::Room,
    },
}

/// `Peer`'s negotiation role.
///
/// Some [`Event`]s can trigger SDP negotiation:
/// - If [`Event`] contains [`NegotiationRole::Offerer`], then `Peer` is
///   expected to create SDP Offer and send it via [`Command::MakeSdpOffer`].
/// - If [`Event`] contains [`NegotiationRole::Answerer`], then `Peer` is
///   expected to apply provided SDP Offer and provide its SDP Answer in a
///   [`Command::MakeSdpAnswer`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum NegotiationRole {
    /// [`Command::MakeSdpOffer`] should be sent by client.
    Offerer,

    /// [`Command::MakeSdpAnswer`] should be sent by client.
    Answerer(String),
}

/// [`Track`] update which should be applied to the `Peer`.
#[allow(variant_size_differences)]
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PeerUpdate {
    /// New [`Track`] should be added to the `Peer`.
    Added(Track),

    /// [`Track`] with the provided [`TrackId`] should be removed from the
    /// `Peer`.
    ///
    /// Can only refer [`Track`]s already known to the `Peer`.
    Removed(TrackId),

    /// [`Track`] should be updated by this [`TrackPatchEvent`] in the `Peer`.
    /// Can only refer tracks already known to the `Peer`.
    Updated(TrackPatchEvent),

    /// `Peer` should start ICE restart process on the next renegotiation.
    IceRestart,
}

/// Representation of [RTCIceCandidateInit][1] object.
///
/// [1]: https://w3.org/TR/webrtc/#dom-rtcicecandidateinit
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IceCandidate {
    /// [`candidate-attribute`][0] of this [`IceCandidate`].
    ///
    /// If this [`IceCandidate`] represents an end-of-candidates indication,
    /// then it's an empty string.
    ///
    /// [0]: https://w3.org/TR/webrtc/#dfn-candidate-attribute
    pub candidate: String,

    /// Index (starting at zero) of the media description in the SDP this
    /// [`IceCandidate`] is associated with.
    pub sdp_m_line_index: Option<u16>,

    /// [Media stream "identification-tag"] for the media component this
    /// [`IceCandidate`] is associated with.
    ///
    /// [0]: https://w3.org/TR/webrtc/#dfn-media-stream-identification-tag
    pub sdp_mid: Option<String>,
}

/// Track with a [`Direction`].
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Track {
    /// ID of this [`Track`].
    pub id: TrackId,

    /// [`Direction`] of this [`Track`].
    pub direction: Direction,

    /// [`MediaType`] of this [`Track`].
    pub media_type: MediaType,
}

impl Track {
    /// Indicates whether this [`Track`] is required to call starting.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.media_type.required()
    }
}

/// Patch of a [`Track`] which Web Client can request with a
/// [`Command::UpdateTracks`].
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "server", derive(Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrackPatchCommand {
    /// ID of the [`Track`] this patch is intended for.
    pub id: TrackId,

    /// [`Track`]'s media exchange state.
    pub enabled: Option<bool>,

    /// [`Track`]'s mute state.
    ///
    /// Muting and unmuting can be performed without adding/removing tracks
    /// from transceivers, hence renegotiation is not required.
    pub muted: Option<bool>,
}

/// Patch of a [`Track`] which Media Server can send with an
/// [`Event::PeerUpdated`].
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrackPatchEvent {
    /// ID of the [`Track`] which should be patched.
    pub id: TrackId,

    /// General media exchange direction of the `Track`.
    pub media_direction: Option<MediaDirection>,

    /// [`Track`]'s mute state.
    ///
    /// Muting and unmuting can be performed without adding/removing tracks
    /// from transceivers, hence renegotiation is not required.
    pub muted: Option<bool>,
}

/// Media exchange direction of a `Track`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MediaDirection {
    /// `Track` is enabled on both receiver and sender sides.
    SendRecv = 0,

    /// `Track` is enabled on sender side only.
    SendOnly = 1,

    /// `Track` is enabled on receiver side only.
    RecvOnly = 2,

    /// `Track` is disabled on both sides.
    Inactive = 3,
}

impl MediaDirection {
    /// Indicates whether a `Track` is enabled on sender side only.
    #[must_use]
    pub const fn is_send_enabled(self) -> bool {
        matches!(self, Self::SendRecv | Self::SendOnly)
    }

    /// Indicates whether a `Track` is enabled on receiver side only.
    #[must_use]
    pub const fn is_recv_enabled(self) -> bool {
        matches!(self, Self::SendRecv | Self::RecvOnly)
    }

    /// Indicates whether a `Track` is enabled on both sender and receiver
    /// sides.
    #[must_use]
    pub const fn is_enabled_general(self) -> bool {
        matches!(self, Self::SendRecv)
    }
}

impl From<TrackPatchCommand> for TrackPatchEvent {
    fn from(from: TrackPatchCommand) -> Self {
        Self {
            id: from.id,
            muted: from.muted,
            media_direction: from.enabled.map(|enabled| {
                if enabled {
                    MediaDirection::SendRecv
                } else {
                    MediaDirection::Inactive
                }
            }),
        }
    }
}

impl TrackPatchEvent {
    /// Returns a new empty [`TrackPatchEvent`] with the provided [`TrackId`].
    #[must_use]
    pub const fn new(id: TrackId) -> Self {
        Self {
            id,
            muted: None,
            media_direction: None,
        }
    }

    /// Merges this [`TrackPatchEvent`] with the provided one.
    ///
    /// Does nothing if [`TrackId`] of this [`TrackPatchEvent`] and the
    /// provided [`TrackPatchEvent`] are different.
    pub fn merge(&mut self, another: &Self) {
        if self.id != another.id {
            return;
        }

        if let Some(muted) = another.muted {
            self.muted = Some(muted);
        }

        if let Some(direction) = another.media_direction {
            self.media_direction = Some(direction);
        }
    }
}

/// Representation of [RTCIceServer][1] (item of `iceServers` field
/// from [RTCConfiguration][2]).
///
/// [1]: https://developer.mozilla.org/en-US/docs/Web/API/RTCIceServer
/// [2]: https://developer.mozilla.org/en-US/docs/Web/API/RTCConfiguration
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IceServer {
    /// URLs of this [`IceServer`].
    pub urls: Vec<String>,

    /// Optional username to authenticate on this [`IceServer`] with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Optional secret to authenticate on this [`IceServer`] with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
}

/// Possible directions of a [`Track`].
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
// TODO: Use different struct without mids in PeerUpdated event.
pub enum Direction {
    /// Outgoing direction.
    Send {
        /// IDs of the `Member`s who should receive this outgoing [`Track`].
        receivers: Vec<MemberId>,

        /// [Media stream "identification-tag"] of this outgoing [`Track`].
        ///
        /// [0]: https://w3.org/TR/webrtc/#dfn-media-stream-identification-tag
        mid: Option<String>,
    },

    /// Incoming direction.
    Recv {
        /// IDs of the `Member` this incoming [`Track`] is received from.
        sender: MemberId,

        /// [Media stream "identification-tag"] of this incoming [`Track`].
        ///
        /// [0]: https://w3.org/TR/webrtc/#dfn-media-stream-identification-tag
        mid: Option<String>,
    },
}

/// Possible media types of a [`Track`].
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MediaType {
    /// Audio [`Track`].
    Audio(AudioSettings),

    /// Video [`Track`].
    Video(VideoSettings),
}

impl MediaType {
    /// Indicates whether this [`MediaType`] is required to call starting.
    #[must_use]
    pub const fn required(&self) -> bool {
        match self {
            Self::Audio(audio) => audio.required,
            Self::Video(video) => video.required,
        }
    }
}

/// Settings of an audio [`Track`].
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AudioSettings {
    /// Importance of the audio.
    ///
    /// If `false` then audio may be not published.
    pub required: bool,
}

/// Settings of a video [`Track`].
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VideoSettings {
    /// Importance of the video.
    ///
    /// If `false` then video may be not published.
    pub required: bool,

    /// Source kind of this [`VideoSettings`] media.
    pub source_kind: MediaSourceKind,
}

/// Possible media sources of a video [`Track`].
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MediaSourceKind {
    /// Media is sourced by some media device (webcam or microphone).
    Device,

    /// Media is obtained with screen-capture.
    Display,
}

/// Estimated connection quality.
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[derive(Clone, Copy, Debug, Display, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConnectionQualityScore {
    /// Nearly all users dissatisfied.
    Poor = 1,

    /// Many users dissatisfied.
    Low = 2,

    /// Some users dissatisfied.
    Medium = 3,

    /// Satisfied.
    High = 4,
}
