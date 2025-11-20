//! Contains DTOs for [RTCPeerConnection] metrics according to the
//! [Identifiers for WebRTC's Statistics API][0]
//!
//! [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
//! [0]: https://w3.org/TR/webrtc-stats

// TODO: Needs refactoring.
#![expect(clippy::module_name_repetitions, reason = "needs refactoring")]

use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
    time::{Duration, SystemTime, SystemTimeError},
};

use derive_more::with_trait::{Display, From};
use serde::{Deserialize, Serialize};

/// Enum with which you can try to deserialize some known enum and if it
/// isn't known, then unknown data will be stored as [`String`] in the
/// [`NonExhaustive::Unknown`] variant.
#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[serde(untagged)]
pub enum NonExhaustive<T> {
    /// Will store known enum variant if it successfully deserialized.
    Known(T),

    /// Will store unknown enum variant with it's data as [`String`].
    #[display("Unknown: {}", _0)]
    Unknown(String),
}

/// Unique ID that is associated with the object that was inspected to produce
/// [`RtcStat`] object.
///
/// Two [`RtcStat`]s objects, extracted from two different [RTCStatsReport]
/// objects, MUST have the same ID if they were produced by inspecting the same
/// underlying object.
///
/// [RTCStatsReport]: https://w3.org/TR/webrtc#dom-rtcstatsreport
#[derive(
    Clone, Debug, Deserialize, Display, Eq, From, Hash, PartialEq, Serialize,
)]
#[from(forward)]
pub struct StatId(pub String);

/// Represents the [stats object] constructed by inspecting a specific
/// [monitored object].
///
/// [Full doc on W3C][1].
///
/// [stats object]: https://w3.org/TR/webrtc-stats#dfn-stats-object
/// [monitored object]: https://w3.org/TR/webrtc-stats#dfn-monitored-object
/// [1]: https://w3.org/TR/webrtc#rtcstats-dictionary
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct RtcStat {
    /// Unique ID that is associated with the object that was inspected to
    /// produce this [RTCStats] object.
    ///
    /// [RTCStats]: https://w3.org/TR/webrtc#dom-rtcstats
    pub id: StatId,

    /// Timestamp associated with this object.
    ///
    /// The time is relative to the UNIX epoch (Jan 1, 1970, UTC).
    ///
    /// For statistics that came from a remote source (e.g., from received RTCP
    /// packets), timestamp represents the time at which the information
    /// arrived at the local endpoint. The remote timestamp can be found in an
    /// additional field in an [`RtcStat`]-derived dictionary, if applicable.
    pub timestamp: HighResTimeStamp,

    /// Actual stats of this [`RtcStat`].
    ///
    /// All possible stats are described in the [`RtcStatsType`] enum.
    #[serde(flatten)]
    pub stats: RtcStatsType,
}

/// All known types of [`RtcStat`]s.
///
/// [List of all RTCStats types on W3C][1].
///
/// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcstatstype
/// [`RtcStat`]: super::RtcStat
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum RtcStatsType {
    /// Statistics for a codec that is currently used by [RTP stream]s
    /// being sent or received by [RTCPeerConnection] object.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection
    Codec(Box<RtcCodecStats>),

    /// Statistics for an inbound [RTP stream] that is currently received
    /// with this [RTCPeerConnection] object.
    ///
    /// RTX streams do not show up as separate [`RtcInboundRtpStreamStats`]
    /// objects but affect the [`RtcReceivedRtpStreamStats::packets_received`],
    /// [`RtcInboundRtpStreamStats::bytes_received`],
    /// [`RtcInboundRtpStreamStats::retransmitted_packets_received`] and
    /// [`RtcInboundRtpStreamStats::retransmitted_bytes_received`] counters of
    /// the relevant [`RtcInboundRtpStreamStats`] objects.
    ///
    /// FEC streams do not show up as separate [`RtcInboundRtpStreamStats`]
    /// objects but affect the [`RtcReceivedRtpStreamStats::packets_received`],
    /// [`RtcInboundRtpStreamStats::bytes_received`],
    /// [`RtcInboundRtpStreamStats::fec_packets_received`] and
    /// [`RtcInboundRtpStreamStats::fec_bytes_received`] counters of the
    /// relevant [`RtcInboundRtpStreamStats`] objects.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    InboundRtp(Box<RtcInboundRtpStreamStats>),

    /// Statistics for an outbound [RTP stream] that is currently sent with
    /// this [RTCPeerConnection] object.
    ///
    /// When there are multiple [RTP stream]s connected to the same sender due
    /// to using simulcast, there will be one [`RtcOutboundRtpStreamStats`]
    /// per [RTP stream], with distinct values of the [SSRC] member.
    /// [RTX stream]s do not show up as separate [`RtcOutboundRtpStreamStats`]
    /// objects but affect the [`RtcSentRtpStreamStats::packets_sent`],
    /// [`RtcSentRtpStreamStats::bytes_sent`],
    /// [`RtcOutboundRtpStreamStats::retransmitted_packets_sent`] and
    /// [`RtcOutboundRtpStreamStats::retransmitted_bytes_sent`] counters of the
    /// relevant [`RtcOutboundRtpStreamStats`] objects.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    OutboundRtp(Box<RtcOutboundRtpStreamStats>),

    /// Statistics for the remote endpoint's inbound [RTP stream] corresponding
    /// to an outbound stream that is currently sent with this
    /// [RTCPeerConnection] object. It is measured at the remote endpoint and
    /// reported in an [RTCP Receiver Report][1] (RR) or
    /// [RTCP Extended Report][2] (XR).
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    /// [1]: https://w3.org/TR/webrtc-stats/#dfn-receiver-report
    /// [2]: https://w3.org/TR/webrtc-stats/#dfn-extended-report
    RemoteInboundRtp(Box<RtcRemoteInboundRtpStreamStats>),

    /// Statistics for the remote endpoint's outbound [RTP stream] corresponding
    /// to an inbound stream that is currently received with this
    /// [RTCPeerConnection] object. It is measured at the remote endpoint and
    /// reported in an [RTCP Sender Report][1] (SR).
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    /// [1]: https://w3.org/TR/webrtc-stats/#dfn-sender-report
    RemoteOutboundRtp(Box<RtcRemoteOutboundRtpStreamStats>),

    /// Statistics for the media produced by a [MediaStreamTrack][1] that is
    /// currently attached to an [RTCRtpSender]. This reflects the media that
    /// is fed to the encoder; after [getUserMedia()][2] constraints have been
    /// applied (i.e. not the raw media produced by the camera).
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
    /// [RTCRtpSender]: https://w3.org/TR/webrtc/#dom-rtcrtpsender
    /// [2]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    MediaSource(Box<RtcMediaSourceStats>),

    /// Statistics related to audio playout.
    MediaPlayout(Box<RtcAudioPlayoutStats>),

    /// Statistics related to the [RTCPeerConnection] object.
    ///
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    PeerConnection(Box<RtcPeerConnectionStats>),

    /// Statistics related to each [RTCDataChannel] ID.
    ///
    /// [RTCDataChannel]: https://w3.org/TR/webrtc#dom-rtcdatachannel
    DataChannel(Box<RtcDataChannelStats>),

    /// Transport statistics related to the [RTCPeerConnection] object. It is
    /// accessed by the [`RtcTransportStats`].
    ///
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    Transport(Box<RtcTransportStats>),

    /// ICE candidate pair statistics related to the [RTCIceTransport]
    /// objects.
    ///
    /// A candidate pair that is not the current pair for a transport is
    /// [deleted][1] when the [RTCIceTransport] does an ICE restart, at the
    /// time the state changes to `new`. The candidate pair that is the current
    /// pair for a transport is deleted after an ICE restart when the
    /// [RTCIceTransport] switches to using a candidate pair generated from
    /// the new candidates; this time doesn't correspond to any other
    /// externally observable event.
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
    /// [1]: https://w3.org/TR/webrtc-stats#dfn-deleted
    CandidatePair(Box<RtcIceCandidatePairStats>),

    /// ICE local candidate statistics related to the [RTCIceTransport]
    /// objects.
    ///
    /// A local candidate is [deleted][1] when the [RTCIceTransport] does
    /// an ICE restart, and the candidate is no longer a member of
    /// any non-deleted candidate pair.
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
    /// [1]: https://w3.org/TR/webrtc-stats#dfn-deleted
    LocalCandidate(Box<RtcIceCandidateStats>),

    /// ICE remote candidate statistics related to the [RTCIceTransport]
    /// objects.
    ///
    /// A remote candidate is [deleted][1] when the [RTCIceTransport] does
    /// an ICE restart, and the candidate is no longer a member of
    /// any non-deleted candidate pair.
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
    /// [1]: https://w3.org/TR/webrtc-stats#dfn-deleted
    RemoteCandidate(Box<RtcIceCandidateStats>),

    /// Information about a certificate used by [RTCIceTransport]. It is
    /// accessed by the [`RtcCertificateStats`].
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
    Certificate(Box<RtcCertificateStats>),

    /// Disabled or unknown variants of stats will be deserialized as
    /// [`RtcStatsType::Other`].
    #[serde(other)]
    Other,
}

/// Stats that apply to any end of any [RTP stream].
///
/// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RtcRtpStreamStats {
    /// The synchronization source (SSRC) identifier is an unsigned integer
    /// value per [RFC3550] used to identify the stream of RTP packets that
    /// this stats object is describing.
    ///
    /// For outbound and inbound local, SSRC describes the stats for the
    /// RTP stream that were sent and received, respectively by those
    /// endpoints. For the remote inbound and remote outbound, SSRC describes
    /// the stats for the RTP stream that were received by and sent to the
    /// remote endpoint.
    ///
    /// [RFC3550]: https://rfc-editor.org/rfc/rfc3550
    pub ssrc: u32,

    /// Either "audio" or "video". This MUST match the kind attribute of the
    /// related [MediaStreamTrack][0].
    ///
    /// [0]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
    pub kind: String, // TODO: add enum?

    /// It is a unique identifier that is associated to the object that was
    /// inspected to produce the [`RtcTransportStats`] associated with this
    /// [RTP stream].
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    pub transport_id: Option<String>,

    /// It is a unique identifier that is associated to the object that
    /// was inspected to produce the [`RtcCodecStats`] associated with
    /// this [RTP stream].
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    pub codec_id: Option<String>,
}

/// Stats measured at the receiving end of an RTP stream, known either
/// because they're measured locally or transmitted via an RTCP
/// [Receiver Report] (RR) or [Extended Report] (XR) block.
///
/// [Receiver Report]: https://w3.org/TR/webrtc-stats/#dfn-receiver-report
/// [Extended Report]: https://w3.org/TR/webrtc-stats/#dfn-extended-report
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RtcReceivedRtpStreamStats {
    /// Generic [RTP stream] data.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    #[serde(flatten)]
    pub stream: RtcRtpStreamStats,

    /// Total number of RTP packets received for this [SSRC]. This includes
    /// retransmissions. At the receiving endpoint, this is calculated as
    /// defined in [RFC3550] section 6.4.1. At the sending endpoint the
    /// `packets_received` is estimated by subtracting the Cumulative Number
    /// of Packets Lost from the Extended Highest Sequence Number
    /// Received, both reported in the [RTCP Receiver Report][0], and then
    /// subtracting the initial Extended Sequence Number that was sent to
    /// this SSRC in a [RTCP Sender Report] and then adding one, to mirror what
    /// is discussed in Appendix A.3 in [RFC3550], but for the sender side.
    /// If no [RTCP Receiver Report][0] has been received yet, then return 0.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [RFC3550]: https://rfc-editor.org/rfc/rfc3550
    /// [0]: https://w3.org/TR/webrtc-stats/#dfn-receiver-report
    /// [RTCP Sender Report]: https://w3.org/TR/webrtc-stats/#dfn-sender-report
    pub packets_received: Option<u64>,

    /// Total number of RTP packets received for this [SSRC] marked with the
    /// "ECT(1)" marking.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub packets_received_with_ect1: Option<u64>,

    /// Total number of RTP packets received for this [SSRC] marked with the
    /// "CE" marking.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub packets_received_with_ce: Option<u64>,

    /// Total number of RTP packets for which an [RFC8888] section 3.1 report
    /// has been sent with a zero R bit. Only exists if support for the "ccfb"
    /// feedback mechanism has been negotiated.
    ///
    /// [RFC8888]: https://rfc-editor.org/rfc/rfc8888
    pub packets_reported_as_lost: Option<u64>,

    /// Total number of RTP packets for which an [RFC8888] section 3.1 report
    /// has been sent with a zero R bit, but a later report for the same packet
    /// has the R bit set to 1. Only exists if support for the "ccfb" feedback
    /// mechanism has been negotiated.
    ///
    /// [RFC8888]: https://rfc-editor.org/rfc/rfc8888
    pub packets_reported_as_lost_but_recovered: Option<u64>,

    /// Total number of RTP packets lost for this [SSRC]. Calculated as defined
    /// in [RFC3550] section 6.4.1. Note that because of how this is estimated,
    /// it can be negative if more packets are received than sent.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [RFC3550]: https://rfc-editor.org/rfc/rfc3550
    pub packets_lost: Option<i64>,

    /// Packet Jitter measured in seconds for this [SSRC]. Calculated as defined
    /// in section 6.4.1. of [RFC3550].
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [RFC3550]: https://rfc-editor.org/rfc/rfc3550
    pub jitter: Option<Double>,
}

