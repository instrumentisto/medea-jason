use std::ptr;

use medea_client_api_proto::stats::{
    Float, HighResTimeStamp, RtcInboundRtpStreamMediaType,
    RtcInboundRtpStreamStats,
};

use crate::{api::DartValueArg, platform::utils::NonNullDartValueArgExt};

#[derive(Debug)]
#[repr(C)]
#[allow(missing_copy_implementations)]
pub struct RTCInboundRTPStreamFfiStats {
    /// ID of the stats object representing the receiving track.
    remote_id: ptr::NonNull<DartValueArg<Option<String>>>,

    /// Total number of bytes received for this SSRC.
    bytes_received: ptr::NonNull<DartValueArg<Option<u64>>>,

    /// Total number of RTP data packets received for this SSRC.
    packets_received: ptr::NonNull<DartValueArg<Option<u32>>>,

    /// Total number of RTP data packets for this SSRC that have been lost
    /// since the beginning of reception.
    ///
    /// This number is defined to be the number of packets expected less the
    /// number of packets actually received, where the number of packets
    /// received includes any which are late or duplicates.
    /// Thus, packets that arrive late are not counted as lost,
    /// and the loss __may be negative__
    /// if there are duplicates.
    packets_lost: ptr::NonNull<DartValueArg<Option<u64>>>,

    /// Packet jitter measured in seconds for this SSRC.
    jitter: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Total number of seconds that have been spent decoding the
    /// [`framesDecoded`] frames of this stream.
    ///
    /// The average decode time can be calculated by dividing this value
    /// with [`framesDecoded`].
    /// The time it takes to decode one frame is the time
    /// passed between feeding the decoder a frame and the decoder returning
    /// decoded data for that frame.
    ///
    /// [`framesDecoded`]: https://tinyurl.com/srfwrwt
    total_decode_time: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Total number of audio samples or video frames
    /// that have come out of the
    /// jitter buffer (increasing [`jitterBufferDelay`]).
    ///
    /// [`jitterBufferDelay`]: https://tinyurl.com/qvoojt5
    jitter_buffer_emitted_count: ptr::NonNull<DartValueArg<Option<u64>>>,

    /// Fields which should be in the [`RtcStat`] based on `mediaType`.
    media_type: ptr::NonNull<RTCInboundRTPStreamMediaType>,
}

#[derive(Debug)]
#[repr(C)]
#[allow(missing_copy_implementations)]
struct RTCInboundRTPStreamFfiStatsAudio {
    /// Indicator whether the last RTP packet whose frame was delivered to
    /// the [RTCRtpReceiver]'s [MediaStreamTrack][1] for playout contained
    /// voice activity or not based on the presence of the V bit in the
    /// extension header, as defined in [RFC 6464].
    ///
    /// [RTCRtpReceiver]: https://w3.org/TR/webrtc#rtcrtpreceiver-interface
    /// [RFC 6464]: https://tools.ietf.org/html/rfc6464#page-3
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    voice_activity_flag: ptr::NonNull<DartValueArg<Option<bool>>>,

    /// Total number of samples that have been received on this RTP stream.
    /// This includes [`concealedSamples`].
    ///
    /// [`concealedSamples`]: https://tinyurl.com/s6c4qe4
    total_samples_received: ptr::NonNull<DartValueArg<Option<u64>>>,

    /// Total number of samples that are concealed samples.
    ///
    /// A concealed sample is a sample that was replaced with synthesized
    /// samples generated locally before being played out.
    /// Examples of samples that have to be concealed are samples from lost
    /// packets (reported in [`packetsLost`]) or samples from packets that
    /// arrive too late to be played out (reported in
    /// [`packetsDiscarded`]).
    ///
    /// [`packetsLost`]: https://tinyurl.com/u2gq965
    /// [`packetsDiscarded`]: https://tinyurl.com/yx7qyox3
    concealed_samples: ptr::NonNull<DartValueArg<Option<u64>>>,

    /// Total number of concealed samples inserted that are "silent".
    ///
    /// Playing out silent samples results in silence or comfort noise.
    /// This is a subset of [`concealedSamples`].
    ///
    /// [`concealedSamples`]: https://tinyurl.com/s6c4qe4
    silent_concealed_samples: ptr::NonNull<DartValueArg<Option<u64>>>,

