//! Deserialization of [`RtcStats`].

use std::convert::TryFrom;

use dart_sys::Dart_Handle;
use medea_client_api_proto::stats::{
    Float, HighResTimeStamp, KnownIceCandidatePairState, MediaKind,
    MediaSourceStats, NonExhaustive, RtcIceCandidatePairStats,
    RtcInboundRtpStreamMediaType, RtcInboundRtpStreamStats,
    RtcOutboundRtpStreamMediaType, RtcOutboundRtpStreamStats,
    RtcRemoteInboundRtpStreamStats, RtcRemoteOutboundRtpStreamStats, RtcStat,
    RtcStatsType, RtcTransportStats, StatId, RtcIceCandidateStats, KnownCandidateType,
};
use medea_macro::dart_bridge;

use crate::api::dart_string_into_rust;

use super::utils::{handle::DartHandle, NonNullDartValueArgExt};

/// All available [`RtcStatsType`]s of a [`RtcPeerConnection`].
///
/// [`RtcStatsType`]: medea_client_api_proto::stats::RtcStatsType
/// [`RtcPeerConnection`]: crate::platform::RtcPeerConnection
#[derive(Clone, Debug)]
pub struct RtcStats(pub Vec<RtcStat>);

#[dart_bridge("flutter/lib/src/native/platform/stats.g.dart")]
mod stats {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::{api::DartValueArg, platform::dart::utils::handle::DartHandle};
    