/// Stats measured at the sending end of an [RTP stream], known either because
/// they're measured locally or because they're received via RTCP, usually
/// in an [RTCP Sender Report] (SR).
///
/// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
/// [RTCP Sender Report]: https://w3.org/TR/webrtc-stats/#dfn-sender-report
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct RtcSentRtpStreamStats {
    /// Generic [RTP stream] data.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    #[serde(flatten)]
    pub stream: RtcRtpStreamStats,

    /// Total number of RTP packets sent for this [SSRC]. This includes
    /// retransmissions. Calculated as defined in [RFC3550] section 6.4.1.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [RFC3550]: https://rfc-editor.org/rfc/rfc3550
    pub packets_sent: Option<u64>,

    /// Total number of bytes sent for this [SSRC]. This includes
    /// retransmissions. Calculated as defined in [RFC3550] section 6.4.1.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [RFC3550]: https://rfc-editor.org/rfc/rfc3550
    pub bytes_sent: Option<u64>,
}

/// Codecs are created when registered for an RTP transport, but only the
/// subset of codecs that are in use (referenced by an RTP stream) are
/// exposed in [getStats()].
///
/// The [`RtcCodecStats`] object is created when one or more
/// [`RtcRtpStreamStats::codec_id`] references the codec. When there no longer
/// exists any reference to the [`RtcCodecStats`], the stats object is deleted.
/// If the same codec is used again in the future, the [`RtcCodecStats`]
/// object is revived with the same [`StatId`] as before.
///
/// Codec objects may be referenced by multiple RTP streams in media sections
/// using the same transport, but similar codecs in different transports have
/// different [`RtcCodecStats`] objects.
///
/// [getStats()]: https://tinyurl.com/webrtc-rfc-get-stats
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcCodecStats {
    /// Payload type as used in RTP encoding or decoding.
    pub payload_type: u32,

    /// The unique identifier of the transport on which this codec is being
    /// used, which can be used to look up the corresponding
    /// [`RtcTransportStats`] object.
    pub transport_id: String,

    /// The codec MIME media type/subtype defined in the IANA media types
    /// registry [IANA-MEDIA-TYPES][0], e.g. `video/VP8`.
    ///
    /// [0]: https://iana.org/assignments/media-types/media-types.xhtml
    pub mime_type: String,

    /// Represents the media sampling rate.
    pub clock_rate: Option<u32>,

    /// When present, indicates the number of channels (mono=1, stereo=2).
    pub channels: Option<u32>,

    /// The "format specific parameters" field from the `a=fmtp` line in the
    /// SDP corresponding to the codec, if one exists, as defined by
    /// [RFC8829].
    ///
    /// [RFC8829]: https://rfc-editor.org/rfc/rfc8829#section-5.8
    pub sdp_fmtp_line: Option<String>,
}

/// Represents the measurement metrics for the incoming RTP media stream. The
/// timestamp reported in the statistics object is the time at which the data
/// was sampled.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcInboundRtpStreamStats {
    /// Generic ingress [RTP stream] data.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    #[serde(flatten)]
    pub received_stream: RtcReceivedRtpStreamStats,

    /// Media kind specific part of [`RtcInboundRtpStreamStats`].
    #[serde(flatten)]
    pub media_specific: InboundRtpMediaType,

    /// The value of the [MediaStreamTrack][0]'s `id` attribute.
    ///
    /// [0]: https://w3.org/TR/mediacapture-streams/#mediastreamtrack
    pub track_identifier: Option<String>,

    /// If the [RTCRtpTransceiver][0] owning this stream has a `mid` value
    /// that is not `null`, this is that value, otherwise this member MUST
    /// NOT be present.
    ///
    /// [0]: https://w3.org/TR/webrtc/#rtcrtptransceiver-interface
    pub mid: Option<String>,

    /// Used for looking up the remote [`RtcRemoteOutboundRtpStreamStats`]
    /// object for the same [SSRC].
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub remote_id: Option<String>,

    /// Total number of bytes received for this [SSRC]. This includes
    /// retransmissions. Calculated as defined in [RFC3550] section 6.4.1.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [RFC3550]: https://rfc-editor.org/rfc/rfc3550
    pub bytes_received: Option<u64>,

    /// Total number of audio samples or video frames that have come out of the
    /// jitter buffer (increasing
    /// [`RtcInboundRtpStreamStats::jitter_buffer_delay`]).
    pub jitter_buffer_emitted_count: Option<u64>,

    /// The purpose of the jitter buffer is to recombine RTP packets into frames
    /// (in the case of video) and have smooth playout. The model described here
    /// assumes that the samples or frames are still compressed and have not yet
    /// been decoded. It is the sum of the time, in seconds, each [audio sample]
    /// or a video frame takes from the time the first packet is received by the
    /// jitter buffer (ingest timestamp) to the time it exits the jitter buffer
    /// (emit timestamp). In the case of audio, several samples belong to the
    /// same RTP packet, hence they will have the same ingest timestamp but
    /// different jitter buffer emit timestamps. In the case of video, the frame
    /// may be received over several RTP packets, hence the ingest timestamp is
    /// the earliest packet of the frame that entered the jitter buffer and the
    /// emit timestamp is when the whole frame exits the jitter buffer. This
    /// metric increases upon samples or frames exiting, having completed their
    /// time in the buffer (and incrementing
    /// [`RtcInboundRtpStreamStats::jitter_buffer_emitted_count`]). The average
    /// jitter buffer delay can be calculated by dividing the
    /// [`RtcInboundRtpStreamStats::jitter_buffer_delay`] with the
    /// [`RtcInboundRtpStreamStats::jitter_buffer_emitted_count`].
    ///
    /// [audio sample]: https://w3.org/TR/webrtc-stats/#dfn-audio-sample
    pub jitter_buffer_delay: Option<Double>,

    /// This value is increased by the target jitter buffer delay every time a
    /// sample is emitted by the jitter buffer. The added target is the target
    /// delay, in seconds, at the time that the sample was emitted from the
    /// jitter buffer. To get the average target delay, divide by
    /// [`RtcInboundRtpStreamStats::jitter_buffer_emitted_count`].
    pub jitter_buffer_target_delay: Option<Double>,

    /// There are various reasons why the jitter buffer delay might be increased
    /// to a higher value, such as to achieve AV synchronization or because a
    /// [jitterBufferTarget][0] was set on an [RTCRtpReceiver]. When using
    /// one of these mechanisms, it can be useful to keep track of the minimal
    /// jitter buffer delay that could have been achieved, so WebRTC clients
    /// can track the amount of additional delay that is being added.
    //
    /// This metric works the same way as
    /// [`RtcInboundRtpStreamStats::jitter_buffer_target_delay`], except that
    /// it is not affected by external mechanisms that increase the jitter
    /// buffer target delay, such as [jitterBufferTarget][0], AV sync, or any
    /// other mechanisms. This metric is purely based on the network
    /// characteristics such as jitter and packet loss, and can be seen as the
    /// minimum obtainable jitter buffer delay if no external factors would
    /// affect it. The metric is updated every time
    /// [`RtcInboundRtpStreamStats::jitter_buffer_emitted_count`] is updated.
    ///
    /// [0]: https://w3.org/TR/webrtc/#dom-rtcrtpreceiver-jitterbuffertarget
    /// [RTCRtpReceiver]: https://w3.org/TR/webrtc/#rtcrtpreceiver-interface
    pub jitter_buffer_minimum_delay: Option<Double>,

    /// Total number of RTP header and padding bytes received for this [SSRC].
    /// This includes retransmissions. Does not include transport headers
    /// (IP/UDP). [`RtcInboundRtpStreamStats::header_bytes_received`] +
    /// [`RtcInboundRtpStreamStats::bytes_received`] equals the total number of
    /// bytes received as payload over the transport.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub header_bytes_received: Option<u64>,

    /// The cumulative number of RTP packets discarded by the jitter buffer
    /// due to late or early-arrival, i.e., these packets are not played out.
    /// RTP packets discarded due to packet duplication are not reported in
    /// this metric [XRBLOCK-STATS]. Calculated as defined in [RFC7002]
    /// section 3.2 and Appendix A.a.
    ///
    /// [XRBLOCK-STATS]: https://tinyurl.com/xr-report
    /// [RFC7002]: https://rfc-editor.org/rfc/rfc7002
    pub packets_discarded: Option<u64>,

    /// The timestamp at which the last RTP packet was received for this [SSRC].
    ///
    /// Differs from `timestamp`, which represents when the statistics were
    /// generated or received at the local endpoint.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub last_packet_received_timestamp: Option<HighResTimeStamp>,

    /// Estimated playout time of this receiver's track in sender NTP time.
    ///
    /// Can be used to estimate A/V sync across tracks from the same source.
    pub estimated_playout_timestamp: Option<HighResTimeStamp>,

    /// Total number of RTP FEC bytes received for this [SSRC], only including
    /// payload bytes.
    ///
    /// This is a subset of [`Self::bytes_received`]. If FEC uses a different
    /// [SSRC], packets are still accounted for here.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub fec_bytes_received: Option<u64>,

    /// Total number of RTP FEC packets received for this [SSRC].
    ///
    /// If FEC uses a different [SSRC], packets are still accounted for here.
    /// Can also increment when receiving in-band FEC (for example, Opus).
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub fec_packets_received: Option<u64>,

    /// Total number of RTP FEC packets received for this [SSRC] where the error
    /// correction payload was discarded (for example, sources already recovered
    /// or FEC arrived late).
    ///
    /// This is a subset of [`Self::fec_bytes_received`].
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub fec_packets_discarded: Option<u64>,

    /// It is the sum of the time, in seconds, each [audio sample] or video
    /// frame takes from the time the first RTP packet is received (reception
    /// timestamp) and to the time the corresponding sample or frame is decoded
    /// (decoded timestamp). At this point the audio sample or video frame is
    /// ready for playout by the MediaStreamTrack. Typically ready for playout
    /// here means after the audio sample or video frame is fully decoded by the
    /// decoder.
    ///
    /// [audio sample]: https://w3.org/TR/webrtc-stats/#dfn-audio-sample
    pub total_processing_delay: Option<Double>,

    /// Total number of `Negative ACKnowledgement` (NACK) RTCP feedback packets
    /// sent by this receiver for this [SSRC], as defined in
    /// [RFC4585 section 6.2.1][0].
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [0]: https://rfc-editor.org/rfc/rfc4585#section-6.2.1
    pub nack_count: Option<u32>,

    /// Total number of retransmitted packets that were received for this
    /// [SSRC].
    ///
    /// This is a subset of [`RtcReceivedRtpStreamStats::packets_received`]. If
    /// RTX is not negotiated, retransmitted packets can not be identified
    /// and this member MUST NOT exist.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub retransmitted_packets_received: Option<u64>,

    /// Total number of retransmitted bytes that were received for this [SSRC],
    /// only including payload bytes.
    ///
    /// This is a subset of [`RtcInboundRtpStreamStats::bytes_received`]. If
    /// RTX is not negotiated, retransmitted packets can not be identified and
    /// this member MUST NOT exist.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub retransmitted_bytes_received: Option<u64>,

    /// If RTX is negotiated for retransmissions on a separate [RTP stream],
    /// this is the SSRC of the RTX stream that is associated with this
    /// stream's [`RtcRtpStreamStats::ssrc`].
    ///
    /// If RTX is not negotiated, this value MUST NOT be present.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    pub rtx_ssrc: Option<u32>,

    /// If a FEC mechanism that uses a separate [RTP stream] is negotiated,
    /// this is the SSRC of the FEC stream that is associated with this
    /// stream's [`RtcRtpStreamStats::ssrc`]. If FEC is not negotiated
    /// or uses the same [RTP stream], this value MUST NOT be present.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    pub fec_ssrc: Option<u32>,
}