    /// Audio level of the receiving track.
    audio_level: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Audio energy of the receiving track.
    total_audio_energy: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Audio duration of the receiving track.
    ///
    /// For audio durations of tracks attached locally, see
    /// [RTCAudioSourceFfiStats][1] instead.
    ///
    /// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
    total_samples_duration: ptr::NonNull<DartValueArg<Option<f64>>>,
}

#[derive(Debug)]
#[repr(C)]
#[allow(missing_copy_implementations)]
struct RTCInboundRTPStreamFfiStatsVideo {
    /// Total number of frames correctly decoded for this RTP stream, i.e.
    /// frames that would be displayed if no frames are dropped.
    frames_decoded: ptr::NonNull<DartValueArg<Option<u32>>>,

    /// Total number of key frames, such as key frames in VP8 [RFC 6386] or
    /// IDR-frames in H.264 [RFC 6184], successfully decoded for this RTP
    /// media stream.
    ///
    /// This is a subset of [`framesDecoded`].
    /// [`framesDecoded`] - [`keyFramesDecoded`] gives you the number of
    /// delta frames decoded.
    ///
    /// [RFC 6386]: https://w3.org/TR/webrtc-stats/#bib-rfc6386
    /// [RFC 6184]: https://w3.org/TR/webrtc-stats/#bib-rfc6184
    /// [`framesDecoded`]: https://tinyurl.com/srfwrwt
    /// [`keyFramesDecoded`]: https://tinyurl.com/qtdmhtm
    key_frames_decoded: ptr::NonNull<DartValueArg<Option<u32>>>,

    /// Width of the last decoded frame.
    ///
    /// Before the first frame is decoded this attribute is missing.
    frame_width: ptr::NonNull<DartValueArg<Option<u32>>>,

    /// Height of the last decoded frame.
    ///
    /// Before the first frame is decoded this attribute is missing.
    frame_height: ptr::NonNull<DartValueArg<Option<u32>>>,

    /// Sum of the interframe delays in seconds between consecutively
    /// decoded frames, recorded just after a frame has been decoded.
    total_inter_frame_delay: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Number of decoded frames in the last second.
    frames_per_second: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Total number of Full Intra Request (FIR) packets sent by this
    /// receiver.
    fir_count: ptr::NonNull<DartValueArg<Option<u32>>>,

    /// Total number of Picture Loss Indication (PLI) packets sent by this
    /// receiver.
    pli_count: ptr::NonNull<DartValueArg<Option<u32>>>,

    /// Number of concealment events.
    ///
    /// This counter increases every time a concealed sample is synthesized
    /// after a non-concealed sample. That is, multiple consecutive
    /// concealed samples will increase the [`concealedSamples`] count
    /// multiple times but is a single concealment event.
    ///
    /// [`concealedSamples`]: https://tinyurl.com/s6c4qe4
    concealment_events: ptr::NonNull<DartValueArg<Option<u64>>>,

    /// Total number of complete frames received on this RTP stream.
    ///
    /// This metric is incremented when the complete frame is received.
    frames_received: ptr::NonNull<DartValueArg<Option<i32>>>,

    /// Total number of Slice Loss Indication (SLI) packets sent by this
    /// receiver.
    sli_count: ptr::NonNull<DartValueArg<Option<u32>>>,
}

/// Type-erased value that can be transferred via Ffi boundaries to/from Dart.
#[allow(missing_copy_implementations, dead_code)] // not trivially copyable
#[derive(Debug)]
#[repr(u8)]
enum RTCInboundRTPStreamMediaType {
    /// Stats when media type of [`RTCInboundRTPStreamFfiStats`] is video.
    Video(ptr::NonNull<RTCInboundRTPStreamFfiStatsVideo>),
    /// Stats when media type of [`RTCInboundRTPStreamFfiStats`] is audio.
    Audio(ptr::NonNull<RTCInboundRTPStreamFfiStatsAudio>),
}

