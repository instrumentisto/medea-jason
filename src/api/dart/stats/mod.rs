use std::ptr;

use libc::c_char;
use medea_client_api_proto::stats::{
    HighResTimeStamp, MediaSourceStats, RtcIceCandidatePairStats,
    RtcIceCandidateStats, RtcInboundRtpStreamStats, RtcOutboundRtpStreamStats,
    RtcRemoteInboundRtpStreamStats, RtcRemoteOutboundRtpStreamStats, RtcStat,
    RtcStatsType, RtcTransportStats, StatId,
};

use crate::{
    api::dart_string_into_rust, platform::utils::NonNullDartValueArgExt,
};

use self::{
    ice_candidate_pair::RTCIceCandidatePairFfiStats,
    inbound_stream::RTCInboundRTPStreamFfiStats,
    media_source::RTCMediaSourceFfiStats,
    outbound_stream::RTCOutboundRTPStreamFfiStats,
    remote_inbound_stream::RTCRemoteInboundRtpStreamFfiStats,
    remote_outbound_stream::RTCRemoteOutboundRtpStreamFfiStats,
    transport::RTCTransportFfiStats, ice_candidate::RTCIceCandidateFfiStats,
};

mod ice_candidate;
mod ice_candidate_pair;
mod inbound_stream;
mod media_source;
mod outbound_stream;
mod remote_inbound_stream;
mod remote_outbound_stream;
mod transport;

/// Returns a boxed pointer to the provided [`RTCFfiStats`].
#[no_mangle]
unsafe extern "C" fn box_foreign_stats(
    val: RTCFfiStats,
) -> ptr::NonNull<RTCFfiStats> {
    ptr::NonNull::from(Box::leak(Box::new(val)))
}

/// Represents the [stats object] constructed by inspecting a specific
/// [monitored object].
///
/// [Full doc on W3C][1].
///
/// [stats object]: https://w3.org/TR/webrtc-stats/#dfn-stats-object
/// [monitored object]: https://w3.org/TR/webrtc-stats/#dfn-monitored-object
/// [1]: https://w3.org/TR/webrtc#rtcstats-dictionary
#[derive(Debug)]
#[repr(C)]
#[allow(missing_copy_implementations)]
pub struct RTCFfiStats {
    /// Unique ID that is associated with the object that was inspected to
    /// produce this [RTCFfiStats] object.
    ///
    /// [RTCFfiStats]: https://w3.org/TR/webrtc#dom-rtcstats
    id: ptr::NonNull<c_char>,

    /// Timestamp associated with this object.
    ///
    /// The time is relative to the UNIX epoch (Jan 1, 1970, UTC).
    ///
    /// For statistics that came from a remote source (e.g., from received RTCP
    /// packets), timestamp represents the time at which the information
    /// arrived at the local endpoint. The remote timestamp can be found in an
    /// additional field in an [`RtcStat`]-derived dictionary, if applicable.
    timestamp_us: i64,

    /// Actual stats of this [`RTCFfiStats`].
    ///
    /// All possible stats are described in the [`RTCFfiStatsType`] enum.
    kind: ptr::NonNull<RTCFfiStatsType>,
}