/// Represents the measurement metrics for the outgoing [RTP stream]. The
/// timestamp reported in the statistics object is the time at which the data
/// was sampled.
///
/// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcOutboundRtpStreamStats {
    /// Generic egress [RTP stream] data.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    #[serde(flatten)]
    pub sent_stream: RtcSentRtpStreamStats,

    /// Media kind specific part of [`RtcOutboundRtpStreamStats`].
    #[serde(flatten)]
    pub media_specific: OutboundRtpMediaType,

    /// If the [RTCRtpTransceiver][0] owning this stream has a [mid] value that
    /// is not null, this is that value, otherwise this member MUST NOT be
    /// present.
    ///
    /// [0]: https://w3.org/TR/webrtc/#rtcrtptransceiver-interface
    /// [mid]: https://w3.org/TR/webrtc/#dom-rtptransceiver-mid
    pub mid: Option<String>,

    /// The identifier of the stats object representing the track currently
    /// attached to the sender of this stream, an [`RtcMediaSourceStats`].
    pub media_source_id: Option<String>,

    /// Is used for looking up the remote [`RtcRemoteInboundRtpStreamStats`]
    /// object for the same [SSRC].
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub remote_id: Option<String>,

    /// Total number of RTP header and padding bytes sent for this [SSRC].
    /// This does not include the size of transport layer headers such as
    /// IP or UDP. [`RtcOutboundRtpStreamStats::header_bytes_sent`] +
    /// [`RtcSentRtpStreamStats::bytes_sent`] equals the number of bytes
    /// sent as payload over the transport.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub header_bytes_sent: Option<u64>,

    /// The total number of packets that were retransmitted for this [SSRC].
    /// This is a subset of [`RtcSentRtpStreamStats::packets_sent`]. If
    /// RTX is not negotiated, retransmitted packets are sent over this
    /// [`RtcRtpStreamStats::ssrc`]. If RTX was negotiated, retransmitted
    /// packets are sent over a separate SSRC but is still accounted for here.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub retransmitted_packets_sent: Option<u64>,

    /// The total number of bytes that were retransmitted for this [SSRC], only
    /// including payload bytes. This is a subset of
    /// [`RtcSentRtpStreamStats::bytes_sent`]. If RTX is not negotiated,
    /// retransmitted bytes are sent over this [`RtcRtpStreamStats::ssrc`]. If
    /// RTX was negotiated, retransmitted bytes are sent over a separate SSRC
    /// but is still accounted for here.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub retransmitted_bytes_sent: Option<u64>,

    /// If RTX is negotiated for retransmissions on a separate [RTP stream],
    /// this is the SSRC of the RTX stream that is associated with this
    /// stream's [`RtcRtpStreamStats::ssrc`]. If RTX is not negotiated, this
    /// value MUST NOT be present.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    pub rtx_ssrc: Option<u32>,

    /// Reflects the current encoder target in bits per second. The target is
    /// an instantaneous value reflecting the encoder's settings, but the
    /// resulting payload bytes sent per second, excluding retransmissions,
    /// SHOULD closely correlate to the target. See also
    /// [`RtcSentRtpStreamStats::bytes_sent`] and
    /// [`RtcOutboundRtpStreamStats::retransmitted_bytes_sent`]. This is
    /// defined in the same way as the (TIAS) bitrate [RFC3890].
    ///
    /// [RFC3890]: https://rfc-editor.org/rfc/rfc3890
    pub target_bitrate: Option<Double>,

    /// The total number of seconds that packets have spent buffered locally
    /// before being transmitted onto the network. The time is measured from
    /// when a packet is emitted from the RTP packetizer until it is handed
    /// over to the OS network socket. This measurement is added to
    /// `total_packet_send_delay` when
    /// [`RtcSentRtpStreamStats::packets_sent`] is incremented.
    pub total_packet_send_delay: Option<Double>,

    /// Count the total number of `Negative ACKnowledgement` (NACK) packets, as
    /// defined in [RFC4585] section 6.2.1, received by this sender.
    ///
    /// [RFC4585]: https://rfc-editor.org/rfc/rfc4585
    pub nack_count: Option<u32>,

    /// Indicates whether this [RTP stream] is configured to be sent or
    /// disabled.
    ///
    /// Note that an active stream can still not be sending, e.g. when being
    /// limited by network conditions.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    pub active: Option<bool>,

    /// Total number of RTP packets sent for this [SSRC] with the `ECT(1)`
    /// marking defined in [RFC3168] section 5 and used by the `L4S` protocol
    /// described in [RFC9331].
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [RFC3168]: https://rfc-editor.org/rfc/rfc3168
    /// [RFC9331]: https://rfc-editor.org/rfc/rfc9331
    pub packets_sent_with_ect1: Option<u64>,
}

/// Represents the remote endpoint's measurement metrics for a particular
/// incoming [RTP stream] (corresponding to an outgoing [RTP stream] at the
/// sending endpoint). The timestamp reported in the statistics object is the
/// time at which the corresponding [RTCP RR] was received.
///
/// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
/// [RTCP RR]: https://w3.org/TR/webrtc-stats/#dfn-receiver-report
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcRemoteInboundRtpStreamStats {
    /// Generic ingress [RTP stream] data.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    #[serde(flatten)]
    pub received_stream: RtcReceivedRtpStreamStats,

    /// Used for looking up the local [`RtcOutboundRtpStreamStats`] object
    /// for the same [SSRC].
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub local_id: Option<String>,

    /// Estimated round trip time for this [SSRC] based on the RTCP timestamps
    /// in the RTCP Receiver Report (RR) and measured in seconds. Calculated
    /// as defined in section 6.4.1. of [RFC3550]. MUST NOT exist until a
    /// [RTCP Receiver Report][0] is received with a DLSR value other than `0`
    /// has been received.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [RFC3550]: https://rfc-editor.org/rfc/rfc3550
    /// [0]: https://w3.org/TR/webrtc-stats/#dfn-receiver-report
    pub round_trip_time: Option<Double>,

    /// Represents the cumulative sum of all round trip time measurements in
    /// seconds since the beginning of the session. The individual round trip
    /// time is calculated based on the RTCP timestamps in the
    /// [RTCP Receiver Report][0] (RR) [RFC3550], hence requires a DLSR value
    /// other than `0`. The average round trip time can be computed from
    /// [`Self::total_round_trip_time`] by dividing it by
    /// [`Self::round_trip_time_measurements`].
    ///
    /// [0]: https://w3.org/TR/webrtc-stats/#dfn-receiver-report
    /// [RFC3550]: https://rfc-editor.org/rfc/rfc3550
    pub total_round_trip_time: Option<Double>,

    /// The fraction packet loss reported for this [SSRC]. Calculated as defined
    /// in [RFC3550] section 6.4.1 and Appendix A.3.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [RFC3550]: https://rfc-editor.org/rfc/rfc3550
    pub fraction_lost: Option<Double>,

    /// Represents the total number of [RTCP RR] blocks received for this
    /// [SSRC] that contain a valid round trip time. This counter will not
    /// increment if the [`Self::round_trip_time`] can not be calculated
    /// because no [RTCP Receiver Report][0] with a DLSR value other than
    /// `0` has been received.
    ///
    /// [RTCP RR]: https://w3.org/TR/webrtc-stats/#dfn-receiver-report
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [0]: https://w3.org/TR/webrtc-stats/#dfn-receiver-report
    pub round_trip_time_measurements: Option<Double>,

    /// Number of packets that were sent with ECT(1) markings per [RFC3168]
    /// section 3, but where an [RFC8888] report gave information that the
    /// packet was received with a marking of "not-ECT".
    ///
    /// [RFC3168]: https://rfc-editor.org/rfc/rfc3168
    /// [RFC8888]: https://rfc-editor.org/rfc/rfc8888
    pub packets_with_bleached_ect1marking: Option<u64>,
}

/// Represents the remote endpoint's measurement metrics for its outgoing
/// [RTP stream] (corresponding to an outgoing [RTP stream] at the sending
/// endpoint). The timestamp reported in the statistics object is the time
/// at which the corresponding [RTCP SR] was received.
///
/// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
/// [RTCP SR]: https://w3.org/TR/webrtc-stats/#dfn-sender-report
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcRemoteOutboundRtpStreamStats {
    /// Generic egress [RTP stream] data.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    #[serde(flatten)]
    pub sent_stream: RtcSentRtpStreamStats,

    /// Used for looking up the local [`RtcInboundRtpStreamStats`] object
    /// for the same [SSRC].
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub local_id: Option<String>,

    /// Represents the remote timestamp at which these statistics were sent
    /// by the remote endpoint. This differs from [`RtcStat::timestamp`], which
    /// represents the time at which the statistics were generated or received
    /// by the local endpoint. The remote timestamp, if present, is derived
    /// from the NTP timestamp in an [RTCP Sender Report] (SR) block, which
    /// reflects the remote endpoint's clock. That clock may not be
    /// synchronized with the local clock.
    ///
    /// [RTCP Sender Report]: https://w3.org/TR/webrtc-stats/#dfn-sender-report
    pub remote_timestamp: Option<HighResTimeStamp>,

    /// Represents the total number of [RTCP Sender Report] (SR) blocks sent
    /// for this [SSRC].
    ///
    /// [RTCP Sender Report]: https://w3.org/TR/webrtc-stats/#dfn-sender-report
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    pub reports_sent: Option<u64>,

    /// Estimated round trip time for this [SSRC] based on the latest
    /// [RTCP Sender Report] (SR) that contains a DLRR report block as defined
    /// in [RFC3611]. The Calculation of the round trip time is defined in
    /// section 4.5. of [RFC3611]. MUST NOT exist if the latest SR does not
    /// contain the DLRR report block, or if the last RR timestamp in the
    /// DLRR report block is zero, or if the delay since last RR value in
    /// the DLRR report block is zero.
    ///
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [RTCP Sender Report]: https://w3.org/TR/webrtc-stats/#dfn-sender-report
    /// [RFC3611]: https://rfc-editor.org/rfc/rfc3611
    pub round_trip_time: Option<Double>,

    /// Represents the cumulative sum of all round trip time measurements in
    /// seconds since the beginning of the session. The individual round trip
    /// time is calculated based on the DLRR report block in the
    /// [RTCP Sender Report] (SR) [RFC3611]. This counter will not increment if
    /// the [`Self::round_trip_time`] can not be calculated. The average round
    /// trip time can be computed from [`Self::total_round_trip_time`] by
    /// dividing it by [`Self::round_trip_time_measurements`].
    ///
    /// [RTCP Sender Report]: https://w3.org/TR/webrtc-stats/#dfn-sender-report
    /// [RFC3611]: https://rfc-editor.org/rfc/rfc3611
    pub total_round_trip_time: Option<Double>,

    /// Represents the total number of [RTCP Sender Report] (SR) blocks
    /// received for this [SSRC] that contain a DLRR report block that can
    /// derive a valid round trip time according to [RFC3611]. This counter
    /// will not increment if the [`Self::round_trip_time`] can not be
    /// calculated.
    ///
    /// [RTCP Sender Report]: https://w3.org/TR/webrtc-stats/#dfn-sender-report
    /// [SSRC]: https://w3.org/TR/webrtc-stats/#dfn-ssrc
    /// [RFC3611]: https://rfc-editor.org/rfc/rfc3611
    pub round_trip_time_measurements: Option<u64>,
}

