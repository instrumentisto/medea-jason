//! Platform-agnostic functionality of [`platform::RtcStats`].

use std::rc::Rc;

use derive_more::{Display, From};
use medea_client_api_proto::stats::{
    KnownCandidateType, KnownIceCandidatePairState, KnownProtocol,
};

use crate::{platform, utils::Caused};

/// Errors which can occur during deserialization of a [`RtcStatsType`].
///
/// [`RtcStatsType`]: medea_client_api_proto::stats::RtcStatsType
#[derive(Caused, Clone, Debug, Display, From)]
#[cause(error = "platform::Error")]
pub enum RtcStatsError {
    /// [RTCStats.id][1] is undefined.
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-rtcstats-id
    #[display(fmt = "RTCStats.id is undefined")]
    UndefinedId,

    /// [RTCStats.stats] are undefined.
    ///
    /// [1]: https://w3.org/TR/webrtc-stats/#dfn-stats-object
    #[display(fmt = "RTCStats.stats are undefined")]
    UndefinedStats,

    /// Some platform error occurred.
    #[display(fmt = "Unexpected platform error: {}", _0)]
    Platform(platform::Error),

    /// `RTCStats.entries` are undefined.
    #[display(fmt = "RTCStats.entries are undefined")]
    UndefinedEntries,

    /// [`platform::RtcStats`] deserialization error.
    #[display(fmt = "Failed to deserialize into RtcStats: {}", _0)]
    ParseError(Rc<serde_json::Error>),
}

/// Known protocols used in the WebRTC.
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
#[repr(u8)]
pub enum Protocol {
    /// [Transmission Control Protocol][1].
    ///
    /// [1]: https://en.wikipedia.org/wiki/Transmission_Control_Protocol
    #[display(fmt = "TCP")]
    Tcp = 0,
    /// [User Datagram Protocol][1].
    ///
    /// [1]: https://en.wikipedia.org/wiki/User_Datagram_Protocol
    #[display(fmt = "UDP")]
    Udp = 1,
}

/// Variants of [ICE roles][1].
///
/// More info in the [RFC 5245].
///
/// [RFC 5245]: https://tools.ietf.org/html/rfc5245
/// [1]: https://w3.org/TR/webrtc#dom-icetransport-role
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
#[repr(u8)]
pub enum IceRole {
    /// Agent whose role as defined by [Section 3 in RFC 5245][1], has not yet
    /// been determined.
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-3
    #[display(fmt = "unknown")]
    Unknown = 0,

    /// Controlling agent as defined by [Section 3 in RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-3
    #[display(fmt = "controlling")]
    Controlling = 1,

    /// Controlled agent as defined by [Section 3 in RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-3
    #[display(fmt = "controlled")]
    Controlled = 2,
}

/// [RTCIceCandidateType] represents the type of the ICE candidate, as
/// defined in [Section 15.1 of RFC 5245][1].
///
/// [RTCIceCandidateType]: https://w3.org/TR/webrtc#rtcicecandidatetype-enum
/// [1]: https://tools.ietf.org/html/rfc5245#section-15.1
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
#[repr(u8)]
pub enum CandidateType {
    /// Host candidate, as defined in [Section 4.1.1.1 of RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-4.1.1.1
    #[display(fmt = "host")]
    Host = 0,
    /// Server reflexive candidate, as defined in
    /// [Section 4.1.1.2 of RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-4.1.1.2
    #[display(fmt = "srlfx")]
    Srlfx = 1,
    /// Peer reflexive candidate, as defined in
    /// [Section 4.1.1.2 of RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-4.1.1.2
    #[display(fmt = "prflx")]
    Prflx = 2,
    /// Relay candidate, as defined in [Section 7.1.3.2.1 of RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-7.1.3.2.1
    #[display(fmt = "relay")]
    Relay = 3,
}

impl From<IceRole> for medea_client_api_proto::stats::IceRole {
    fn from(role: IceRole) -> Self {
        match role {
            IceRole::Unknown => Self::Unknown,
            IceRole::Controlling => Self::Controlling,
            IceRole::Controlled => Self::Controlled,
        }
    }
}

impl From<CandidateType> for KnownCandidateType {
    fn from(kind: CandidateType) -> Self {
        match kind {
            CandidateType::Host => Self::Host,
            CandidateType::Srlfx => Self::Srlfx,
            CandidateType::Prflx => Self::Prflx,
            CandidateType::Relay => Self::Relay,
        }
    }
}

impl From<Protocol> for KnownProtocol {
    fn from(protocol: Protocol) -> Self {
        match protocol {
            Protocol::Tcp => Self::Tcp,
            Protocol::Udp => Self::Udp,
        }
    }
}

/// Representation of [`RTCStatsIceCandidatePairState`][1].
///
/// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstatsicecandidatepairstate
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
#[repr(u8)]
pub enum RTCStatsIceCandidatePairState {
    /// Check for this pair hasn't been performed, and it can't yet be
    /// performed until some other check succeeds, allowing this pair to
    /// unfreeze and move into the [`KnownIceCandidatePairState::Waiting`]
    /// state.
    Frozen = 0,
    /// Check has not been performed for this pair, and can be performed as
    /// soon as it is the highest-priority Waiting pair on the check list.
    Waiting = 1,
    /// Check has been sent for this pair, but the transaction is in progress.
    InProgress = 2,
    /// Check for this pair was already done and failed, either never producing
    /// any response or producing an unrecoverable failure response.
    Failed = 3,
    /// Check for this pair was already done and produced a successful result.
    Succeeded = 4,
}

impl From<RTCStatsIceCandidatePairState> for KnownIceCandidatePairState {
    fn from(state: RTCStatsIceCandidatePairState) -> Self {
        match state {
            RTCStatsIceCandidatePairState::Frozen => Self::Frozen,
            RTCStatsIceCandidatePairState::Waiting => Self::Waiting,
            RTCStatsIceCandidatePairState::InProgress => Self::InProgress,
            RTCStatsIceCandidatePairState::Failed => Self::Failed,
            RTCStatsIceCandidatePairState::Succeeded => Self::Succeeded,
        }
    }
}
