use std::ptr;

use medea_client_api_proto::stats::{
    HighResTimeStamp, RtcRemoteOutboundRtpStreamStats,
};

use crate::{api::DartValueArg, platform::utils::NonNullDartValueArgExt};

/// Statistics for the remote endpoint's outbound [RTP] stream corresponding
/// to an inbound stream that is currently received with [RTCPeerConnection]
/// object.
///
/// It is measured at the remote endpoint and reported in an RTCP Sender Report
/// (SR).
///
/// [`RtcStatsType::RemoteOutboundRtp`] variant.
///
/// [Full doc on W3C][1].
///
/// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
/// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
/// [1]: https://w3.org/TR/webrtc-stats/#remoteoutboundrtpstats-dict%2A
#[derive(Debug)]
#[repr(C)]
#[allow(missing_copy_implementations)]
pub struct RTCRemoteOutboundRtpStreamFfiStats {
    /// [`localId`] is used for looking up the local
    /// [RTCInboundRtpStreamFfiStats][1] object for the same SSRC.
    ///
    /// [`localId`]: https://tinyurl.com/vu9tb2e
    /// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats
    local_id: ptr::NonNull<DartValueArg<Option<String>>>,

    /// [`remoteTimestamp`] (as [HIGHRES-TIME]) is the remote timestamp at
    /// which these statistics were sent by the remote endpoint. This
    /// differs from timestamp, which represents the time at which the
    /// statistics were generated or received by the local endpoint. The
    /// [`remoteTimestamp`], if present, is derived from the NTP timestamp
    /// in an RTCP Sender Report (SR) block, which reflects the remote
    /// endpoint's clock. That clock may not be synchronized with the local
    /// clock.
    ///
    /// [`remoteTimestamp`]: https://tinyurl.com/rzlhs87
    /// [HIGRES-TIME]: https://w3.org/TR/webrtc-stats/#bib-highres-time
    remote_timestamp: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Total number of RTCP SR blocks sent for this SSRC.
    reports_sent: ptr::NonNull<DartValueArg<Option<u64>>>,
}

#[allow(clippy::fallible_impl_from)]
impl From<RTCRemoteOutboundRtpStreamFfiStats>
    for RtcRemoteOutboundRtpStreamStats
{
    fn from(stats: RTCRemoteOutboundRtpStreamFfiStats) -> Self {
        Self {
            local_id: Option::try_from(unsafe { stats.local_id.unbox() })
                .unwrap(),
            remote_timestamp: Option::try_from(unsafe {
                stats.remote_timestamp.unbox()
            })
            .unwrap()
            .map(HighResTimeStamp),
            reports_sent: Option::try_from(unsafe {
                stats.reports_sent.unbox()
            })
            .unwrap(),
        }
    }
}