/// Represents a track that is currently attached to one or more senders.
/// It contains information about media sources such as frame rate and
/// resolution prior to encoding. This is the media passed from the
/// [MediaStreamTrack][0] to the [RTCRtpSender]s. This is in contrast to
/// [`RtcOutboundRtpStreamStats`] whose members describe metrics as measured
/// after the encoding step. For example, a track may be captured from
/// a high-resolution camera, its frames downscaled due to track constraints
/// and then further downscaled by the encoders due to CPU and network
/// conditions. This dictionary reflects the video frames or [audio sample]s
/// passed out from the track - after track constraints have been applied
/// but before any encoding or further downsampling occurs.
///
/// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
/// [RTCRtpSender]: https://w3.org/TR/webrtc/#dom-rtcrtpsender
/// [audio sample]: https://w3.org/TR/webrtc-stats/#dfn-audio-sample
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcMediaSourceStats {
    /// Value of the [MediaStreamTrack][1]'s ID attribute.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    pub track_identifier: Option<String>,

    /// Fields which should be in the [`RtcStat`] based on `kind`.
    #[serde(flatten)]
    pub kind: MediaSourceKind,
}

/// Represents one playout path - if the same playout stats object is
/// referenced by multiple [`RtcInboundRtpStreamStats`] this is an
/// indication that audio mixing is happening in which case sample counters
/// in this stats object refer to the samples after mixing.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Hash, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RtcAudioPlayoutStats {
    /// For audio playout, this has the value `audio`. This reflects the kind
    /// attribute [MediaStreamTrack][0](s) being played out.
    ///
    /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    pub kind: Option<String>,

    /// If the playout path is unable to produce audio samples on time for
    /// device playout, samples are synthesized to be playout out instead.
    /// [`Self::synthesized_samples_duration`] is measured in seconds and is
    /// incremented each time an audio sample is synthesized by this playout
    /// path. This metric can be used together with
    /// [`Self::total_samples_duration`] to calculate the percentage of played
    /// out media being synthesized.
    ///
    /// Synthesization typically only happens if the pipeline is
    /// underperforming. Samples synthesized by the
    /// [`RtcInboundRtpStreamStats`] are not counted for here, but in
    /// [`InboundRtpMediaType::Audio::concealed_samples`].
    pub synthesized_samples_duration: Option<Double>,

    /// The number of synthesized samples events. This counter increases
    /// every time a sample is synthesized after a non-synthesized sample.
    /// That is, multiple consecutive synthesized samples will increase
    /// [`Self::synthesized_samples_duration`] multiple times but is a single
    /// synthesization samples event.
    pub synthesized_samples_events: Option<u32>,

    /// The total duration, in seconds, of all audio samples that have been
    /// playout. Includes both synthesized and non-synthesized samples.
    pub total_samples_duration: Option<Double>,

    /// When audio samples are pulled by the playout device, this counter is
    /// incremented with the estimated delay of the playout path for that
    /// audio sample. The playout delay includes the delay from being emitted
    /// to the actual time of playout on the device. This metric can be used
    /// together with [`Self::total_samples_count`] to calculate the average
    /// playout delay per sample.
    pub total_playout_delay: Option<Double>,

    /// When audio samples are pulled by the playout device, this counter is
    /// incremented with the number of samples emitted for playout.
    pub total_samples_count: Option<u64>,
}

/// Stats for the [RTCPeerConnection] object.
///
/// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcPeerConnectionStats {
    /// Number of unique `DataChannel`s that have entered the `open` state
    /// during their lifetime.
    pub data_channels_opened: Option<u64>,

    /// Number of unique `DataChannel`s that have left the `open` state during
    /// their lifetime (due to being closed by either end or the underlying
    /// transport being closed). `DataChannel`s that transition from
    /// `connecting` to `closing` or `closed` without ever being `open` are not
    /// counted in this number.
    pub data_channels_closed: Option<u64>,
}

/// Non-exhaustive version of [`KnownRtcDataChannelState`].
pub type RtcDataChannelState = NonExhaustive<KnownRtcDataChannelState>;

/// State of the [RTCDataChannel]'s underlying data connection.
///
/// [RTCDataChannel]: https://w3.org/TR/webrtc#dom-rtcdatachannel
#[derive(
    Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum KnownRtcDataChannelState {
    /// User agent is attempting to establish the underlying data transport.
    /// This is the initial state of [RTCDataChannel] object, whether created
    /// with [createDataChannel][1], or dispatched as a part of an
    /// [RTCDataChannelEvent].
    ///
    /// [RTCDataChannel]: https://w3.org/TR/webrtc#dom-rtcdatachannel
    /// [RTCDataChannelEvent]: https://w3.org/TR/webrtc#dom-rtcdatachannelevent
    /// [1]: https://w3.org/TR/webrtc#dom-peerconnection-createdatachannel
    #[display("connecting")]
    Connecting,

    /// [Underlying data transport][1] is established and communication is
    /// possible.
    ///
    /// [1]: https://w3.org/TR/webrtc#dfn-data-transport
    #[display("open")]
    Open,

    /// [`procedure`][2] to close down the [underlying data transport][1] has
    /// started.
    ///
    /// [1]: https://w3.org/TR/webrtc#dfn-data-transport
    /// [2]: https://w3.org/TR/webrtc#data-transport-closing-procedure
    #[display("closing")]
    Closing,

    /// [Underlying data transport][1] has been [`closed`][2] or could not be
    /// established.
    ///
    /// [1]: https://w3.org/TR/webrtc#dfn-data-transport
    /// [2]: https://w3.org/TR/webrtc#dom-rtcdatachannelstate-closed
    #[display("closed")]
    Closed,
}

/// Statistics related to each [RTCDataChannel] ID.
///
/// [RTCDataChannel]: https://w3.org/TR/webrtc#dom-rtcdatachannel
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcDataChannelStats {
    /// [`label`] value of the [RTCDataChannel] object.
    ///
    /// [RTCDataChannel]: https://w3.org/TR/webrtc#dom-rtcdatachannel
    /// [`label`]: https://w3.org/TR/webrtc#dom-datachannel-label
    pub label: Option<String>,

    /// [`protocol`][1] value of the [RTCDataChannel] object.
    ///
    /// [RTCDataChannel]: https://w3.org/TR/webrtc#dom-rtcdatachannel
    /// [1]: https://w3.org/TR/webrtc#dom-datachannel-protocol
    pub protocol: Option<String>,

    /// [`id`][1] attribute of the [RTCDataChannel] object.
    ///
    /// [RTCDataChannel]: https://w3.org/TR/webrtc#dom-rtcdatachannel
    /// [1]: https://w3.org/TR/webrtc#dom-rtcdatachannel-id
    pub data_channel_identifier: Option<u64>,

    /// [`readyState`][1] value of the [RTCDataChannel] object.
    ///
    /// [RTCDataChannel]: https://w3.org/TR/webrtc#dom-rtcdatachannel
    /// [1]: https://w3.org/TR/webrtc#dom-datachannel-readystate
    pub state: Option<RtcDataChannelState>,

    /// Total number of API `message` events sent.
    pub messages_sent: Option<u64>,

    /// Total number of payload bytes sent on this [RTCDataChannel].
    ///
    /// [RTCDataChannel]: https://w3.org/TR/webrtc#dom-rtcdatachannel
    pub bytes_sent: Option<u64>,

    /// Total number of API `message` events received.
    pub messages_received: Option<u64>,

    /// Total number of bytes received on this [RTCDataChannel].
    ///
    /// [RTCDataChannel]: https://w3.org/TR/webrtc#dom-rtcdatachannel
    pub bytes_received: Option<u64>,
}

/// Represents the stats corresponding to an [RTCDtlsTransport] and its
/// underlying [RTCIceTransport]. When bundling is used, a single transport
/// will be used for all [MediaStreamTrack][0]s in the bundle group. If
/// bundling is not used, different [MediaStreamTrack][0] will use different
/// transports. Bundling is described in [WEBRTC].
///
/// [RTCDtlsTransport]: https://w3.org/TR/webrtc#dom-rtcdtlstransport
/// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
/// [WEBRTC]: https://w3.org/TR/webrtc/
/// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcTransportStats {
    /// Total number of packets sent over this transport.
    pub packets_sent: Option<u64>,

    /// Total number of packets received on this transport.
    pub packets_received: Option<u64>,

    /// Represents the total number of payload bytes sent on this
    /// [RTCIceTransport], i.e., not including headers, padding or ICE
    /// connectivity checks.
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
    pub bytes_sent: Option<u64>,

    /// Represents the total number of payload bytes received on this
    /// [RTCIceTransport], i.e., not including headers, padding or ICE
    /// connectivity checks.
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
    pub bytes_received: Option<u64>,

    /// Set to the current value of the [`role` attribute][1] of the
    /// [underlying RTCDtlsTransport's `transport`][2].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-icetransport-role
    /// [2]: https://w3.org/TR/webrtc#dom-rtcdtlstransport-icetransport
    pub ice_role: Option<RtcIceRole>,

    /// Set to the current value of the local username fragment used in
    /// message validation procedures [RFC5245] for this [RTCIceTransport].
    /// It may be updated on [setLocalDescription()][0] and on ICE restart.
    ///
    /// [RFC5245]: https://rfc-editor.org/rfc/rfc5245
    /// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
    /// [0]: https://w3.org/TR/webrtc/#dom-peerconnection-setlocaldescription
    pub ice_local_username_fragment: Option<String>,

    /// Set to the current value of the state attribute of the underlying
    /// [RTCIceTransport].
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
    pub ice_state: Option<RtcIceTransportState>,

    /// Set to the current value of the state attribute of the underlying
    /// [RTCDtlsTransport].
    ///
    /// [RTCDtlsTransport]: https://w3.org/TR/webrtc#dom-rtcdtlstransport
    pub dtls_state: Option<RtcDtlsTransportState>,

    /// It is a unique identifier that is associated to the object that was
    /// inspected to produce the [`RtcIceCandidatePairStats`] associated with
    /// this transport.
    pub selected_candidate_pair_id: Option<String>,

    /// For components where DTLS is negotiated, give local certificate.
    pub local_certificate_id: Option<String>,

    /// For components where DTLS is negotiated, give remote certificate.
    pub remote_certificate_id: Option<String>,

    /// For components where DTLS is negotiated, the TLS version agreed. Only
    /// exists after DTLS negotiation is complete.
    pub tls_version: Option<String>,

    /// Set to the current value of the state attribute of the underlying
    /// [RTCDtlsTransport].
    ///
    /// [RTCDtlsTransport]: https://w3.org/TR/webrtc#dom-rtcdtlstransport
    pub dtls_cipher: Option<String>,

    /// [`KnownRtcDtlsRole::Client`] or [`KnownRtcDtlsRole::Server`] depending
    /// on the DTLS role. [`KnownRtcDtlsRole::Unknown`] before the DTLS
    /// negotiation starts.
    pub dtls_role: Option<RtcDtlsRole>,

    /// Descriptive name of the protection profile used for the SRTP transport,
    /// as defined in the "Profile" column of the IANA DTLS-SRTP protection
    /// profile registry [IANA-DTLS-SRTP][0] and described further in [RFC5764].
    ///
    /// [0]: https://iana.org/assignments/srtp-protection/srtp-protection.xhtml
    /// [RFC5764]: https://rfc-editor.org/rfc/rfc5764
    pub srtp_cipher: Option<String>,

    /// The number of Transport-Layer Feedback Messages of type
    /// `CongestionControl Feedback Packet`, as described in [RFC8888]
    /// section 3.1, sent on this transport.
    ///
    /// [RFC8888]: https://rfc-editor.org/rfc/rfc8888
    pub ccfb_messages_sent: Option<u32>,

    /// The number of Transport-Layer Feedback Messages of type
    /// `CongestionControl Feedback Packet`, as described in
    /// [RFC8888] section 3.1, received on this transport
    ///
    /// [RFC8888]: https://rfc-editor.org/rfc/rfc8888
    pub ccfb_messages_received: Option<u32>,

    /// The number of times that the selected candidate pair of this transport
    /// has changed. Going from not having a selected candidate pair to having
    /// a selected candidate pair, or the other way around, also increases
    /// this counter. It is initially zero and becomes one when an initial
    /// candidate pair is selected.
    pub selected_candidate_pair_changes: Option<u32>,
}