    extern "C" {
        pub fn rtc_stats_kind(stats: Dart_Handle) -> Dart_Handle;
        pub fn rtc_stats_type(stats: Dart_Handle) -> ptr::NonNull<c_char>;

        pub fn rtc_stats_timestamp_us(stats: Dart_Handle) -> i32;
        pub fn rtc_stats_id(stats: Dart_Handle) -> ptr::NonNull<c_char>;

        pub fn rtc_media_source_stats_track_identifier(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        pub fn rtc_stats_cast_to_rtc_media_source_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;
        pub fn rtc_media_source_stats_cast_to_rtc_video_source_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;
        pub fn rtc_media_source_stats_cast_to_rtc_audio_source_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;
        pub fn rtc_stats_cast_to_rtc_ice_candidate_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;
        pub fn rtc_stats_cast_to_rtc_ice_candidate_pair_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;
        pub fn rtc_stats_cast_to_rtc_transport_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;
        pub fn rtc_stats_cast_to_rtc_remote_inbound_rtp_stream_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;
        pub fn rtc_stats_cast_to_rtc_remote_outbound_rtp_stream_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;
        pub fn rtc_stats_cast_to_rtc_inbound_rtp_stream_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;
        pub fn rtc_stats_cast_to_rtc_outbound_rtp_stream_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;

        pub fn rtc_ice_candidate_stats_transport_id(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
        pub fn rtc_ice_candidate_stats_is_remote(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<bool>>>;
        pub fn rtc_ice_candidate_stats_address(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
        pub fn rtc_ice_candidate_stats_port(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<i32>>>;
        pub fn rtc_ice_candidate_stats_protocol(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
        pub fn rtc_ice_candidate_stats_candidate_type(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
        pub fn rtc_ice_candidate_stats_priority(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<i32>>>;
        pub fn rtc_ice_candidate_stats_url(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        pub fn rtc_outbound_rtp_stream_stats_track_id(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
        pub fn rtc_outbound_rtp_stream_stats_kind(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
        pub fn rtc_outbound_rtp_stream_stats_bytes_sent(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_outbound_rtp_stream_stats_packets_sent(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        pub fn rtc_outbound_rtp_stream_stats_media_source_id(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
        pub fn rtc_outbound_rtp_stream_stats_frame_width(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        pub fn rtc_outbound_rtp_stream_stats_frame_height(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        pub fn rtc_outbound_rtp_stream_stats_frames_per_second(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        pub fn rtc_inbound_rtp_stream_stats_remote_id(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
        pub fn rtc_inbound_rtp_stream_stats_bytes_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_inbound_rtp_stream_stats_packets_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        // pub fn RTCInboundRTPStreamStats_packets_lost(stats: Dart_Handle) ->
        // ptr::NonNull<DartValueArg<Option<String>>>;
        // pub fn RTCInboundRTPStreamStats_jitter(stats: Dart_Handle) ->
        // ptr::NonNull<DartValueArg<Option<String>>>;
        pub fn rtc_inbound_rtp_stream_stats_total_decode_time(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        pub fn rtc_inbound_rtp_stream_stats_jitter_buffer_emitted_count(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        pub fn rtc_inbound_rtp_stream_stats_media_type(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<DartHandle>>>;

        pub fn rtc_inbound_rtp_stream_stats_media_type_class(
            stats: Dart_Handle,
        ) -> ptr::NonNull<c_char>;

        pub fn rtc_inbound_rtp_stream_media_type_cast_to_audio(
            stats: Dart_Handle,
        ) -> Dart_Handle;
        pub fn rtc_inbound_rtp_stream_media_type_cast_to_video(
            stats: Dart_Handle,
        ) -> Dart_Handle;

        // todo audio
        // pub fn RTCInboundRTPStreamStats_voice_activity_flag(stats:
        // Dart_Handle) -> UniquePtr<RTCStatsMemberbool>;
        pub fn rtc_inbound_rtp_stream_audio_total_samples_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_inbound_rtp_stream_audio_concealed_samples(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_inbound_rtp_stream_audio_silent_concealed_samples(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_inbound_rtp_stream_audio_audio_level(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        pub fn rtc_inbound_rtp_stream_audio_total_audio_energy(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        pub fn rtc_inbound_rtp_stream_audio_total_samples_duration(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        // todo audio

        // todo video
        pub fn rtc_inbound_rtp_stream_video_frames_decoded(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        pub fn rtc_inbound_rtp_stream_video_key_frames_decoded(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        pub fn rtc_inbound_rtp_stream_video_frame_width(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        pub fn rtc_inbound_rtp_stream_video_frame_height(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        pub fn rtc_inbound_rtp_stream_video_total_inter_frame_delay(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        pub fn rtc_inbound_rtp_stream_video_frames_per_second(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        pub fn rtc_inbound_rtp_stream_video_frame_bit_depth(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        pub fn rtc_inbound_rtp_stream_video_fir_count(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        pub fn rtc_inbound_rtp_stream_video_pli_count(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        // pub fn RTCInboundRTPStreamStats_sli_count(stats: Dart_Handle) ->
        // ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_inbound_rtp_stream_video_concealment_events(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_inbound_rtp_stream_video_frames_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<i32>>>;

        pub fn rtc_ice_candidate_pair_stats_state(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
        pub fn rtc_ice_candidate_pair_stats_nominated(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<bool>>>;
        pub fn rtc_ice_candidate_pair_stats_bytes_sent(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_ice_candidate_pair_stats_bytes_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_ice_candidate_pair_stats_total_round_trip_time(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        pub fn rtc_ice_candidate_pair_stats_current_round_trip_time(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        pub fn rtc_ice_candidate_pair_stats_available_outgoing_bitrate(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        pub fn rtc_transport_stats_packets_sent(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_transport_stats_packets_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_transport_stats_bytes_sent(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_transport_stats_bytes_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;
        // pub fn RTCTransportStats_ice_role(stats:Dart_Handle) ->
        // ptr::NonNull<DartValueArg<Option<String>>>;

        pub fn rtc_remote_inbound_rtp_stream_stats_local_id(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
        // pub fn RTCRemoteInboundRtpStreamStats_jitter(stats: Dart_Handle) ->
        // ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_remote_inbound_rtp_stream_stats_round_trip_time(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        pub fn rtc_remote_inbound_rtp_stream_stats_fraction_lost(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        // pub fn RTCRemoteInboundRtpStreamStats_reports_received(stats:
        // Dart_Handle) -> ptr::NonNull<DartValueArg<Option<u64>>>;
        pub fn rtc_remote_inbound_rtp_stream_stats_round_trip_time_measurements(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<i32>>>;

        pub fn rtc_remote_outbound_rtp_stream_stats_local_id(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
        pub fn rtc_remote_outbound_rtp_stream_stats_remote_timestamp(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        pub fn rtc_remote_outbound_rtp_stream_stats_reports_sent(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        pub fn rtc_video_source_stats_width(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        pub fn rtc_video_source_stats_height(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        pub fn rtc_video_source_stats_frames(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;
        pub fn rtc_video_source_stats_frames_per_second(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        pub fn rtc_audio_source_stats_audio_level(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        pub fn rtc_audio_source_stats_total_audio_energy(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        pub fn rtc_audio_source_stats_total_samples_duration(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        pub fn rtc_audio_source_stats_echo_return_loss(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;
        pub fn rtc_audio_source_stats_echo_return_loss_enhancement(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

    }
}

#[derive(Debug)]
pub struct RTCInboundRTPStreamAudio(DartHandle);
impl RTCInboundRTPStreamAudio {
    pub fn total_samples_received(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_audio_total_samples_received(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    pub fn concealed_samples(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_audio_concealed_samples(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn silent_concealed_samples(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_audio_silent_concealed_samples(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    pub fn audio_level(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_audio_audio_level(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn total_audio_energy(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_audio_total_audio_energy(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn total_samples_duration(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_audio_total_samples_duration(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }
}

#[derive(Debug)]
pub struct RTCInboundRTPStreamVideo(DartHandle);
impl RTCInboundRTPStreamVideo {
    pub fn frames_decoded(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_frames_decoded(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn key_frames_decoded(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_key_frames_decoded(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn frame_width(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_frame_width(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn frame_height(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_frame_height(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn total_inter_frame_delay(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_total_inter_frame_delay(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    pub fn frames_per_second(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_frames_per_second(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn frame_bit_depth(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_frame_bit_depth(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn fir_count(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_fir_count(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn pli_count(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_pli_count(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn concealment_events(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_concealment_events(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn frames_received(&self) -> Option<i32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_frames_received(self.0.get())
                .unbox()
        })
        .unwrap()
    }
}

#[derive(Debug)]
pub enum RTCInboundRTPStreamMediaType {
    Audio(RTCInboundRTPStreamAudio),
    Video(RTCInboundRTPStreamVideo)
}

impl From<Dart_Handle> for RTCInboundRTPStreamMediaType {
    fn from(handle: Dart_Handle) -> Self {
        unsafe {
            let kind = dart_string_into_rust(stats::rtc_inbound_rtp_stream_stats_media_type_class(handle));
            match kind.as_str() {
                "_$RTCInboundRTPStreamAudio" => Self::Audio(RTCInboundRTPStreamAudio(DartHandle::new(stats::rtc_inbound_rtp_stream_media_type_cast_to_audio(handle)))),
                _ => Self::Video(RTCInboundRTPStreamVideo(DartHandle::new(stats::rtc_inbound_rtp_stream_media_type_cast_to_video(handle))))
            } 
        }
    }
}

#[derive(Debug)]
pub struct RTCInboundRTPStreamStats(DartHandle);
impl RTCInboundRTPStreamStats {
    pub fn remote_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_stats_remote_id(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn bytes_received(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_stats_bytes_received(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn packets_received(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_stats_packets_received(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn total_decode_time(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_stats_total_decode_time(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn jitter_buffer_emitted_count(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_stats_jitter_buffer_emitted_count(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    pub fn media_type(&self) -> Option<DartHandle> {
        unsafe{
        stats::rtc_inbound_rtp_stream_stats_media_type(
            self.0.get(),
        )
        .unbox()}.try_into()
        .unwrap()
    }

}

impl From<&RTCInboundRTPStreamMediaType> for RtcInboundRtpStreamMediaType {
    fn from(stats: &RTCInboundRTPStreamMediaType) -> Self {
        match stats {
            RTCInboundRTPStreamMediaType::Audio(audio) => Self::Audio{
                voice_activity_flag: None,
                total_samples_received: audio.total_samples_received(),
                concealed_samples: audio.concealed_samples(),
                silent_concealed_samples: audio.silent_concealed_samples(),
                audio_level: audio.audio_level().map(|s| Float(s)),
                total_audio_energy: audio.total_audio_energy().map(|s| Float(s)),
                total_samples_duration: audio
                    .total_samples_duration()
                    .map(|v| HighResTimeStamp(v)),
            },
            RTCInboundRTPStreamMediaType::Video(video) => Self::Video {
                frames_decoded: video.frames_decoded().map(|v| v as u64),
                key_frames_decoded: video.key_frames_decoded().map(|v| v as u64),
                frame_width: video.frame_width().map(|v| v as u64),
                frame_height: video.frame_height().map(|v| v as u64),
                total_inter_frame_delay: video
                    .total_inter_frame_delay()
                    .map(|s| Float(s)),
                frames_per_second: video.frames_per_second().map(|v| v as u64),
                frame_bit_depth: video.frame_bit_depth().map(|v| v as u64),
                fir_count: video.fir_count().map(|v| v as u64),
                pli_count: video.pli_count().map(|v| v as u64),
                sli_count: None,
                concealment_events: video.concealment_events(),
                frames_received: video.frames_received().map(|v| v as u64),
            },
        }
    }
}

impl From<RTCInboundRTPStreamStats> for RtcInboundRtpStreamStats {
    fn from(stats: RTCInboundRTPStreamStats) -> Self {
        todo!();
        // let media_specific_stats = RtcInboundRtpStreamMediaType::from();
        // Self {
        //     track_id: stats.remote_id(),
        //     media_specific_stats,
        //     bytes_received: stats.bytes_received().unwrap(),
        //     packets_received: stats.packets_received().unwrap() as u64,
        //     packets_lost: None,
        //     jitter: None,
        //     total_decode_time: stats
        //         .total_decode_time()
        //         .map(|v| HighResTimeStamp(v)),
        //     jitter_buffer_emitted_count: stats.jitter_buffer_emitted_count(),
        // }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum RTCStatsIceCandidatePairState {
    Frozen,
    Waiting,
    InProgress,
    Failed,
    Succeeded,
}

#[derive(Debug)]
pub struct RTCIceCandidatePairStats(DartHandle);
impl RTCIceCandidatePairStats {
    pub fn state(&self) -> RTCStatsIceCandidatePairState {
        todo!()
    }

    pub fn nominated(&self) -> Option<bool> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_pair_stats_nominated(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn bytes_sent(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_pair_stats_bytes_sent(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn bytes_received(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_pair_stats_bytes_received(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn total_round_trip_time(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_pair_stats_total_round_trip_time(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    pub fn current_round_trip_time(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_pair_stats_current_round_trip_time(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    pub fn available_outgoing_bitrate(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_pair_stats_available_outgoing_bitrate(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }
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

impl From<RTCIceCandidatePairStats> for RtcIceCandidatePairStats {
    fn from(stats: RTCIceCandidatePairStats) -> Self {
        let state = KnownIceCandidatePairState::from(stats.state());
        Self {
            state: NonExhaustive::Known(state),
            nominated: stats.nominated().unwrap(),
            bytes_sent: stats.bytes_sent().unwrap(),
            bytes_received: stats.bytes_received().unwrap(),
            total_round_trip_time: stats
                .total_round_trip_time()
                .map(|v| HighResTimeStamp(v)),
            current_round_trip_time: stats
                .current_round_trip_time()
                .map(|v| HighResTimeStamp(v)),
            available_outgoing_bitrate: stats
                .available_outgoing_bitrate()
                .map(|v| v as u64),
        }
    }
}

#[derive(Debug)]
pub struct RTCTransportStats(DartHandle);
impl RTCTransportStats {
    pub fn packets_sent(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_transport_stats_packets_sent(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn packets_received(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_transport_stats_packets_received(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn bytes_sent(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_transport_stats_bytes_sent(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn bytes_received(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_transport_stats_bytes_received(self.0.get()).unbox()
        })
        .unwrap()
    }
}

impl From<RTCTransportStats> for RtcTransportStats {
    fn from(stats: RTCTransportStats) -> Self {
        Self {
            packets_sent: stats.packets_sent(),
            packets_received: stats.packets_received(),
            bytes_sent: stats.bytes_sent(),
            bytes_received: stats.bytes_received(),
            ice_role: None,
        }
    }
}

#[derive(Debug)]
pub struct RTCRemoteInboundRtpStreamStats(DartHandle);
impl RTCRemoteInboundRtpStreamStats {
    pub fn local_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_remote_inbound_rtp_stream_stats_local_id(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn round_trip_time(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_remote_inbound_rtp_stream_stats_round_trip_time(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    pub fn fraction_lost(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_remote_inbound_rtp_stream_stats_fraction_lost(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    pub fn round_trip_time_measurements(&self) -> Option<i32> {
        Option::try_from(unsafe {
            stats::rtc_remote_inbound_rtp_stream_stats_round_trip_time_measurements(self.0.get()).unbox()
        })
        .unwrap()
    }
}

impl From<RTCRemoteInboundRtpStreamStats> for RtcRemoteInboundRtpStreamStats {
    fn from(stats: RTCRemoteInboundRtpStreamStats) -> Self {
        Self {
            local_id: stats.local_id(),
            jitter: None,
            round_trip_time: stats.round_trip_time().map(|v| Float(v)),
            fraction_lost: stats.fraction_lost().map(|v| Float(v)),
            reports_received: None,
            round_trip_time_measurements: stats
                .round_trip_time_measurements()
                .map(|v| Float(v as f64)),
        }
    }
}

#[derive(Debug)]
pub struct RTCRemoteOutboundRtpStreamStats(DartHandle);
impl RTCRemoteOutboundRtpStreamStats {
    pub fn local_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_remote_outbound_rtp_stream_stats_local_id(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn remote_timestamp(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_remote_outbound_rtp_stream_stats_remote_timestamp(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    pub fn reports_sent(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_remote_outbound_rtp_stream_stats_reports_sent(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }
}

impl From<RTCRemoteOutboundRtpStreamStats> for RtcRemoteOutboundRtpStreamStats {
    fn from(stats: RTCRemoteOutboundRtpStreamStats) -> Self {
        Self {
            local_id: stats.local_id(),
            remote_timestamp: stats
                .remote_timestamp()
                .map(|v| HighResTimeStamp(v)),
            reports_sent: stats.reports_sent(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TrackKind {
    Audio,
    Video,
}

#[derive(Debug)]
pub struct RTCOutboundRTPStreamStats(DartHandle);
impl RTCOutboundRTPStreamStats {
    pub fn track_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_track_id(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn kind(&self) -> TrackKind {
        todo!()
    }

    pub fn frame_width(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_frame_width(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn frame_height(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_frame_height(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn frames_per_second(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_frames_per_second(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn bytes_sent(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_bytes_sent(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn packets_sent(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_packets_sent(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn media_source_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_media_source_id(self.0.get())
                .unbox()
        })
        .unwrap()
    }
}

impl From<&RTCOutboundRTPStreamStats> for RtcOutboundRtpStreamMediaType {
    fn from(stats: &RTCOutboundRTPStreamStats) -> Self {
        match stats.kind() {
            TrackKind::Audio => Self::Audio {
                total_samples_sent: None,
                voice_activity_flag: None,
            },
            TrackKind::Video => Self::Video {
                frame_width: stats.frame_width().map(|v| v as u64),
                frame_height: stats.frame_height().map(|v| v as u64),
                frames_per_second: stats.frames_per_second().map(|v| v as u64),
            },
        }
    }
}

impl From<RTCOutboundRTPStreamStats> for RtcOutboundRtpStreamStats {
    fn from(stats: RTCOutboundRTPStreamStats) -> Self {
        let media_type = RtcOutboundRtpStreamMediaType::from(&stats);
        Self {
            track_id: stats.track_id(),
            media_type,
            bytes_sent: stats.bytes_sent(),
            packets_sent: stats.packets_sent().map(|v| v as u64),
            media_source_id: stats.media_source_id(),
        }
    }
}

#[derive(Debug)]
pub struct RTCVideoSourceStats(DartHandle);
impl RTCVideoSourceStats {
    pub fn width(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_video_source_stats_width(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn height(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_video_source_stats_height(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn frames(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_video_source_stats_frames(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn frames_per_second(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_video_source_stats_frames_per_second(self.0.get())
                .unbox()
        })
        .unwrap()
    }
}

#[derive(Debug)]
pub struct RTCAudioSourceStats(DartHandle);
impl RTCAudioSourceStats {
    pub fn audio_level(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_audio_source_stats_audio_level(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn total_audio_energy(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_audio_source_stats_total_audio_energy(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn total_samples_duration(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_audio_source_stats_total_samples_duration(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    pub fn echo_return_loss(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_audio_source_stats_echo_return_loss(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn echo_return_loss_enhancement(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_audio_source_stats_echo_return_loss_enhancement(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }
}

#[derive(Debug)]
pub enum RTCMediaSourceStatsType {
    RTCVideoSourceStats(RTCVideoSourceStats),
    RTCAudioSourceStats(RTCAudioSourceStats),
}

#[derive(Debug)]
pub struct RTCMediaSourceStats(DartHandle);
impl RTCMediaSourceStats {
    pub fn track_identifier(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_media_source_stats_track_identifier(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn kind(&self) -> RTCMediaSourceStatsType {
        todo!()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CandidateType {
    Host,
    Srflx,
    Prflx,
    Relay,
}

impl From<CandidateType> for KnownCandidateType {
    fn from(kind: CandidateType) -> Self {
        match kind {
            CandidateType::Host => Self::Host,
            CandidateType::Srflx => Self::Srlfx,
            CandidateType::Prflx => Self::Prflx,
            CandidateType::Relay => Self::Relay,
        }
    }
}

#[derive(Debug)]
pub struct RTCIceCandidateStats(DartHandle);
impl RTCIceCandidateStats {
    pub fn is_remote(&self) -> Option<bool> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_stats_is_remote(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn transport_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_stats_transport_id(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn address(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_stats_address(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn port(&self) -> Option<i32> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_stats_priority(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn protocol(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_stats_protocol(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn candidate_type(&self) -> CandidateType {
        todo!()
    }

    pub fn priority(&self) -> Option<i32> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_stats_priority(self.0.get()).unbox()
        })
        .unwrap()
    }

    pub fn url(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_stats_url(self.0.get()).unbox()
        })
        .unwrap()
    }
}

impl From<RTCIceCandidateStats> for RtcIceCandidateStats{
    fn from(stats: RTCIceCandidateStats) -> Self {
        Self {
            transport_id: stats.transport_id(),
            address: stats.address(),
            port: stats.port().unwrap() as u16,
            protocol: NonExhaustive::Unknown(stats.protocol().unwrap()),
            candidate_type: NonExhaustive::Known(KnownCandidateType::from(stats.candidate_type())),
            priority: stats.priority().unwrap() as u32,
            url: stats.url(),
            relay_protocol: None,
        }
    }
}

#[derive(Debug)]
pub enum RTCStatsType {
    RTCMediaSourceStats(RTCMediaSourceStats),
    RTCIceCandidateStats(RTCIceCandidateStats),
    RTCOutboundRTPStreamStats(RTCOutboundRTPStreamStats),
    RTCInboundRTPStreamStats(RTCInboundRTPStreamStats),
    RTCIceCandidatePairStats(RTCIceCandidatePairStats),
    RTCTransportStats(RTCTransportStats),
    RTCRemoteInboundRtpStreamStats(RTCRemoteInboundRtpStreamStats),
    RTCRemoteOutboundRtpStreamStats(RTCRemoteOutboundRtpStreamStats),
    Unimplenented,
}

impl From<Dart_Handle> for RTCStatsType {
    fn from(inner: Dart_Handle) -> Self {
        unsafe {
            let kind = dart_string_into_rust(stats::rtc_stats_type(inner));
            match kind.as_str() {
            "_$RTCMediaSourceStats" => Self::RTCMediaSourceStats(RTCMediaSourceStats(DartHandle::new(stats::rtc_stats_cast_to_rtc_media_source_stats(inner)))),
            "_$RTCIceCandidateStats" => Self::RTCIceCandidateStats(RTCIceCandidateStats(DartHandle::new(stats::rtc_stats_cast_to_rtc_ice_candidate_stats(inner)))),
            "_$RTCOutboundRTPStreamStats" => Self::RTCOutboundRTPStreamStats(RTCOutboundRTPStreamStats(DartHandle::new(stats::rtc_stats_cast_to_rtc_outbound_rtp_stream_stats(inner)))),
            "_$RTCInboundRTPStreamStats" => Self::RTCInboundRTPStreamStats(RTCInboundRTPStreamStats(DartHandle::new(stats::rtc_stats_cast_to_rtc_inbound_rtp_stream_stats(inner)))),
            "_$RTCIceCandidatePairStats" => Self::RTCIceCandidatePairStats(RTCIceCandidatePairStats(DartHandle::new(stats::rtc_stats_cast_to_rtc_ice_candidate_pair_stats(inner)))),
            "_$RTCTransportStats" => Self::RTCTransportStats(RTCTransportStats(DartHandle::new(stats::rtc_stats_cast_to_rtc_transport_stats(inner)))),
            "_$RTCRemoteInboundRtpStreamStats" => Self::RTCRemoteInboundRtpStreamStats(RTCRemoteInboundRtpStreamStats(DartHandle::new(stats::rtc_stats_cast_to_rtc_remote_inbound_rtp_stream_stats(inner)))),
            "_$RTCRemoteOutboundRtpStreamStats" => Self::RTCRemoteOutboundRtpStreamStats(RTCRemoteOutboundRtpStreamStats(DartHandle::new(stats::rtc_stats_cast_to_rtc_remote_outbound_rtp_stream_stats(inner)))),
            _ => Self::Unimplenented,
        }
        }
    }
}

#[derive(Debug)]
pub struct RTCStats(pub DartHandle);

impl RTCStats {
    pub fn id(&self) -> String {
        unsafe { dart_string_into_rust(stats::rtc_stats_id(self.0.get())) }
    }

    pub fn timestamp_us(&self) -> i32 {
        unsafe { stats::rtc_stats_timestamp_us(self.0.get()) }
    }

    pub fn kind(&self) -> RTCStatsType {
        unsafe { RTCStatsType::from(stats::rtc_stats_kind(self.0.get())) }
    }
}

impl From<RTCMediaSourceStatsType> for MediaKind {
    fn from(stats: RTCMediaSourceStatsType) -> Self {
        match stats {
            RTCMediaSourceStatsType::RTCVideoSourceStats(stats) => {
                Self::Video {
                    width: stats.width(),
                    height: stats.height(),
                    frames_per_second: stats
                        .frames_per_second()
                        .map(|s| s as u32),
                }
            }
            RTCMediaSourceStatsType::RTCAudioSourceStats(stats) => {
                Self::Audio {
                    audio_level: stats.audio_level().map(|s| Float(s)),
                    total_audio_energy: stats
                        .total_audio_energy()
                        .map(|s| Float(s)),
                    total_samples_duration: stats
                        .total_samples_duration()
                        .map(|s| Float(s)),
                }
            }
        }
    }
}

impl From<RTCMediaSourceStats> for MediaSourceStats {
    fn from(stats: RTCMediaSourceStats) -> Self {
        Self {
            track_identifier: stats.track_identifier(),
            kind: MediaKind::from(stats.kind()),
        }
    }
}

impl From<RTCStatsType> for RtcStatsType {
    fn from(stats: RTCStatsType) -> Self {
        match stats {
            RTCStatsType::RTCMediaSourceStats(stats) => {
                Self::MediaSource(Box::new(MediaSourceStats::from(stats)))
            }
            RTCStatsType::RTCOutboundRTPStreamStats(stats) => {
                Self::OutboundRtp(Box::new(RtcOutboundRtpStreamStats::from(
                    stats,
                )))
            }
            RTCStatsType::RTCInboundRTPStreamStats(stats) => Self::InboundRtp(
                Box::new(RtcInboundRtpStreamStats::from(stats)),
            ),
            RTCStatsType::RTCIceCandidatePairStats(stats) => {
                Self::CandidatePair(Box::new(RtcIceCandidatePairStats::from(
                    stats,
                )))
            }
            RTCStatsType::RTCTransportStats(stats) => {
                Self::Transport(Box::new(RtcTransportStats::from(stats)))
            }
            RTCStatsType::RTCRemoteInboundRtpStreamStats(stats) => {
                Self::RemoteInboundRtp(Box::new(
                    RtcRemoteInboundRtpStreamStats::from(stats),
                ))
            }
            RTCStatsType::RTCRemoteOutboundRtpStreamStats(stats) => {
                Self::RemoteOutboundRtp(Box::new(
                    RtcRemoteOutboundRtpStreamStats::from(stats),
                ))
            }
            RTCStatsType::RTCIceCandidateStats(stats) => {
                let remote = stats.is_remote().unwrap(); 
                let candidate = RtcIceCandidateStats::from(stats);
                if remote {
                    Self::RemoteCandidate(Box::new(
                        candidate
                    ))
                } else 
                {
                    Self::LocalCandidate(Box::new(
                        candidate
                    ))
                }
            }
            _ => Self::Other,
        }
    }
}

impl From<RTCStats> for RtcStat {
    fn from(stats: RTCStats) -> Self {
        let id = stats.id();
        let time = HighResTimeStamp(stats.timestamp_us() as f64);
        let stats = RtcStatsType::from(stats.kind());
        Self {
            id: StatId(id),
            timestamp: time,
            stats,
        }
    }
}
