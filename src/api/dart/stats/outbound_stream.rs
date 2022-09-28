use std::ptr;

use libc::c_void;
use medea_client_api_proto::stats::{
    RtcOutboundRtpStreamMediaType, RtcOutboundRtpStreamStats,
};

use crate::{api::DartValueArg, platform::utils::NonNullDartValueArgExt};

/// Statistics for an outbound [RTP] stream that is currently sent with this
/// [RTCPeerConnection] object.
///
/// When there are multiple [RTP] streams connected to the same sender, such
/// as when using simulcast or RTX, there will be one
/// [`RtcOutboundRtpStreamStats`] per RTP stream, with distinct values of
/// the `ssrc` attribute, and all these senders will have a reference to
/// the same "sender" object (of type [RTCAudioSenderStats][1] or
/// [RTCVideoSenderStats][2]) and "track" object (of type
/// [RTCSenderAudioTrackAttachmentStats][3] or
/// [RTCSenderVideoTrackAttachmentStats][4]).
///
/// [`RtcStatsType::OutboundRtp`] variant.
///
/// [Full doc on W3C][5].
///
/// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
/// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
/// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcaudiosenderstats
/// [2]: https://w3.org/TR/webrtc-stats/#dom-rtcvideosenderstats
/// [3]: https://tinyurl.com/sefa5z4
/// [4]: https://tinyurl.com/rkuvpl4
/// [5]: https://w3.org/TR/webrtc-stats/#outboundrtpstats-dict%2A
#[allow(missing_copy_implementations)]
#[derive(Debug)]
#[repr(C)]
pub struct RTCOutboundRTPStreamFfiStats {
    /// ID of the stats object representing the current track attachment
    /// to the sender of this stream.
    track_id: ptr::NonNull<DartValueArg<Option<String>>>,

    /// Total number of bytes sent for this SSRC.
    bytes_sent: ptr::NonNull<DartValueArg<Option<u64>>>,

    /// Total number of RTP packets sent for this SSRC.
    packets_sent: ptr::NonNull<DartValueArg<Option<u32>>>,

    /// ID of the stats object representing the track currently
    /// attached to the sender of this stream.
    media_source_id: ptr::NonNull<DartValueArg<Option<String>>>,

    /// Fields which should be in the [`RtcStat`] based on `mediaType`.
    media_type: ptr::NonNull<
        DartValueArg<ptr::NonNull<RTCOutboundRTPStreamFfiStatsMediaType>>,
    >,
}

/// Stats when [`RTCOutboundRTPStreamFfiStats.media_type`] is `video`.
#[allow(missing_copy_implementations)]
#[derive(Debug)]
#[repr(C)]
struct RTCOutboundRTPStreamFfiStatsVideo {
    /// Width of the last encoded frame.
    ///
    /// The resolution of the encoded frame may be lower than the media
    /// source (see [RTCVideoSourceFfiStats.width][1]).
    ///
    /// Before the first frame is encoded this attribute is missing.
    ///
    /// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats-width
    frame_width: ptr::NonNull<DartValueArg<Option<u32>>>,

    /// Height of the last encoded frame.
    ///
    /// The resolution of the encoded frame may be lower than the media
    /// source (see [RTCVideoSourceFfiStats.height][1]).
    ///
    /// Before the first frame is encoded this attribute is missing.
    ///
    /// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats-height
    frame_height: ptr::NonNull<DartValueArg<Option<u32>>>,

    /// Number of encoded frames during the last second.
    ///
    /// This may be lower than the media source frame rate (see
    /// [RTCVideoSourceFfiStats.framesPerSecond][1]).
    ///
    /// [1]: https://tinyurl.com/rrmkrfk
    frames_per_second: ptr::NonNull<DartValueArg<Option<f64>>>,
}

/// Stats when [`RTCOutboundRTPStreamFfiStats.media_type`] is `audio`.
#[allow(missing_copy_implementations)]
#[derive(Debug)]
#[repr(C)]
struct RTCOutboundRTPStreamFfiStatsAudio {
    /// Total number of samples that have been sent over this RTP stream.
    total_samples_sent: ptr::NonNull<DartValueArg<Option<u64>>>,
    /// Whether the last RTP packet sent contained voice activity or not
    /// based on the presence of the V bit in the extension header.
    voice_activity_flag: ptr::NonNull<DartValueArg<Option<bool>>>,
}

// Type-erased value that can be transferred via Ffi boundaries to/from Dart.
#[allow(missing_copy_implementations, dead_code)] // not trivially copyable
#[derive(Debug)]
#[repr(u8)]
/// [`RtcStat`] fields of [`RtcStatsType::OutboundRtp`] type based on
/// `mediaType`.
enum RTCOutboundRTPStreamFfiStatsMediaType {
    /// Stats when `media_type` is `video`.
    Video(ptr::NonNull<RTCOutboundRTPStreamFfiStatsVideo>),
    /// Stats when `media_type` is `audio`.
    Audio(ptr::NonNull<RTCOutboundRTPStreamFfiStatsAudio>),
}

#[allow(clippy::fallible_impl_from)]
impl From<RTCOutboundRTPStreamFfiStatsMediaType>
    for RtcOutboundRtpStreamMediaType
{
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn from(stats: RTCOutboundRTPStreamFfiStatsMediaType) -> Self {
        match stats {
            RTCOutboundRTPStreamFfiStatsMediaType::Video(video) => {
                let video = unsafe { video.unbox() };
                Self::Video {
                    frame_width: Option::<u32>::try_from(unsafe {
                        video.frame_width.unbox()
                    })
                    .unwrap()
                    .map(u64::from),
                    frame_height: Option::<u32>::try_from(unsafe {
                        video.frame_height.unbox()
                    })
                    .unwrap()
                    .map(u64::from),
                    frames_per_second: Option::<f64>::try_from(unsafe {
                        video.frames_per_second.unbox()
                    })
                    .unwrap()
                    .map(|v| v as u64),
                }
            }
            RTCOutboundRTPStreamFfiStatsMediaType::Audio(audio) => {
                let audio = unsafe { audio.unbox() };
                Self::Audio {
                    total_samples_sent: Option::<u64>::try_from(unsafe {
                        audio.total_samples_sent.unbox()
                    })
                    .unwrap(),
                    voice_activity_flag: Option::<bool>::try_from(unsafe {
                        audio.voice_activity_flag.unbox()
                    })
                    .unwrap(),
                }
            }
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<RTCOutboundRTPStreamFfiStats> for RtcOutboundRtpStreamStats {
    fn from(stats: RTCOutboundRTPStreamFfiStats) -> Self {
        let kind = unsafe { stats.media_type.unbox() };
        let kind: Option<ptr::NonNull<c_void>> =
            Option::try_from(kind).unwrap();
        let kind: Option<RTCOutboundRTPStreamFfiStatsMediaType> =
            kind.map(|v| unsafe { v.cast().unbox() });
        let media_type = kind.map(RtcOutboundRtpStreamMediaType::from);
        Self {
            track_id: Option::try_from(unsafe { stats.track_id.unbox() })
                .unwrap(),
            media_type,
            bytes_sent: Option::<u64>::try_from(unsafe {
                stats.bytes_sent.unbox()
            })
            .unwrap(),
            packets_sent: Option::<u32>::try_from(unsafe {
                stats.packets_sent.unbox()
            })
            .unwrap()
            .map(u64::from),
            media_source_id: Option::try_from(unsafe {
                stats.media_source_id.unbox()
            })
            .unwrap(),
        }
    }
}