/// ICE candidate pair statistics related to the [RTCIceTransport] objects.
///
/// A candidate pair that is not the current pair for a transport is
/// [deleted][1] when the [RTCIceTransport] does an ICE restart, at the time
/// the state changes to [`KnownRtcIceTransportState::New`].
///
/// The candidate pair that is the current pair for a transport is deleted after
/// an ICE restart when the [RTCIceTransport] switches to using a candidate pair
/// generated from the new candidates; this time doesn't correspond to any other
/// externally observable event.
///
/// [`RtcStatsType::CandidatePair`] variant.
///
/// [Full doc on W3C][2].
///
/// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
/// [1]: https://w3.org/TR/webrtc-stats#dfn-deleted
/// [2]: https://w3.org/TR/webrtc-stats#candidatepair-dict%2A
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcIceCandidatePairStats {
    /// Unique identifier associated to the object that was inspected to
    /// produce the [`RtcTransportStats`] associated with this candidates
    /// pair.
    pub transport_id: Option<String>,

    /// Unique identifier associated to the object that was inspected to
    /// produce the [`RtcIceCandidateStats`] for the local candidate
    /// associated with this candidates pair.
    pub local_candidate_id: Option<String>,

    /// Unique identifier associated to the object that was inspected to
    /// produce the [`RtcIceCandidateStats`] for the remote candidate
    /// associated with this candidates pair.
    pub remote_candidate_id: Option<String>,

    /// State of the checklist for the local and remote candidates in a pair.
    pub state: RtcStatsIceCandidatePairState,

    /// Related to updating the nominated flag described in
    /// [Section 7.1.3.2.4 of RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-7.1.3.2.4
    pub nominated: Option<bool>,

    /// Calculated as defined in [Section 15.1 of RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-15.1
    pub priority: Option<u64>,

    /// Represents the total number of packets sent on this candidate pair.
    pub packets_sent: Option<u64>,

    /// Represents the total number of packets received on this candidate pair.
    pub packets_received: Option<u64>,

    /// Represents the total number of payload bytes sent on this candidate
    /// pair, i.e., not including headers, padding or ICE connectivity checks.
    pub bytes_sent: Option<u64>,

    /// Represents the total number of payload bytes received on this
    /// candidate pair, i.e., not including headers, padding or ICE
    /// connectivity checks.
    pub bytes_received: Option<u64>,

    /// Represents the timestamp at which the last packet was sent on this
    /// particular candidate pair, excluding STUN packets.
    pub last_packet_sent_timestamp: Option<HighResTimeStamp>,

    /// Represents the timestamp at which the last packet was received on this
    /// particular candidate pair, excluding STUN packets.
    pub last_packet_received_timestamp: Option<HighResTimeStamp>,

    /// Sum of all round trip time measurements in seconds since the beginning
    /// of the session, based on STUN connectivity check [STUN-PATH-CHAR]
    /// responses (responsesReceived), including those that reply to requests
    /// that are sent in order to verify consent [RFC 7675].
    ///
    /// The average round trip time can be computed from
    /// [`Self::total_round_trip_time`] by dividing it by
    /// [`Self::responses_received`].
    ///
    /// [STUN-PATH-CHAR]: https://w3.org/TR/webrtc-stats#bib-stun-path-char
    /// [RFC 7675]: https://tools.ietf.org/html/rfc7675
    pub total_round_trip_time: Option<Double>,

    /// Latest round trip time measured in seconds, computed from both STUN
    /// connectivity checks [STUN-PATH-CHAR], including those that are sent for
    /// consent verification [RFC 7675].
    ///
    /// [STUN-PATH-CHAR]: https://w3.org/TR/webrtc-stats#bib-stun-path-char
    /// [RFC 7675]: https://tools.ietf.org/html/rfc7675
    pub current_round_trip_time: Option<Double>,

    /// It is calculated by the underlying congestion control by combining the
    /// available bitrate for all the outgoing [RTP stream]s using this
    /// candidate pair. The bitrate measurement does not count the size of the
    /// IP or other transport layers like TCP or UDP. It is similar to the TIAS
    /// defined in [RFC3890], i.e., it is measured in bits per second and the
    /// bitrate is calculated over a `1` second window. For candidate pairs in
    /// use, the estimate is normally no lower than the bitrate for the packets
    /// sent at [`Self::last_packet_sent_timestamp`], but might be higher.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    /// [RFC3890]: https://rfc-editor.org/rfc/rfc3890
    pub available_outgoing_bitrate: Option<Double>,

    /// It is calculated by the underlying congestion control by combining the
    /// available bitrate for all the incoming RTP streams using this candidate
    /// pair. The bitrate measurement does not count the size of the IP or
    /// other transport layers like TCP or UDP. It is similar to the TIAS
    /// defined in [RFC3890], i.e., it is measured in bits per second and
    /// the bitrate is calculated over a `1` second window. For pairs in use,
    /// the estimate is normally no lower than the bitrate for the packets
    /// received at [`Self::last_packet_received_timestamp`], but might be
    /// higher.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    /// [RFC3890]: https://rfc-editor.org/rfc/rfc3890
    pub available_incoming_bitrate: Option<Double>,

    /// Represents the total number of connectivity check requests received
    /// (including retransmissions). It is impossible for the receiver to tell
    /// whether the request was sent in order to check connectivity or check
    /// consent, so all connectivity checks requests are counted here.
    pub requests_received: Option<u64>,

    /// Represents the total number of connectivity check requests sent
    /// (not including retransmissions).
    pub requests_sent: Option<u64>,

    /// Represents the total number of connectivity check responses
    /// received.
    pub responses_received: Option<u64>,

    /// Represents the total number of connectivity check responses sent.
    /// Since we cannot distinguish connectivity check requests and consent
    /// requests, all responses are counted.
    pub responses_sent: Option<u64>,

    /// Represents the total number of consent requests sent.
    pub consent_requests_sent: Option<u64>,

    /// Total number of packets for this candidate pair that have been
    /// discarded due to socket errors, i.e. a socket error occurred when
    /// handing the packets to the socket. This might happen due to various
    /// reasons, including full buffer or no available memory.
    pub packets_discarded_on_send: Option<u32>,

    /// Total number of bytes for this candidate pair that have been discarded
    /// due to socket errors, i.e. a socket error occurred when handing the
    /// packets containing the bytes to the socket. This might happen due to
    /// various reasons, including full buffer or no available memory.
    /// Calculated as defined in [RFC3550] section 6.4.1.
    ///
    /// [RFC3550]: https://rfc-editor.org/rfc/rfc3550
    pub bytes_discarded_on_send: Option<u64>,
}

/// Reflects the properties of a `candidate` in Section 15.1 of [RFC5245]. It
/// corresponds to a [RTCIceCandidate] object.
///
/// [RFC5245]: https://rfc-editor.org/rfc/rfc5245
/// [RTCIceCandidate]: https://w3.org/TR/webrtc/#dom-rtcicecandidate
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcIceCandidateStats {
    /// Unique ID that is associated to the object that was inspected to
    /// produce the [`RtcTransportStats`] associated with this candidate.
    pub transport_id: Option<String>,

    /// Address of the candidate, allowing for IPv4 addresses, IPv6 addresses,
    /// and fully qualified domain names (FQDNs).
    ///
    /// See [RFC5245] section 15.1 for details.
    ///
    /// [RFC5245]: https://rfc-editor.org/rfc/rfc5245
    pub address: Option<String>,

    /// Port number of the candidate.
    pub port: Option<u16>,

    /// Valid values for transport is one of `udp` and `tcp`. Based on the
    /// "transport" defined in [RFC5245] section 15.1.
    ///
    /// [RFC5245]: https://rfc-editor.org/rfc/rfc5245
    pub protocol: Option<String>,

    /// Type of the ICE candidate.
    pub candidate_type: RtcIceCandidateType,

    /// Calculated as defined in [RFC5245] section 15.1.
    ///
    /// [RFC5245]: https://rfc-editor.org/rfc/rfc5245
    pub priority: Option<u32>,

    /// For local candidates of type [`KnownRtcIceCandidateType::Srflx`] or type
    /// [`KnownRtcIceCandidateType::Relay`] this is the URL of the ICE server
    /// from which the candidate was obtained and defined in [WEBRTC].
    ///
    /// For remote candidates, this property MUST NOT be present.
    ///
    /// [WEBRTC]: https://w3.org/TR/webrtc/
    pub url: Option<String>,

    /// The protocol used by the endpoint to communicate with the TURN server.
    /// This is only present for local relay candidates and defined in
    /// [WEBRTC].
    ///
    /// For remote candidates, this property MUST NOT be present.
    ///
    /// [WEBRTC]: https://w3.org/TR/webrtc/
    pub relay_protocol: Option<IceServerTransportProtocol>,

    /// The ICE foundation as defined in [RFC5245] section 15.1.
    ///
    /// [RFC5245]: https://rfc-editor.org/rfc/rfc5245
    pub foundation: Option<String>,

    /// The ICE rel-addr defined in [RFC5245] section 15.1. Only set for
    /// [`KnownRtcIceCandidateType::Srflx`],
    /// [`KnownRtcIceCandidateType::Prflx`] and
    /// [`KnownRtcIceCandidateType::Relay`] candidates.
    ///
    /// [RFC5245]: https://rfc-editor.org/rfc/rfc5245
    pub related_address: Option<String>,

    /// The ICE rel-addr defined in [RFC5245] section 15.1. Only set for
    /// [`KnownRtcIceCandidateType::Srflx`],
    /// [`KnownRtcIceCandidateType::Prflx`] and
    /// [`KnownRtcIceCandidateType::Relay`] candidates.
    ///
    /// [RFC5245]: https://rfc-editor.org/rfc/rfc5245
    pub related_port: Option<i32>,

    /// The ICE username fragment as defined in [RFC5245] section 7.1.2.3. For
    /// [`KnownRtcIceCandidateType::Prflx`] remote candidates this is not set
    /// unless the ICE username fragment has been previously signaled.
    ///
    /// [RFC5245]: https://rfc-editor.org/rfc/rfc5245
    pub username_fragment: Option<String>,

    /// The ICE candidate TCP type, as defined іn [`RtcIceTcpCandidateType`]
    /// and used in [RTCIceCandidate].
    ///
    /// [RTCIceCandidate]: https://w3.org/TR/webrtc/#dom-rtcicecandidate
    pub tcp_type: Option<RtcIceTcpCandidateType>,

    /// Specifies the type of network used by a local ICE candidate.
    pub network_type: Option<String>,
}

/// Non-exhaustive version of [`KnownIceServerTransportProtocol`].
pub type IceServerTransportProtocol =
    NonExhaustive<KnownIceServerTransportProtocol>;

/// Represents the type of the transport protocol used between the client and
/// the server, as defined in [RFC8656] section 3.1.
///
/// [RFC8656]: https://rfc-editor.org/rfc/rfc8656
#[derive(
    Clone, Copy, Debug, Deserialize, Display, Serialize, PartialEq, Eq, Hash,
)]
#[serde(rename_all = "lowercase")]
pub enum KnownIceServerTransportProtocol {
    /// The TURN client is using UDP as transport to the server.
    #[display("udp")]
    Udp,

    /// The TURN client is using TCP as transport to the server.
    #[display("tcp")]
    Tcp,

    /// The TURN client is using TLS as transport to the server.
    #[display("tls")]
    Tls,
}

/// Non-exhaustive version of [`KnownRtcIceTcpCandidateType`].
pub type RtcIceTcpCandidateType = NonExhaustive<KnownRtcIceTcpCandidateType>;

/// Represents the type of the ICE TCP candidate, as defined in [RFC6544].
///
/// [RFC6544]: https://rfc-editor.org/rfc/rfc6544
#[derive(
    Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum KnownRtcIceTcpCandidateType {
    /// An [`Self::Active`] TCP candidate is one for which the transport will
    /// attempt to open an outbound connection but will not receive incoming
    /// connection requests.
    #[display("active")]
    Active,

    /// A [`Self::Passive`] TCP candidate is one for which the transport will
    /// receive incoming connection attempts but not attempt a connection.
    #[display("passive")]
    Passive,

    /// An [`Self::So`] candidate is one for which the transport will attempt
    /// to open a connection simultaneously with its peer.
    #[display("so")]
    So,
}

/// Information about a certificate used by [RTCIceTransport].
///
/// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcCertificateStats {
    /// Fingerprint of the certificate.
    ///
    /// Only use the fingerprint value as defined in [Section 5 of RFC
    /// 4572][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc4572#section-5
    pub fingerprint: String,

    /// Hash function used to compute the certificate fingerprint.
    /// For instance, `sha-256`.
    pub fingerprint_algorithm: String,

    /// The DER-encoded Base64 representation of the certificate.
    pub base64_certificate: String,
}

/// Non-exhaustive version of [`KnownRtcIceRole`].
pub type RtcIceRole = NonExhaustive<KnownRtcIceRole>;

