//! Platform-agnostic functionality of [`platform::RtcPeerConnection`].

use derive_more::with_trait::{Display, From};
#[cfg(doc)]
use platform::Transceiver;

use crate::{
    platform::{self, RtcStatsError},
    utils::Caused,
};

/// Representation of [RTCSdpType].
///
/// [RTCSdpType]: https://w3.org/TR/webrtc#dom-rtcsdptype
#[derive(Debug)]
pub enum SdpType {
    /// [`offer` type][1] of SDP.
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcsdptype-offer
    Offer(String),

    /// [`answer` type][1] of SDP.
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcsdptype-answer
    Answer(String),
}

/// [RTCIceCandidate][1] representation.
///
/// [1]: https://w3.org/TR/webrtc#rtcicecandidate-interface
#[derive(Debug)]
pub struct IceCandidate {
    /// [`candidate` field][2] of the discovered [RTCIceCandidate][1].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcicecandidate
    /// [2]: https://w3.org/TR/webrtc#dom-rtcicecandidate-candidate
    pub candidate: String,

    /// [`sdpMLineIndex` field][2] of the discovered [RTCIceCandidate][1].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcicecandidate
    /// [2]: https://w3.org/TR/webrtc#dom-rtcicecandidate-sdpmlineindex
    pub sdp_m_line_index: Option<u16>,

    /// [`sdpMid` field][2] of the discovered [RTCIceCandidate][1].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcicecandidate
    /// [2]: https://w3.org/TR/webrtc#dom-rtcicecandidate-sdpmid
    pub sdp_mid: Option<String>,
}

/// Error occurred with an [ICE] candidate from a `PeerConnection`.
///
/// [ICE]: https://webrtcglossary.com/ice
#[derive(Debug)]
pub struct IceCandidateError {
    /// Local IP address used to communicate with a [STUN]/[TURN] server.
    ///
    /// [STUN]: https://webrtcglossary.com/stun
    /// [TURN]: https://webrtcglossary.com/turn
    pub address: Option<String>,

    /// Port used to communicate with a [STUN]/[TURN] server.
    ///
    /// [STUN]: https://webrtcglossary.com/stun
    /// [TURN]: https://webrtcglossary.com/turn
    pub port: Option<u32>,

    /// URL identifying the [STUN]/[TURN] server for which the failure
    /// occurred.
    ///
    /// [STUN]: https://webrtcglossary.com/stun
    /// [TURN]: https://webrtcglossary.com/turn
    pub url: String,

    /// Numeric [STUN] error code returned by the [STUN]/[TURN] server.
    ///
    /// If no host candidate can reach the server, this error code will be set
    /// to the value `701`, which is outside the [STUN] error code range. This
    /// error is only fired once per server URL while in the
    /// `RTCIceGatheringState` of "gathering".
    ///
    /// [STUN]: https://webrtcglossary.com/stun
    /// [TURN]: https://webrtcglossary.com/turn
    pub error_code: i32,

    /// [STUN] reason text returned by the [STUN]/[TURN] server.
    ///
    /// If the server could not be reached, this reason test will be set to an
    /// implementation-specific value providing details about the error.
    ///
    /// [STUN]: https://webrtcglossary.com/stun
    /// [TURN]: https://webrtcglossary.com/turn
    pub error_text: String,
}

/// Errors that may occur during signaling between this and remote
/// [RTCPeerConnection][1] and event handlers setting errors.
///
/// [1]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
#[derive(Caused, Clone, Debug, Display, From)]
#[cause(error = platform::Error)]
pub enum RtcPeerConnectionError {
    /// Occurs when cannot adds new remote candidate to the
    /// [RTCPeerConnection][1]'s remote description.
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    #[display("Failed to add ICE candidate: {_0}")]
    #[from(ignore)]
    AddIceCandidateFailed(platform::Error),

    /// Occurs when cannot obtains [SDP answer][`SdpType::Answer`] from
    /// the underlying [`platform::RtcPeerConnection`].
    #[display("Failed to create SDP answer: {_0}")]
    #[from(ignore)]
    CreateAnswerFailed(platform::Error),

    /// Occurs when a new [`platform::RtcPeerConnection`] cannot be created.
    #[display("Failed to create PeerConnection: {_0}")]
    #[from(ignore)]
    PeerCreationError(platform::Error),

    /// Occurs when cannot obtains [SDP offer][`SdpType::Offer`] from
    /// the underlying [`platform::RtcPeerConnection`].
    #[display("Failed to create SDP offer: {_0}")]
    #[from(ignore)]
    CreateOfferFailed(platform::Error),

    /// Occurs while getting and parsing [`platform::RtcStats`] of
    /// [`platform::RtcPeerConnection`].
    #[display("Failed to get RTCStats: {_0}")]
    RtcStatsError(#[cause] RtcStatsError),

    /// [PeerConnection.getStats][1] promise thrown exception.
    ///
    /// [1]: https://tinyurl.com/w6hmt5f
    #[display("PeerConnection.getStats() failed with error: {_0}")]
    #[from(ignore)]
    GetStatsException(platform::Error),

    /// Occurs if the local description associated with the
    /// [`platform::RtcPeerConnection`] cannot be changed.
    #[display("Failed to set local SDP description: {_0}")]
    #[from(ignore)]
    SetLocalDescriptionFailed(platform::Error),

    /// Occurs if the description of the remote end of the
    /// [`platform::RtcPeerConnection`] cannot be changed.
    #[display("Failed to set remote SDP description: {_0}")]
    #[from(ignore)]
    SetRemoteDescriptionFailed(platform::Error),

    /// [`Transceiver::update_send_encodings`] error.
    #[display("Failed to update sender encodings: {_0}")]
    #[from(ignore)]
    UpdateSendEncodingsError(platform::transceiver::UpdateSendEncodingError),
}