// Type-erased value that can be transferred via Ffi boundaries to/from Dart.
#[allow(missing_copy_implementations, dead_code)] // not trivially copyable
#[derive(Debug)]
#[repr(u8)]
/// All known types of [`RtcStat`]s.
///
/// [List of all RTCStats types on W3C][1].
///
/// [1]: https://w3.org/TR/webrtc-stats/#rtctatstype-%2A
/// [`RtcStat`]: super::RtcStat
enum RTCFfiStatsType {
    /// Disabled or unknown variants.
    Unimplemented,
    /// Transport statistics related to the [RTCPeerConnection] object.
    ///
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    Transport(ptr::NonNull<RTCTransportFfiStats>),
    /// Statistics for the media produced by a [MediaStreamTrack][1] that
    /// is currently attached to an [RTCRtpSender]. This reflects
    /// the media that is fed to the encoder after [getUserMedia]
    /// constraints have been applied (i.e. not the raw media
    /// produced by the camera).
    ///
    /// [RTCRtpSender]: https://w3.org/TR/webrtc#rtcrtpsender-interface
    /// [getUserMedia]: https://tinyurl.com/sngpyr6
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    MediaSource(ptr::NonNull<RTCMediaSourceFfiStats>),
    /// ICE candidate statistics related to the [RTCIceTransport]
    /// objects.
    ///
    /// A local candidate is [deleted][1] when the [RTCIceTransport] does
    /// an ICE restart, and the candidate is no longer a member of
    /// any non-deleted candidate pair.
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
    /// [1]: https://w3.org/TR/webrtc-stats/#dfn-deleted
    IceCandidate(ptr::NonNull<RTCIceCandidateFfiStats>),
    /// Statistics for an outbound [RTP] stream that is currently sent with
    /// [RTCPeerConnection] object.
    ///
    /// When there are multiple [RTP] streams connected to the same sender,
    /// such as when using simulcast or RTX, there will be one
    /// [`RtcOutboundRtpStreamStats`] per RTP stream, with distinct values
    /// of the `ssrc` attribute, and all these senders will have a
    /// reference to the same "sender" object (of type
    /// [RTCAudioSenderStats][1] or [RTCVideoSenderStats][2]) and
    /// "track" object (of type
    /// [RTCSenderAudioTrackAttachmentStats][3] or
    /// [RTCSenderVideoTrackAttachmentStats][4]).
    ///
    /// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    /// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcaudiosenderstats
    /// [2]: https://w3.org/TR/webrtc-stats/#dom-rtcvideosenderstats
    /// [3]: https://tinyurl.com/sefa5z4
    /// [4]: https://tinyurl.com/rkuvpl4
    OutboundRTPStream(ptr::NonNull<RTCOutboundRTPStreamFfiStats>),
    /// Statistics for an inbound [RTP] stream that is currently received
    /// with [RTCPeerConnection] object.
    ///
    /// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    InboundRTPStream(ptr::NonNull<RTCInboundRTPStreamFfiStats>),
    /// ICE candidate pair statistics related to the [RTCIceTransport]
    /// objects.
    ///
    /// A candidate pair that is not the current pair for a transport is
    /// [deleted][1] when the [RTCIceTransport] does an ICE restart, at the
    /// time the state changes to `new`.
    ///
    /// The candidate pair that is the current pair for a transport is
    /// deleted after an ICE restart when the [RTCIceTransport]
    /// switches to using a candidate pair generated from the new
    /// candidates; this time doesn't correspond to any other
    /// externally observable event.
    ///
    /// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
    /// [1]: https://w3.org/TR/webrtc-stats/#dfn-deleted
    IceCandidatePair(ptr::NonNull<RTCIceCandidatePairFfiStats>),
    /// Statistics for the remote endpoint's inbound [RTP] stream
    /// corresponding to an outbound stream that is currently sent with
    /// [RTCPeerConnection] object.
    ///
    /// It is measured at the remote endpoint and reported in a RTCP
    /// Receiver Report (RR) or RTCP Extended Report (XR).
    ///
    /// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    RemoteInboundRTPStream(ptr::NonNull<RTCRemoteInboundRtpStreamFfiStats>),
    /// Statistics for the remote endpoint's outbound [RTP] stream
    /// corresponding to an inbound stream that is currently received with
    /// [RTCPeerConnection] object.
    ///
    /// It is measured at the remote endpoint and reported in an RTCP
    /// Sender Report (SR).
    ///
    /// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    RemoteOutboundRTPStream(ptr::NonNull<RTCRemoteOutboundRtpStreamFfiStats>),
}

impl From<RTCFfiStatsType> for RtcStatsType {
    fn from(stats: RTCFfiStatsType) -> Self {
        match stats {
            RTCFfiStatsType::MediaSource(stats) => {
                let stats = unsafe { stats.unbox() };
                Self::MediaSource(Box::new(MediaSourceStats::from(stats)))
            }
            RTCFfiStatsType::OutboundRTPStream(stats) => {
                let stats = unsafe { stats.unbox() };
                Self::OutboundRtp(Box::new(RtcOutboundRtpStreamStats::from(
                    stats,
                )))
            }
            RTCFfiStatsType::InboundRTPStream(stats) => {
                let stats = unsafe { stats.unbox() };
                Self::InboundRtp(Box::new(RtcInboundRtpStreamStats::from(
                    stats,
                )))
            }
            RTCFfiStatsType::IceCandidatePair(stats) => {
                let stats = unsafe { stats.unbox() };
                Self::CandidatePair(Box::new(RtcIceCandidatePairStats::from(
                    stats,
                )))
            }
            RTCFfiStatsType::Transport(stats) => {
                let stats = unsafe { stats.unbox() };
                Self::Transport(Box::new(RtcTransportStats::from(stats)))
            }
            RTCFfiStatsType::RemoteInboundRTPStream(stats) => {
                let stats = unsafe { stats.unbox() };
                Self::RemoteInboundRtp(Box::new(
                    RtcRemoteInboundRtpStreamStats::from(stats),
                ))
            }
            RTCFfiStatsType::RemoteOutboundRTPStream(stats) => {
                let stats = unsafe { stats.unbox() };
                Self::RemoteOutboundRtp(Box::new(
                    RtcRemoteOutboundRtpStreamStats::from(stats),
                ))
            }
            RTCFfiStatsType::IceCandidate(stats) => {
                let stats = unsafe { stats.unbox() };
                match stats {
                    RTCIceCandidateFfiStats::Remote(candidate) => {
                        let unbox = unsafe { candidate.unbox() };
                        Self::RemoteCandidate(Box::new(
                            RtcIceCandidateStats::from(unbox),
                        ))
                    }
                    RTCIceCandidateFfiStats::Local(candidate) => {
                        let unbox = unsafe { candidate.unbox() };
                        Self::LocalCandidate(Box::new(
                            RtcIceCandidateStats::from(unbox),
                        ))
                    }
                }
            }
            RTCFfiStatsType::Unimplemented => Self::Other,
        }
    }
}

impl From<RTCFfiStats> for RtcStat {
    #[allow(clippy::cast_precision_loss)]
    fn from(stats: RTCFfiStats) -> Self {
        let id = unsafe { dart_string_into_rust(stats.id) };
        let time = HighResTimeStamp(stats.timestamp_us as f64);
        let kind = unsafe { stats.kind.unbox() };
        let stats_type = RtcStatsType::from(kind);
        Self {
            id: StatId(id),
            timestamp: time,
            stats: stats_type,
        }
    }
}