/// Variants of [ICE roles][1].
///
/// More info in the [RFC 5245].
///
/// [RFC 5245]: https://tools.ietf.org/html/rfc5245
/// [1]: https://w3.org/TR/webrtc#dom-icetransport-role
#[derive(
    Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum KnownRtcIceRole {
    /// Agent whose role as defined by [Section 3 in RFC 5245][1], has not yet
    /// been determined.
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-3
    #[display("unknown")]
    Unknown,

    /// Controlling agent as defined by [Section 3 in RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-3
    #[display("controlling")]
    Controlling,

    /// Controlled agent as defined by [Section 3 in RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-3
    #[display("controlled")]
    Controlled,
}

/// Non-exhaustive version of [`KnownRtcDtlsTransportState`].
pub type RtcDtlsTransportState = NonExhaustive<KnownRtcDtlsTransportState>;

/// Describes the state of the DTLS transport.
#[derive(
    Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum KnownRtcDtlsTransportState {
    /// DTLS has not started negotiating yet.
    #[display("new")]
    New,

    /// DTLS is in the process of negotiating a secure connection and
    /// verifying the remote fingerprint.
    #[display("connecting")]
    Connecting,

    /// DTLS has completed negotiation of a secure connection and verified
    /// the remote fingerprint.
    #[display("connected")]
    Connected,

    /// The transport has been closed intentionally as the result of receipt of
    /// a `close_notify` alert, or calling [close()].
    ///
    /// [close()]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection-close
    #[display("closed")]
    Closed,

    /// The transport has failed as the result of an error (such as receipt of
    /// an error alert or failure to validate the remote fingerprint).
    #[display("failed")]
    Failed,
}

/// Non-exhaustive version of [`KnownRtcIceTransportState`].
pub type RtcIceTransportState = NonExhaustive<KnownRtcIceTransportState>;

/// Describes the status of the underlying [ICE] transport used by a
/// [RTCPeerConnection].
///
/// [ICE]: https://datatracker.ietf.org/doc/html/rfc8445
/// [RTCPeerConnection]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection
#[derive(
    Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum KnownRtcIceTransportState {
    /// The [RTCIceTransport] has shut down and is no longer responding to
    /// STUN requests
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc/#dom-rtcicetransport
    #[display("closed")]
    Closed,

    /// The [RTCIceTransport] has finished gathering, received an indication
    /// that there are no more remote candidates, finished checking all
    /// candidate pairs, and all pairs have either failed connectivity checks
    /// or lost consent, and either zero local candidates were gathered or
    /// the PAC timer has expired [RFC8863]. This is a terminal state until
    /// ICE is restarted. Since an ICE restart may cause connectivity to
    /// resume, entering the [`KnownRtcIceTransportState::Failed`] state does
    /// not cause DTLS transports, SCTP associations or the data channels that
    /// run over them to close, or tracks to mute.
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc/#dom-rtcicetransport
    /// [RFC8863]: https://rfc-editor.org/rfc/rfc8863
    #[display("failed")]
    Failed,

    /// The [ICE Agent] has determined that connectivity is currently lost for
    /// this [RTCIceTransport]. This is a transient state that may trigger
    /// intermittently (and resolve itself without action) on a flaky network.
    /// The way this state is determined is implementation dependent.
    /// Examples include:
    /// - Losing the network interface for the connection in use.
    /// - Repeatedly failing to receive a response to STUN requests.
    ///
    /// Alternatively, the [RTCIceTransport] has finished checking all existing
    /// candidates pairs and not found a connection (or consent checks
    /// [RFC7675] once successful, have now failed), but it is still
    /// gathering and/or waiting for additional remote candidates.
    ///
    /// [ICE Agent]: https://w3.org/TR/webrtc/#dfn-ice-agent
    /// [RTCIceTransport]: https://w3.org/TR/webrtc/#dom-rtcicetransport
    /// [RFC7675]: https://rfc-editor.org/rfc/rfc7675
    #[display("disconnected")]
    Disconnected,

    /// The [RTCIceTransport] is gathering candidates and/or waiting for remote
    /// candidates to be supplied, and has not yet started checking.
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc/#dom-rtcicetransport
    #[display("new")]
    New,

    /// The [RTCIceTransport] has received at least one remote candidate (by
    /// means of [addIceCandidate()][0] or discovered as a peer-reflexive
    /// candidate when receiving a STUN binding request) and is checking
    /// candidate pairs and has either not yet found a connection or consent
    /// checks [RFC7675] have failed on all previously successful candidate
    /// pairs. In addition to checking, it may also still be gathering.
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc/#dom-rtcicetransport
    /// [0]: https://w3.org/TR/webrtc/#dom-peerconnection-addicecandidate
    /// [RFC7675]: https://rfc-editor.org/rfc/rfc7675
    #[display("checking")]
    Checking,

    /// The [RTCIceTransport] has finished gathering, received an indication
    /// that there are no more remote candidates, finished checking all
    /// candidate pairs and found a connection. If consent checks [RFC7675]
    /// subsequently fail on all successful candidate pairs, the state
    /// transitions to [`KnownRtcIceTransportState::Failed`].
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc/#dom-rtcicetransport
    /// [RFC7675]: https://rfc-editor.org/rfc/rfc7675
    #[display("completed")]
    Completed,

    /// The [RTCIceTransport] has found a usable connection, but is still
    /// checking other candidate pairs to see if there is a better connection.
    /// It may also still be gathering and/or waiting for additional remote
    /// candidates. If consent checks [RFC7675] fail on the connection in use,
    /// and there are no other successful candidate pairs available, then
    /// the state transitions to [`Self::Checking`] (if
    /// there are candidate pairs remaining to be checked) or
    /// [`Self::Disconnected`] (if there are no candidate pairs to check,
    /// but the peer is still gathering and/or waiting for additional remote
    /// candidates).
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc/#dom-rtcicetransport
    /// [RFC7675]: https://rfc-editor.org/rfc/rfc7675
    #[display("connected")]
    Connected,
}

/// Non-exhaustive version of [`KnownRtcDtlsRole`].
pub type RtcDtlsRole = NonExhaustive<KnownRtcDtlsRole>;

/// Indicates the role in the DTLS handshake for a transport.
#[derive(
    Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum KnownRtcDtlsRole {
    /// The [RTCPeerConnection] is acting as a DTLS client as defined in
    /// [RFC6347].
    ///
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection
    /// [RFC6347]: https://rfc-editor.org/rfc/rfc6347
    #[display("client")]
    Client,

    /// The [RTCPeerConnection] is acting as a DTLS server as defined in
    /// [RFC6347].
    ///
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection
    /// [RFC6347]: https://rfc-editor.org/rfc/rfc6347
    #[display("server")]
    Server,

    /// The DTLS role of the [RTCPeerConnection] has not been determined yet.
    ///
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection
    #[display("unknown")]
    Unknown,
}

/// Non-exhaustive version of [`KnownRtcStatsIceCandidatePairState`].
pub type RtcStatsIceCandidatePairState =
    NonExhaustive<KnownRtcStatsIceCandidatePairState>;

/// Possible states of a candidate pair.
#[derive(
    Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum KnownRtcStatsIceCandidatePairState {
    /// Check for this pair hasn't been performed, and it can't yet be
    /// performed until some other check succeeds, allowing this pair to
    /// unfreeze and move into the
    /// [`KnownRtcStatsIceCandidatePairState::Waiting`] state.
    #[display("frozen")]
    Frozen,

    /// Check has not been performed for this pair, and can be performed as
    /// soon as it is the highest-priority Waiting pair on the check list.
    #[display("waiting")]
    Waiting,

    /// Check has been sent for this pair, but the transaction is in progress.
    #[display("in-progress")]
    InProgress,

    /// Check for this pair was already done and failed, either never producing
    /// any response or producing an unrecoverable failure response.
    #[display("failed")]
    Failed,

    /// Check for this pair was already done and produced a successful result.
    #[display("succeeded")]
    Succeeded,

    /// Other Candidate pair was nominated.
    ///
    /// This state is **obsolete and not spec compliant**, however, it still
    /// may be emitted by some implementations.
    #[display("cancelled")]
    Cancelled,
}

/// Non-exhaustive version of [`KnownRtcIceCandidateType`].
pub type RtcIceCandidateType = NonExhaustive<KnownRtcIceCandidateType>;

/// [RTCIceCandidateType] represents the type of the ICE candidate, as
/// defined in [Section 15.1 of RFC 5245][1].
///
/// [RTCIceCandidateType]: https://w3.org/TR/webrtc#rtcicecandidatetype-enum
/// [1]: https://tools.ietf.org/html/rfc5245#section-15.1
#[derive(
    Clone, Copy, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum KnownRtcIceCandidateType {
    /// Host candidate, as defined in [Section 4.1.1.1 of RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-4.1.1.1
    #[display("host")]
    Host,

    /// Server reflexive candidate, as defined in
    /// [Section 4.1.1.2 of RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-4.1.1.2
    #[display("srflx")]
    Srflx,

    /// Peer reflexive candidate, as defined in
    /// [Section 4.1.1.2 of RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-4.1.1.2
    #[display("prflx")]
    Prflx,

    /// Relay candidate, as defined in [Section 7.1.3.2.1 of RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-7.1.3.2.1
    #[display("relay")]
    Relay,
}

/// Media kind specific [`RtcInboundRtpStreamStats`]'s part.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(tag = "mediaType", rename_all = "camelCase")]
pub enum InboundRtpMediaType {
    /// Fields when the `kind` is `audio`.
    #[serde(rename_all = "camelCase")]
    Audio {
        /// Total number of samples that have been received on this RTP stream.
        /// This includes [`InboundRtpMediaType::Audio::concealed_samples`].
        total_samples_received: Option<u64>,

        /// The total number of samples that are concealed samples. A concealed
        /// sample is a sample that was replaced with synthesized samples
        /// generated locally before being played out. Examples of samples
        /// that have to be concealed are samples from lost packets (reported
        /// in [`RtcReceivedRtpStreamStats::packets_lost`]) or samples from
        /// packets that arrive too late to be played out (reported
        /// in [`RtcInboundRtpStreamStats::packets_discarded`]).
        concealed_samples: Option<u64>,

        /// The total number of concealed samples inserted that are "silent".
        /// Playing out silent samples results in silence or comfort noise.
        /// This is a subset of
        /// [`InboundRtpMediaType::Audio::concealed_samples`].
        silent_concealed_samples: Option<u64>,

        /// The number of concealment events. This counter increases every
        /// time a concealed sample is synthesized after a non-concealed
        /// sample. That is, multiple consecutive concealed samples will
        /// increase the
        /// [`InboundRtpMediaType::Audio::concealed_samples`] count
        /// multiple times but is a single concealment event.
        concealment_events: Option<u64>,

        /// When playout is slowed down, this counter is increased by the
        /// difference between the number of samples received and the number
        /// of samples played out. If playout is slowed down by inserting
        /// samples, this will be the number of inserted samples.
        inserted_samples_for_deceleration: Option<u64>,

        /// When playout is sped up, this counter is increased by the
        /// difference between the number of samples received and the number
        /// of samples played out. If speedup is achieved by removing samples,
        /// this will be the count of samples removed.
        removed_samples_for_acceleration: Option<u64>,

        /// Represents the audio level of the receiving track. For audio
        /// levels of tracks attached locally, see [`MediaSourceKind::Audio`]
        /// instead.
        ///
        /// The value is between `0..1` (linear), where `1.0` represents
        /// `0 dBov`, `0` represents silence, and `0.5` represents approximately
        /// `6 dBSPL` change in the sound pressure level from `0 dBov`.
        ///
        /// The audioLevel is averaged over some small interval, using the
        /// algorithm described under totalAudioEnergy.
        /// The interval used is implementation-defined.
        audio_level: Option<Double>,

        /// Represents the audio energy of the receiving track. For audio
        /// energy of tracks attached locally, see [`MediaSourceKind::Audio`]
        /// instead.
        total_audio_energy: Option<Double>,

        /// Represents the audio duration of the receiving track.
        /// For audio durations of tracks attached locally, see
        /// [`MediaSourceKind::Audio`] instead.
        total_samples_duration: Option<Double>,

        /// If audio playout is happening, this is used to look up the
        /// corresponding [`RtcAudioPlayoutStats`].
        playout_id: Option<String>,
    },

