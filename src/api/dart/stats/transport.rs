use std::ptr;

use medea_client_api_proto::stats::RtcTransportStats;

use crate::{
    api::DartValueArg,
    platform::{rtc_stats::IceRole, utils::NonNullDartValueArgExt},
};

/// Representation of the stats corresponding to an [RTCDtlsTransport] and its
/// underlying [RTCIceTransport].
///
/// When RTCP multiplexing is used, one transport is used for both RTP and RTCP.
/// Otherwise, RTP and RTCP will be sent on separate transports, and
/// `rtcpTransportStatsId` can be used to pair the resulting
/// [`RtcTransportStats`] objects. Additionally, when bundling is used, a single
/// transport will be used for all [MediaStreamTrack][2]s in the bundle group.
/// If bundling is not used, different [MediaStreamTrack][2]s will use different
/// transports. RTCP multiplexing and bundling are described in [WebRTC].
///
/// [`RtcStatsType::Transport`] variant.
///
/// [Full doc on W3C][1].
///
/// [RTCDtlsTransport]: https://w3.org/TR/webrtc#dom-rtcdtlstransport
/// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
/// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
/// [WebRTC]: https://w3.org/TR/webrtc
/// [1]: https://w3.org/TR/webrtc-stats/#transportstats-dict%2A
/// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
#[derive(Debug)]
#[repr(C)]
#[allow(missing_copy_implementations)]
pub struct RTCTransportFfiStats {
    /// Total number of packets sent over this transport.
    packets_sent: ptr::NonNull<DartValueArg<Option<i32>>>,

    /// Total number of packets received on this transport.
    packets_received: ptr::NonNull<DartValueArg<Option<i32>>>,

    /// Total number of payload bytes sent on this [RTCPeerConnection], i.e.
    /// not including headers or padding.
    ///
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    bytes_sent: ptr::NonNull<DartValueArg<Option<i32>>>,

    /// Total number of bytes received on this [RTCPeerConnection], i.e. not
    /// including headers or padding.
    ///
    /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    bytes_received: ptr::NonNull<DartValueArg<Option<i32>>>,

    /// Set to the current value of the [`role` attribute][1] of the
    /// [underlying RTCDtlsTransport's `transport`][2].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-icetransport-role
    /// [2]: https://w3.org/TR/webrtc#dom-rtcdtlstransport-icetransport
    ice_role: ptr::NonNull<DartValueArg<Option<i32>>>,
}

#[allow(clippy::fallible_impl_from)]
impl From<RTCTransportFfiStats> for RtcTransportStats {
    #[allow(clippy::cast_sign_loss)]
    fn from(stats: RTCTransportFfiStats) -> Self {
        let role_index =
            Option::<i32>::try_from(unsafe { stats.ice_role.unbox() }).unwrap();
        Self {
            packets_sent: Option::<i32>::try_from(unsafe {
                stats.packets_sent.unbox()
            })
            .unwrap()
            .map(|v| v as u64),
            packets_received: Option::<i32>::try_from(unsafe {
                stats.packets_received.unbox()
            })
            .unwrap()
            .map(|v| v as u64),
            bytes_sent: Option::<i32>::try_from(unsafe {
                stats.bytes_sent.unbox()
            })
            .unwrap()
            .map(|v| v as u64),
            bytes_received: Option::<i32>::try_from(unsafe {
                stats.bytes_received.unbox()
            })
            .unwrap()
            .map(|v| v as u64),
            ice_role: role_index
                .and_then(|v| IceRole::try_from(i64::from(v)).ok())
                .map(medea_client_api_proto::stats::IceRole::from),
        }
    }
}