#[allow(clippy::fallible_impl_from)]
impl From<RTCInboundRTPStreamFfiStats> for RtcInboundRtpStreamStats {
    #[allow(clippy::cast_possible_wrap)]
    fn from(stats: RTCInboundRTPStreamFfiStats) -> Self {
        let kind = unsafe { stats.media_type.unbox() };
        let media_specific_stats = RtcInboundRtpStreamMediaType::from(kind);
        Self {
            track_id: Option::try_from(unsafe { stats.remote_id.unbox() })
                .unwrap(),
            media_specific_stats,
            bytes_received: Option::try_from(unsafe {
                stats.bytes_received.unbox()
            })
            .unwrap(),
            packets_received: Option::<u32>::try_from(unsafe {
                stats.packets_received.unbox()
            })
            .unwrap()
            .map(u64::from),
            packets_lost: Option::<u64>::try_from(unsafe {
                stats.packets_lost.unbox()
            })
            .unwrap()
            .map(|v| v as i64),
            jitter: Option::try_from(unsafe { stats.jitter.unbox() })
                .unwrap()
                .map(Float),
            total_decode_time: Option::try_from(unsafe {
                stats.total_decode_time.unbox()
            })
            .unwrap()
            .map(HighResTimeStamp),
            jitter_buffer_emitted_count: Option::try_from(unsafe {
                stats.jitter_buffer_emitted_count.unbox()
            })
            .unwrap(),
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<RTCInboundRTPStreamMediaType> for RtcInboundRtpStreamMediaType {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::too_many_lines,
        clippy::cast_sign_loss
    )]
    fn from(stats: RTCInboundRTPStreamMediaType) -> Self {
        match stats {
            RTCInboundRTPStreamMediaType::Audio(audio) => {
                let audio = unsafe { audio.unbox() };
                Self::Audio {
                    voice_activity_flag: Option::try_from(unsafe {
                        audio.voice_activity_flag.unbox()
                    })
                    .unwrap(),
                    total_samples_received: Option::try_from(unsafe {
                        audio.total_samples_received.unbox()
                    })
                    .unwrap(),
                    concealed_samples: Option::try_from(unsafe {
                        audio.concealed_samples.unbox()
                    })
                    .unwrap(),
                    silent_concealed_samples: Option::try_from(unsafe {
                        audio.silent_concealed_samples.unbox()
                    })
                    .unwrap(),
                    audio_level: Option::try_from(unsafe {
                        audio.audio_level.unbox()
                    })
                    .unwrap()
                    .map(Float),
                    total_audio_energy: Option::try_from(unsafe {
                        audio.total_audio_energy.unbox()
                    })
                    .unwrap()
                    .map(Float),
                    total_samples_duration: Option::try_from(unsafe {
                        audio.total_samples_duration.unbox()
                    })
                    .unwrap()
                    .map(HighResTimeStamp),
                }
            }
            RTCInboundRTPStreamMediaType::Video(video) => {
                let video = unsafe { video.unbox() };
                Self::Video {
                    frames_decoded: Option::<u32>::try_from(unsafe {
                        video.frames_decoded.unbox()
                    })
                    .unwrap()
                    .map(u64::from),
                    key_frames_decoded: Option::<u32>::try_from(unsafe {
                        video.key_frames_decoded.unbox()
                    })
                    .unwrap()
                    .map(u64::from),
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
                    total_inter_frame_delay: Option::try_from(unsafe {
                        video.total_inter_frame_delay.unbox()
                    })
                    .unwrap()
                    .map(Float),
                    frames_per_second: Option::<f64>::try_from(unsafe {
                        video.frames_per_second.unbox()
                    })
                    .unwrap()
                    .map(|v| v as u64),
                    frame_bit_depth: None,
                    fir_count: Option::<u32>::try_from(unsafe {
                        video.fir_count.unbox()
                    })
                    .unwrap()
                    .map(u64::from),
                    pli_count: Option::<u32>::try_from(unsafe {
                        video.pli_count.unbox()
                    })
                    .unwrap()
                    .map(u64::from),
                    sli_count: Option::<u32>::try_from(unsafe {
                        video.sli_count.unbox()
                    })
                    .unwrap()
                    .map(u64::from),
                    concealment_events: Option::try_from(unsafe {
                        video.concealment_events.unbox()
                    })
                    .unwrap(),
                    frames_received: Option::<i32>::try_from(unsafe {
                        video.frames_received.unbox()
                    })
                    .unwrap()
                    .map(|v| v as u64),
                }
            }
        }
    }
}