    /// Fields when the `kind` is `video`.
    #[serde(rename_all = "camelCase")]
    Video {
        /// Total number of frames correctly decoded for this [RTP stream], i.e.
        /// frames that would be displayed if no frames are dropped.
        ///
        /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
        frames_decoded: Option<u32>,

        /// Total number of key frames, such as key frames in VP8 [RFC6386] or
        /// IDR-frames in H.264 [RFC6184], successfully decoded for this RTP
        /// media stream.
        ///
        /// This is a subset of
        /// [`InboundRtpMediaType::Video::frames_decoded`].
        /// [`InboundRtpMediaType::Video::frames_decoded`] -
        /// [`InboundRtpMediaType::Video::key_frames_decoded`] gives
        /// you the number of delta frames decoded.
        ///
        /// [RFC6386]: https://w3.org/TR/webrtc-stats#bib-rfc6386
        /// [RFC6184]: https://w3.org/TR/webrtc-stats#bib-rfc6184
        key_frames_decoded: Option<u32>,

        /// It represents the total number of frames that have been rendered.
        /// It is incremented just after a frame has been rendered.
        frames_rendered: Option<u32>,

        /// The total number of frames dropped prior to decode or dropped
        /// because the frame missed its display deadline for this receiver's
        /// track. The measurement begins when the receiver is created and
        /// is a cumulative metric as defined in Appendix A (g) of [RFC7004].
        ///
        /// [RFC7004]: https://rfc-editor.org/rfc/rfc7004
        frames_dropped: Option<u32>,

        /// Width of the last decoded frame.
        ///
        /// Before the first frame is decoded this attribute is missing.
        frame_width: Option<u64>,

        /// Height of the last decoded frame.
        ///
        /// Before the first frame is decoded this attribute is missing.
        frame_height: Option<u64>,

        /// The number of decoded frames in the last second.
        frames_per_second: Option<Double>,

        /// The sum of the QP values of frames decoded by this receiver.
        /// The count of frames is in
        /// [`InboundRtpMediaType::Video::frames_decoded`].
        ///
        /// The definition of QP value depends on the codec; for VP8, the QP
        /// value is the value carried in the frame header as the syntax
        /// element `y_ac_qi`, and defined in [RFC6386] section 19.2. Its range
        /// is `0..127`.
        ///
        /// Note that the QP value is only an indication of quantizer values
        /// used; many formats have ways to vary the quantizer value within
        /// the frame.
        ///
        /// [RFC6386]: https://rfc-editor.org/rfc/rfc6386
        qp_sum: Option<u64>,

        /// Total number of seconds that have been spent decoding the
        /// [`InboundRtpMediaType::Video::frames_decoded`] frames of
        /// this stream. The average decode time can be calculated by dividing
        /// this value with
        /// [`InboundRtpMediaType::Video::frames_decoded`]. The time
        /// it takes to decode one frame is the time passed between feeding the
        /// decoder a frame and the decoder returning decoded data for that
        /// frame.
        total_decode_time: Option<Double>,

        /// Sum of the interframe delays in seconds between consecutively
        /// rendered frames, recorded just after a frame has been rendered.
        /// The interframe delay variance be calculated from
        /// [`InboundRtpMediaType::Video::total_inter_frame_delay`],
        /// [`InboundRtpMediaType::Video::total_squared_inter_frame_delay`],
        /// and [`InboundRtpMediaType::Video::frames_rendered`]
        /// according to the formula:
        /// `(total_squared_inter_frame_delay - total_inter_frame_delay^2 /
        /// frames_rendered) / frames_rendered`.
        total_inter_frame_delay: Option<Double>,

        /// Sum of the squared interframe delays in seconds between
        /// consecutively rendered frames, recorded just after a frame has
        /// been rendered. See
        /// [`InboundRtpMediaType::Video::total_inter_frame_delay`]
        /// for details on how to calculate the interframe delay variance.
        total_squared_inter_frame_delay: Option<Double>,

        /// Count the total number of video pauses experienced by this receiver.
        /// Video is considered to be paused if time passed since last rendered
        /// frame exceeds 5 seconds. It is incremented when a frame is rendered
        /// after such a pause.
        pause_count: Option<u32>,

        /// Total duration of pauses (for definition of pause see
        /// [`InboundRtpMediaType::Video::pause_count`]), in seconds.
        /// This value is updated when a frame is rendered.
        total_pauses_duration: Option<Double>,

        /// Count the total number of video freezes experienced by this
        /// receiver. It is a freeze if frame duration, which is time interval
        /// between two consecutively rendered frames, is equal or exceeds
        /// `Max(3 * avg_frame_duration_ms, avg_frame_duration_ms + 150)`,
        /// where `avg_frame_duration_ms` is linear average of durations of
        /// last 30 rendered frames.
        freeze_count: Option<u32>,

        /// Total duration of rendered frames which are considered as frozen
        /// (for definition of freeze see
        /// [`InboundRtpMediaType::Video::freeze_count`]), in seconds.
        /// This value is updated when a frame is rendered.
        total_freezes_duration: Option<Double>,

        /// Count the total number of Full Intra Request (FIR) packets, as
        /// defined in [RFC5104] section 4.3.1, sent by this receiver. Does not
        /// count the RTCP FIR indicated in [RFC2032] which was deprecated by
        /// [RFC4587].
        ///
        /// [RFC5104]: https://rfc-editor.org/rfc/rfc5104
        /// [RFC2032]: https://rfc-editor.org/rfc/rfc2032
        /// [RFC4587]: https://rfc-editor.org/rfc/rfc4587
        fir_count: Option<u64>,

        /// Count the total number of Picture Loss Indication (PLI) packets,
        /// as defined in [RFC4585] section 6.3.1, sent by this receiver.
        ///
        /// [RFC4585]: https://rfc-editor.org/rfc/rfc4585
        pli_count: Option<u64>,

        /// Represents the total number of complete frames received on this
        /// [RTP stream]. This metric is incremented when the complete frame is
        /// received.
        ///
        /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
        frames_received: Option<u64>,

        /// Identifies the decoder implementation used. This is useful for
        /// diagnosing interoperability issues.
        decoder_implementation: Option<String>,

        /// Whether the decoder currently used is considered power efficient
        /// by the user agent. This SHOULD reflect if the configuration
        /// results in hardware acceleration, but the user agent MAY take
        /// other information into account when deciding if the configuration
        /// is considered power efficient.
        power_efficient_decoder: Option<bool>,

        /// It represents the total number of frames correctly decoded for this
        /// [RTP stream] that consist of more than one RTP packet. For such
        /// frames the totalAssemblyTime is incremented. The average frame
        /// assembly time can be calculated by dividing the
        /// [`InboundRtpMediaType::Video::total_assembly_time`] with
        /// this value.
        frames_assembled_from_multiple_packets: Option<u32>,

        /// The sum of the time, in seconds, each video frame takes from the
        /// time the first RTP packet is received (reception timestamp) and
        /// to the time the last RTP packet of a frame is received. Only
        /// incremented for frames consisting of more than one RTP packet.
        ///
        /// Given the complexities involved, the time of arrival or the
        /// reception timestamp is measured as close to the network layer as
        /// possible. This metric is not incremented for frames that are not
        /// decoded, i.e.,
        /// [`InboundRtpMediaType::Video::frames_dropped`] or frames
        /// that fail decoding for other reasons (if any). Only incremented
        /// for frames consisting of more than one RTP packet.
        total_assembly_time: Option<Double>,

        /// Represents the cumulative sum of all corruption probability
        /// measurements that have been made for this SSRC, see
        /// [`InboundRtpMediaType::Video::corruption_measurements`]
        /// regarding when this attribute SHOULD be present.
        ///
        /// Each measurement added to
        /// [`InboundRtpMediaType::Video::total_corruption_probability`]
        /// MUST be in the range [0.0, 1.0], where a value of 0.0 indicates
        /// the system has estimated there is no or negligible corruption
        /// present in the processed frame. Similarly a value of 1.0 indicates
        /// there is almost certainly a corruption visible in the processed
        /// frame. A value in between those two indicates there is likely
        /// some corruption visible, but it could for instance have a low
        /// magnitude or be present only in a small portion of the frame.
        total_corruption_probability: Option<Double>,

        /// Represents the cumulative sum of all corruption probability
        /// measurements squared that have been made for this SSRC, see
        /// [`InboundRtpMediaType::Video::corruption_measurements`]
        /// regarding when this attribute SHOULD be present.
        total_squared_corruption_probability: Option<Double>,

        /// When the user agent is able to make a corruption probability
        /// measurement, this counter is incremented for each such
        /// measurement and
        /// [`InboundRtpMediaType::Video::total_corruption_probability`]
        /// and
        /// [`InboundRtpMediaType::Video::total_squared_corruption_probability`]
        /// are aggregated with this measurement and measurement squared
        /// respectively. If the [corruption-detection][0] header extension
        /// is present in the RTP packets, corruption probability measurements
        /// MUST be present.
        ///
        /// [0]: https://tinyurl.com/goog-corruption-detection
        corruption_measurements: Option<u64>,
    },
}

/// [`RtcStat`] fields of [`RtcStatsType::OutboundRtp`] type based on its
/// `kind`.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(tag = "mediaType", rename_all = "camelCase")]
pub enum OutboundRtpMediaType {
    /// Fields when the `kind` is `audio`.
    #[serde(rename_all = "camelCase")]
    Audio {
        /// Total number of samples that have been sent over this RTP stream.
        total_samples_sent: Option<u64>,

        /// Whether the last RTP packet sent contained voice activity or not
        /// based on the presence of the V bit in the extension header.
        voice_activity_flag: Option<bool>,
    },

    /// Fields when the `kind` is `video`.
    #[serde(rename_all = "camelCase")]
    Video(Box<RtcOutboundRtpStreamVideo>),
}

