use std::ptr;

use medea_client_api_proto::stats::{Float, RtcRemoteInboundRtpStreamStats};

use crate::{api::DartValueArg, platform::utils::NonNullDartValueArgExt};

/// Statistics for the remote endpoint's inbound [RTP] stream corresponding
/// to an outbound stream that is currently sent with [RTCPeerConnection]
/// object.
///
/// It is measured at the remote endpoint and reported in a RTCP Receiver
/// Report (RR) or RTCP Extended Report (XR).
///
/// [`RtcStatsType::RemoteInboundRtp`] variant.
///
/// [Full doc on W3C][1].
///
/// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
/// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
/// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats
#[derive(Debug)]
#[repr(C)]
#[allow(missing_copy_implementations)]
pub struct RTCRemoteInboundRtpStreamFfiStats {
    /// [`localId`] is used for looking up the local
    /// [RTCOutboundRtpStreamFfiStats] object for the same SSRC.
    ///
    /// [`localId`]: https://tinyurl.com/r8uhbo9
    /// [RTCOutBoundRtpStreamFfiStats]: https://tinyurl.com/r6f5vqg
    local_id: ptr::NonNull<DartValueArg<Option<String>>>,

    /// Packet jitter measured in seconds for this SSRC.
    jitter: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Estimated round trip time for this SSRC based on
    /// the RTCP timestamps in
    /// the RTCP Receiver Report (RR) and measured in seconds.
    /// Calculated as defined in [Section 6.4.1 of RFC 3550][1].
    /// If no RTCP Receiver Report
    /// is received with a DLSR value other than 0, the round trip time is
    /// left undefined.
    ///
    /// [1]: https://tools.ietf.org/html/rfc3550#section-6.4.1
    round_trip_time: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Fraction packet loss reported for this SSRC.
    /// Calculated as defined in
    /// [Section 6.4.1 of RFC 3550][1] and [Appendix A.3][2].
    ///
    /// [1]: https://tools.ietf.org/html/rfc3550#section-6.4.1
    /// [2]: https://tools.ietf.org/html/rfc3550#appendix-A.3
    fraction_lost: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Total number of RTCP RR blocks received for this SSRC.
    reports_received: ptr::NonNull<DartValueArg<Option<u64>>>,

    /// Total number of RTCP RR blocks received for this SSRC that contain a
    /// valid round trip time. This counter will increment if the
    /// [`roundTripTime`] is undefined.
    ///
    /// [`roundTripTime`]: https://tinyurl.com/ssg83hq
    round_trip_time_measurements: ptr::NonNull<DartValueArg<Option<i32>>>,
}

#[allow(clippy::fallible_impl_from)]
impl From<RTCRemoteInboundRtpStreamFfiStats>
    for RtcRemoteInboundRtpStreamStats
{
    fn from(stats: RTCRemoteInboundRtpStreamFfiStats) -> Self {
        Self {
            local_id: Option::try_from(unsafe { stats.local_id.unbox() })
                .unwrap(),
            jitter: Option::<f64>::try_from(unsafe { stats.jitter.unbox() })
                .unwrap()
                .map(Float),
            round_trip_time: Option::<f64>::try_from(unsafe {
                stats.round_trip_time.unbox()
            })
            .unwrap()
            .map(Float),
            fraction_lost: Option::<f64>::try_from(unsafe {
                stats.fraction_lost.unbox()
            })
            .unwrap()
            .map(Float),
            reports_received: Option::<u64>::try_from(unsafe {
                stats.reports_received.unbox()
            })
            .unwrap(),
            round_trip_time_measurements: Option::<i32>::try_from(unsafe {
                stats.round_trip_time_measurements.unbox()
            })
            .unwrap()
            .map(|v| Float(f64::from(v))),
        }
    }
}