/// Video-specific [`RtcOutboundRtpStreamStats`] part.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcOutboundRtpStreamVideo {
    /// Only exists if a [rid] has been set for this [RTP stream]. If [rid]
    /// is set this value will be present regardless if the RID RTP header
    /// extension has been negotiated.
    ///
    /// [rid]: https://w3.org/TR/webrtc/#dom-rtcrtpcodingparameters-rid
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    pub rid: Option<String>,

    /// This is the index of the encoding that represents this [RTP stream]
    /// in the RTP sender's list of [encodings][0].
    ///
    /// [0]: https://w3.org/TR/webrtc/#dom-rtcrtpsendparameters-encodings
    pub encoding_index: Option<u32>,

    /// This value is increased by the target frame size in bytes every
    /// time a frame has been encoded. The actual frame size may be
    /// bigger or smaller than this number. This value goes up every time
    /// [`OutboundRtpMediaType::Video::frames_encoded`] goes up.
    pub total_encoded_bytes_target: Option<u64>,

    /// Width of the last encoded frame.
    ///
    /// The resolution of the encoded frame may be lower than the media
    /// source (see [RTCVideoSourceStats.width][1]).
    ///
    /// Before the first frame is encoded this attribute is missing.
    ///
    /// [1]: https://w3.org/TR/webrtc-stats#dom-rtcvideosourcestats-width
    pub frame_width: Option<u64>,

    /// Height of the last encoded frame.
    ///
    /// The resolution of the encoded frame may be lower than the media
    /// source (see [RTCVideoSourceStats.height][1]).
    ///
    /// Before the first frame is encoded this attribute is missing.
    ///
    /// [1]: https://w3.org/TR/webrtc-stats#dom-rtcvideosourcestats-height
    pub frame_height: Option<u64>,

    /// Number of encoded frames during the last second.
    ///
    /// This may be lower than the media source frame rate (see
    /// [RTCVideoSourceStats.framesPerSecond][1]).
    ///
    /// [1]: https://tinyurl.com/rrmkrfk
    pub frames_per_second: Option<Double>,

    /// Represents the total number of frames sent on this [RTP stream].
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    pub frames_sent: Option<u32>,

    /// Represents the total number of huge frames sent by this
    /// [RTP stream]. Huge frames, by definition, are frames that
    /// have an encoded size at least 2.5 times the average size of the
    /// frames. The average size of the frames is defined as the target
    /// bitrate per second divided by the target FPS at the time the
    /// frame was encoded. These are usually complex to encode frames
    /// with a lot of changes in the picture. This can be used to
    /// estimate, e.g slide changes in the streamed presentation.
    ///
    /// The multiplier of 2.5 is choosen from analyzing encoded frame
    /// sizes for a sample presentation using WebRTC standalone
    /// implementation. 2.5 is a reasonably large multiplier which still
    /// caused all slide change events to be identified as a huge frames.
    /// It, however, produced 1.4% of false positive slide change
    /// detections which is deemed reasonable.
    ///
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    pub huge_frames_sent: Option<u32>,

    /// It represents the total number of frames successfully encoded
    /// for this RTP media stream.
    pub frames_encoded: Option<u32>,

    /// It represents the total number of key frames, such as key frames
    /// in VP8 [RFC6386] or IDR-frames in H.264 [RFC6184], successfully
    /// encoded for this RTP media stream. This is a subset of
    /// [`OutboundRtpMediaType::Video::frames_encoded`].
    /// [`OutboundRtpMediaType::Video::frames_encoded`] -
    /// [`OutboundRtpMediaType::Video::key_frames_encoded`] gives
    /// you the number of delta frames encoded.
    ///
    /// [RFC6386]: https://rfc-editor.org/rfc/rfc6386
    /// [RFC6184]: https://rfc-editor.org/rfc/rfc6184
    pub key_frames_encoded: Option<u32>,

    /// The sum of the QP values of frames encoded by this sender. The
    /// count of frames is in
    /// [`OutboundRtpMediaType::Video::frames_encoded`].
    ///
    /// The definition of QP value depends on the codec; for VP8, the QP
    /// value is the value carried in the frame header as the syntax element
    /// `y_ac_qi`, and defined in [RFC6386] section 19.2. Its range is
    /// `0..127`.
    ///
    /// Note that the QP value is only an indication of quantizer values
    /// used; many formats have ways to vary the quantizer value within the
    /// frame.
    ///
    /// [RFC6386]: https://rfc-editor.org/rfc/rfc6386
    pub qp_sum: Option<u64>,

    /// Total number of seconds that has been spent encoding the
    /// [`OutboundRtpMediaType::Video::frames_encoded`] frames
    /// of this stream. The average encode time can be calculated by
    /// dividing this value with
    /// [`OutboundRtpMediaType::Video::frames_encoded`]. The time
    /// it takes to encode one frame is the time passed between feeding
    /// the encoder a frame and the encoder returning encoded data for
    /// that frame. This does not include any additional time it may
    /// take to packetize the resulting data.
    pub total_encode_time: Option<Double>,

    /// Count the total number of Full Intra Request (FIR) packets, as
    /// defined in [RFC5104] section 4.3.1, received by this sender.
    /// Does not count the RTCP FIR indicated in [RFC2032] which was
    /// deprecated by [RFC4587].
    ///
    /// [RFC5104]: https://rfc-editor.org/rfc/rfc5104
    /// [RFC2032]: https://rfc-editor.org/rfc/rfc2032
    /// [RFC4587]: https://rfc-editor.org/rfc/rfc4587
    pub fir_count: Option<u32>,

    /// Count the total number of Picture Loss Indication (PLI) packets,
    /// as defined in [RFC4585] section 6.3.1, received by this sender.
    ///
    /// [RFC4585]: https://rfc-editor.org/rfc/rfc4585
    pub pli_count: Option<u32>,

    /// Identifies the encoder implementation used.
    ///
    /// This is useful for diagnosing interoperability issues.
    pub encoder_implementation: Option<String>,

    /// Whether the encoder currently used is considered power efficient
    /// by the user agent. This SHOULD reflect if the configuration results
    /// in hardware acceleration, but the user agent MAY take other
    /// information into account when deciding if the configuration
    /// is considered power efficient.
    pub power_efficient_encoder: Option<bool>,

    /// The current reason for limiting the resolution and/or framerate.
    ///
    /// The implementation reports the most limiting factor. If the
    /// implementation is not able to determine the most limiting factor
    /// because multiple may exist, the reasons MUST be reported in the
    /// following order of priority: `bandwidth`, `cpu`, `other`.
    pub quality_limitation_reason: Option<RtcQualityLimitationReason>,

    /// A record of the total time, in seconds, that this stream has spent
    /// in each quality limitation state. The record includes a mapping
    /// for all [`RtcQualityLimitationReason`] types, including
    /// [`KnownRtcQualityLimitationReason::None`].
    ///
    /// The sum of all entries minus
    /// [`KnownRtcQualityLimitationReason::None`] gives the total time
    /// that the stream has been limited.
    pub quality_limitation_durations:
        BTreeMap<RtcQualityLimitationReason, Double>,

    /// The number of times that the resolution has changed because
    /// we are quality limited (`quality_limitation_reason` has a value
    /// other than [`KnownRtcQualityLimitationReason::None`]). The counter
    /// is initially zero and increases when the resolution goes up or
    /// down. For example, if a `720p` track is sent as `480p` for
    /// some time and then recovers to `720p`, this will have the value `2`.
    pub quality_limitation_resolution_changes: Option<u32>,

    /// Only exists when a [scalability mode][0] is currently configured
    /// for this [RTP stream].
    ///
    /// [0]: https://w3c.github.io/webrtc-svc/#scalabilitymodes*
    /// [RTP stream]: https://w3.org/TR/webrtc-stats/#dfn-rtp-stream
    pub scalability_mode: Option<String>,
}

/// Non-exhaustive version of [`KnownRtcQualityLimitationReason`].
pub type RtcQualityLimitationReason =
    NonExhaustive<KnownRtcQualityLimitationReason>;

/// Describes the reason why the media quality in the stream is currently
/// being reduced by the codec during encoding.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum KnownRtcQualityLimitationReason {
    /// The resolution and/or framerate is not limited.
    #[display("none")]
    None,

    /// The resolution and/or framerate is primarily limited due to CPU load.
    #[display("cpu")]
    Cpu,

    /// The resolution and/or framerate is primarily limited due to congestion
    /// cues during bandwidth estimation. Typical, congestion control
    /// algorithms use inter-arrival time, round-trip time, packet or other
    /// congestion cues to perform bandwidth estimation.
    #[display("bandwidth")]
    Bandwidth,

    /// The resolution and/or framerate is primarily limited for a reason other
    /// than the above.
    #[display("other")]
    Other,
}

/// [`RtcStat`] fields of [`RtcStatsType::MediaSource`] type based on its
/// `kind`.
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum MediaSourceKind {
    /// Represents a video track that is attached to one or more senders. It
    /// is an [`RtcMediaSourceStats`] whose kind is "video".
    #[serde(rename_all = "camelCase")]
    Video {
        /// Width (in pixels) of the last frame originating from the source.
        /// Before a frame has been produced this attribute is missing.
        width: Option<u32>,

        /// Height (in pixels) of the last frame originating from the source.
        /// Before a frame has been produced this attribute is missing.
        height: Option<u32>,

        /// The total number of frames originating from this source.
        frames: Option<u32>,

        /// Number of frames originating from the source, measured during the
        /// last second. For the first second of this object's lifetime this
        /// attribute is missing.
        frames_per_second: Option<Double>,
    },

    /// Represents an audio track that is attached to one or more senders.
    /// It is an [`RtcMediaSourceStats`] whose kind is "audio".
    #[serde(rename_all = "camelCase")]
    Audio {
        /// Represents the audio level of the media source. For audio levels of
        /// remotely sourced tracks, see [`RtcInboundRtpStreamStats`] instead.
        ///
        /// The value is between `0..1` (linear), where `1.0` represents `0`
        /// dBov, `0` represents silence, and `0.5` represents approximately
        /// `6` dBSPL change in the sound pressure level from `0` dBov.
        ///
        /// The audio level is averaged over some small interval, using the
        /// algorithm described under
        /// [`MediaSourceKind::Audio::total_audio_energy`]. The interval used
        /// is implementation-defined.
        audio_level: Option<Double>,

        /// Represents the audio energy of the media source. For audio energy
        /// of remotely sourced tracks, see [`RtcInboundRtpStreamStats`]
        /// instead.
        total_audio_energy: Option<Double>,

        /// Represents the audio duration of the media source. For audio
        /// durations of remotely sourced tracks, see
        /// [`RtcInboundRtpStreamStats`] instead.
        ///
        /// Represents the total duration in seconds of all samples that have
        /// been produced by this source for the lifetime of this stats object.
        /// Can be used with [`MediaSourceKind::Audio::total_audio_energy`]
        /// to compute an average audio level over different intervals.
        total_samples_duration: Option<Double>,

        /// Only exists when the [MediaStreamTrack][0] is sourced from a
        /// microphone where echo cancellation is applied. Calculated in
        /// decibels, as defined in [ECHO] (2012) section 3.14.
        ///
        /// If multiple audio channels are used, the channel of the least
        /// audio energy is considered for any sample.
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
        /// [ECHO]: https://w3.org/TR/webrtc-stats/#bib-echo
        echo_return_loss: Option<Double>,

        /// Only exists when the [MediaStreamTrack][0] is sourced from a
        /// microphone where echo cancellation is applied. Calculated in
        /// decibels, as defined in [ECHO] (2012) section 3.15.
        ///
        /// If multiple audio channels are used, the channel of the least audio
        /// energy is considered for any sample.
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
        /// [ECHO]: https://w3.org/TR/webrtc-stats/#bib-echo
        echo_return_loss_enhancement: Option<Double>,
    },
}

/// Representation of [DOMHighResTimeStamp][1].
///
/// Can be converted to the [`SystemTime`] with millisecond-wise accuracy.
///
/// [`HighResTimeStamp`] type is a [`f64`] and is used to store a time value
/// in milliseconds. This type can be used to describe a discrete point in time
/// or a time interval (the difference in time between two discrete points in
/// time).
///
/// The time, given in milliseconds, should be accurate to 5 µs (microseconds),
/// with the fractional part of the number indicating fractions of a
/// millisecond. However, if the browser is unable to provide a time value
/// accurate to 5 µs (due, for example, to hardware or software constraints),
/// the browser can represent the value as a time in milliseconds accurate to a
/// millisecond. Also note the section below on reduced time precision
/// controlled by browser preferences to avoid timing attacks and
/// fingerprinting.
///
/// Further, if the device or operating system the user agent is running on
/// doesn't have a clock accurate to the microsecond level, they may only be
/// accurate to the millisecond.
///
/// [1]: https://developer.mozilla.org/docs/Web/API/DOMHighResTimeStamp
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct HighResTimeStamp(pub f64);

impl From<HighResTimeStamp> for SystemTime {
    fn from(timestamp: HighResTimeStamp) -> Self {
        Self::UNIX_EPOCH + Duration::from_secs_f64(timestamp.0 / 100.0)
    }
}

impl TryFrom<SystemTime> for HighResTimeStamp {
    type Error = SystemTimeError;

    fn try_from(time: SystemTime) -> Result<Self, Self::Error> {
        Ok(Self(
            time.duration_since(SystemTime::UNIX_EPOCH)?.as_secs_f64() * 100.0,
        ))
    }
}

/// Hashing string representation.
///
/// Some people believe that such behavior is incorrect (but in some programming
/// languages this is a default behavior) due to `NaN`, `Inf` or `-Inf` (they
/// all will have the same hashes).
/// But in the case of [`RtcStat`] received from the client, there should be no
/// such situations, and the hash will always be correct.
impl Hash for HighResTimeStamp {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}

/// Comparison string representations.
///
/// Such implementation is required, so that the results of comparing values and
/// comparing hashes match.
impl PartialEq for HighResTimeStamp {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string().eq(&other.0.to_string())
    }
}

/// Doubleing point numeric type that corresponds to the set of *finite*
/// double-precision 64-bit [IEEE 754] Doubleing point numbers. Web IDL
/// [double] type.
///
/// [double]: https://webidl.spec.whatwg.org/#idl-double
/// [IEEE 754]: https://ieeexplore.ieee.org/document/8766229
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Double(pub f64);

/// Hashing string representation.
///
/// Some people believe that such behavior is incorrect (but in some programming
/// languages this is a default behavior) due to `NaN`, `Inf` or `-Inf` (they
/// all will have the same hashes).
/// But in the case of [`RtcStat`] received from the client, there should be no
/// such situations, and the hash will always be correct.
impl Hash for Double {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}

/// Comparison string representations.
///
/// Such implementation is required, so that the results of comparing values and
/// comparing hashes match.
impl PartialEq for Double {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string().eq(&other.0.to_string())
    }
}
