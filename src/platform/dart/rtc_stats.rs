//! Deserialization of [`RtcStats`].

use std::convert::TryFrom;

use dart_sys::Dart_Handle;
use medea_client_api_proto::stats::{
    Float, HighResTimeStamp, KnownCandidateType, KnownIceCandidatePairState,
    KnownProtocol, MediaSourceStats, NonExhaustive, RtcIceCandidatePairStats,
    RtcIceCandidateStats, RtcInboundRtpStreamMediaType,
    RtcInboundRtpStreamStats, RtcOutboundRtpStreamMediaType,
    RtcOutboundRtpStreamStats, RtcRemoteInboundRtpStreamStats,
    RtcRemoteOutboundRtpStreamStats, RtcStat, RtcStatsType, RtcTransportStats,
    StatId,
};
use medea_macro::dart_bridge;

use crate::{
    api::dart_string_into_rust,
    media::MediaKind,
    platform::rtc_stats::{
        CandidateType, Protocol, RTCStatsIceCandidatePairState,
    },
};

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
        /// Returns kind of the provided [`RTCStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn rtc_stats_kind(stats: Dart_Handle) -> Dart_Handle;

        /// Returns runtime type of the provided [`RTCStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn rtc_stats_type(stats: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns timestamp of the provided [`RTCStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn rtc_stats_timestamp_us(stats: Dart_Handle) -> i64;

        /// Returns id of the provided [`RTCStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn rtc_stats_id(stats: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns trackidentifier of the provided
        /// [`RTCMediaSourceStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcmediasourcestats
        pub fn rtc_media_source_stats_track_identifier(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        /// Returns runtime type of kind of the provided
        /// [`RTCMediaSourceStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcmediasourcestats
        pub fn rtc_media_source_stats_class_type(
            stats: Dart_Handle,
        ) -> ptr::NonNull<c_char>;

        /// Casts a [`RTCStats`][0] to a [`RTCMediaSourceStats`][1].
        ///
        /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcmediasourcestats
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn cast_to_rtc_media_source_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;

        /// Casts a [`RTCMediaSourceStats`][0] to a [`RTCVideoSourceStats`][1].
        ///
        /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcmediasourcestats
        pub fn cast_to_rtc_video_source_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;

        /// Casts a [`RTCMediaSourceStats`][0] to a [`RTCAudioSourceStats`][1].
        ///
        /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcmediasourcestats
        pub fn cast_to_rtc_audio_source_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;

        /// Casts a [`RTCStats`][0] to a [`RTCIceCandidateStats`][1].
        ///
        /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn cast_to_rtc_ice_candidate_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;

        /// Casts a [`RTCStats`][0] to a [`RTCIceCandidatePairStats`][1].
        ///
        /// [1]: https://tinyurl.com/2p9ujsyc
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn cast_to_rtc_ice_candidate_pair_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;

        /// Casts a [`RTCStats`][0] to a [`RTCTransportStats`][1].
        ///
        /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtctransportstats
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn cast_to_rtc_transport_stats(stats: Dart_Handle) -> Dart_Handle;

        /// Casts a [`RTCStats`][0] to a [`RTCRemoteInboundRTPStreamStats`][1].
        ///
        /// [1]: https://tinyurl.com/bddwwp9z
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn cast_to_rtc_remote_inbound_rtp_stream_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;

        /// Casts a [`RTCStats`][0] to a [`RTCRemoteOutboundRTPStreamStats`][1].
        ///
        /// [1]: https://tinyurl.com/ysm7jdny
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn cast_to_rtc_remote_outbound_rtp_stream_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;

        /// Casts a [`RTCStats`][0] to a [`RTCInboundRTPStreamStats`][1].
        ///
        /// [1]: https://tinyurl.com/28vmrtz6
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn cast_to_rtc_inbound_rtp_stream_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;

        /// Casts a [`RTCStats`][0] to a [`RTCOutboundRTPStreamStats`][1].
        ///
        /// [1]: https://tinyurl.com/4uw3fwsb
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn cast_to_rtc_outbound_rtp_stream_stats(
            stats: Dart_Handle,
        ) -> Dart_Handle;

        /// Returns transport_id of the provided
        /// [`RTCIceCandidateStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
        pub fn rtc_ice_candidate_stats_transport_id(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        /// Returns address of the provided [`RTCIceCandidateStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
        pub fn rtc_ice_candidate_stats_address(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        /// Returns port of the provided [`RTCIceCandidateStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
        pub fn rtc_ice_candidate_stats_port(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<i32>>>;

        /// Returns protocol of the provided [`RTCIceCandidateStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
        pub fn rtc_ice_candidate_stats_protocol(stats: Dart_Handle) -> i64;

        /// Returns candidate_type of the provided
        /// [`RTCIceCandidateStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
        pub fn rtc_ice_candidate_stats_candidate_type(
            stats: Dart_Handle,
        ) -> i64;

        /// Returns priority of the provided [`RTCIceCandidateStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
        pub fn rtc_ice_candidate_stats_priority(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<i32>>>;

        /// Returns url of the provided [`RTCIceCandidateStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
        pub fn rtc_ice_candidate_stats_url(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        /// Returns track_id of the provided
        /// [`RTCOutboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/4uw3fwsb
        pub fn rtc_outbound_rtp_stream_stats_track_id(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        /// Returns kind of the provided [`RTCOutboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/4uw3fwsb
        pub fn rtc_outbound_rtp_stream_stats_kind(stats: Dart_Handle) -> i64;

        /// Returns bytes_sent of the provided
        /// [`RTCOutboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/4uw3fwsb
        pub fn rtc_outbound_rtp_stream_stats_bytes_sent(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns packets_sent of the provided
        /// [`RTCOutboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/4uw3fwsb
        pub fn rtc_outbound_rtp_stream_stats_packets_sent(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns media_source_id of the provided
        /// [`RTCOutboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/4uw3fwsb
        pub fn rtc_outbound_rtp_stream_stats_media_source_id(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        /// Returns frame_width of the provided
        /// [`RTCOutboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/4uw3fwsb
        pub fn rtc_outbound_rtp_stream_stats_frame_width(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns frame_height of the provided
        /// [`RTCOutboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/4uw3fwsb
        pub fn rtc_outbound_rtp_stream_stats_frame_height(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns frames_per_second of the provided
        /// [`RTCOutboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/4uw3fwsb
        pub fn rtc_outbound_rtp_stream_stats_frames_per_second(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns remote_id of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_stats_remote_id(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        /// Returns bytes_received of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_stats_bytes_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns packets_received of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_stats_packets_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns total_decode_time of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_stats_total_decode_time(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns jitter_buffer_emitted_count of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_stats_jitter_buffer_emitted_count(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns media_type of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_stats_media_type(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<DartHandle>>>;

        /// Returns runtime type of kind of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_stats_media_type_class(
            stats: Dart_Handle,
        ) -> ptr::NonNull<c_char>;

        /// Casts a [`RTCInboundRTPStreamStats`][0] kind to a
        /// [`RTCInboundRTPStreamAudio`][1].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn rtc_inbound_rtp_stream_media_type_cast_to_audio(
            stats: Dart_Handle,
        ) -> Dart_Handle;

        /// Casts a [`RTCInboundRTPStreamStats`][0] kind to a
        /// [`RTCInboundRTPStreamVideo`][1].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
        pub fn rtc_inbound_rtp_stream_media_type_cast_to_video(
            stats: Dart_Handle,
        ) -> Dart_Handle;

        /// Returns total_samples_received of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_audio_total_samples_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns concealed_samples of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_audio_concealed_samples(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns silent_concealed_samples of the
        /// provided [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_audio_silent_concealed_samples(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns audio_level of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_audio_audio_level(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns total_audio_energy of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_audio_total_audio_energy(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns total_samples_duration of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_audio_total_samples_duration(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns frames_decoded of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_video_frames_decoded(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns key_frames_decoded of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_video_key_frames_decoded(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns frame_width of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_video_frame_width(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns frame_height of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_video_frame_height(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns total_inter_frame_delay of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_video_total_inter_frame_delay(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns frames_per_second of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_video_frames_per_second(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns frame_bit_depth of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_video_frame_bit_depth(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns fir_count of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_video_fir_count(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns pli_count of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_video_pli_count(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns concealment_events of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_video_concealment_events(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns frames_received of the provided
        /// [`RTCInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/28vmrtz6
        pub fn rtc_inbound_rtp_stream_video_frames_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<i32>>>;

        /// Returns state of the provided
        /// [`RTCIceCandidatePairStats`][0].
        ///
        /// [0]: https://tinyurl.com/2p9ujsyc
        pub fn rtc_ice_candidate_pair_stats_state(stats: Dart_Handle) -> i64;

        /// Returns nominated of the provided
        /// [`RTCIceCandidatePairStats`][0].
        ///
        /// [0]: https://tinyurl.com/2p9ujsyc
        pub fn rtc_ice_candidate_pair_stats_nominated(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<bool>>>;

        /// Returns bytes_sent of the provided
        /// [`RTCIceCandidatePairStats`][0].
        ///
        /// [0]: https://tinyurl.com/2p9ujsyc
        pub fn rtc_ice_candidate_pair_stats_bytes_sent(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns bytes_received of the provided
        /// [`RTCIceCandidatePairStats`][0].
        ///
        /// [0]: https://tinyurl.com/2p9ujsyc
        pub fn rtc_ice_candidate_pair_stats_bytes_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns total_round_trip_time of the provided
        /// [`RTCIceCandidatePairStats`][0].
        ///
        /// [0]: https://tinyurl.com/2p9ujsyc
        pub fn rtc_ice_candidate_pair_stats_total_round_trip_time(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns current_round_trip_time of the provided
        /// [`RTCIceCandidatePairStats`][0].
        ///
        /// [0]: https://tinyurl.com/2p9ujsyc
        pub fn rtc_ice_candidate_pair_stats_current_round_trip_time(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns available_outgoing_bitrate of the provided
        /// [`RTCIceCandidatePairStats`][0].
        ///
        /// [0]: https://tinyurl.com/2p9ujsyc
        pub fn rtc_ice_candidate_pair_stats_available_outgoing_bitrate(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns packets_sent of the provided
        /// [`RTCTransportStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtctransportstats
        pub fn rtc_transport_stats_packets_sent(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns packets_received of the provided
        /// [`RTCTransportStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtctransportstats
        pub fn rtc_transport_stats_packets_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns bytes_sent of the provided
        /// [`RTCTransportStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtctransportstats
        pub fn rtc_transport_stats_bytes_sent(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns bytes_received of the provided
        /// [`RTCTransportStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtctransportstats
        pub fn rtc_transport_stats_bytes_received(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns local_id of the provided
        /// [`RTCRemoteInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/bddwwp9z
        pub fn rtc_remote_inbound_rtp_stream_stats_local_id(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        /// Returns round_trip_time[1] of the provided
        /// [`RTCRemoteInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/bddwwp9z
        pub fn rtc_remote_inbound_rtp_stream_stats_round_trip_time(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns fraction_lost of the provided
        /// [`RTCRemoteInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/bddwwp9z
        pub fn rtc_remote_inbound_rtp_stream_stats_fraction_lost(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns round_trip_time_measurements of the provided
        /// [`RTCRemoteInboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/bddwwp9z
        pub fn round_trip_time_measurements(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<i32>>>;

        /// Returns local_id of the provided
        /// [`RTCRemoteOutboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/ysm7jdny
        pub fn rtc_remote_outbound_rtp_stream_stats_local_id(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        /// Returns remote_timestamp of the provided
        /// [`RTCRemoteOutboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/ysm7jdny
        pub fn rtc_remote_outbound_rtp_stream_stats_remote_timestamp(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns reports_sent of the provided
        /// [`RTCRemoteOutboundRTPStreamStats`][0].
        ///
        /// [0]: https://tinyurl.com/ysm7jdny
        pub fn rtc_remote_outbound_rtp_stream_stats_reports_sent(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u64>>>;

        /// Returns width of the provided
        /// [`RTCVideoSourceStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats
        pub fn rtc_video_source_stats_width(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns height of the provided
        /// [`RTCVideoSourceStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats
        pub fn rtc_video_source_stats_height(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns frames of the provided
        /// [`RTCVideoSourceStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats
        pub fn rtc_video_source_stats_frames(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns frames_per_second of the provided
        /// [`RTCVideoSourceStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats
        pub fn rtc_video_source_stats_frames_per_second(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns audio_level of the provided
        /// [`RTCAudioSourceStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
        pub fn rtc_audio_source_stats_audio_level(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns total_audio_energy of the provided
        /// [`RTCAudioSourceStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
        pub fn rtc_audio_source_stats_total_audio_energy(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns total_samples_duration of the provided
        /// [`RTCAudioSourceStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
        pub fn rtc_audio_source_stats_total_samples_duration(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns echo_return_loss of the provided
        /// [`RTCAudioSourceStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
        pub fn rtc_audio_source_stats_echo_return_loss(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

        /// Returns echo_return_loss_enhancement of the provided
        /// [`RTCAudioSourceStats`][0].
        ///
        /// [0]: https://www.w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
        pub fn rtc_audio_source_stats_echo_return_loss_enhancement(
            stats: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<f64>>>;

    }
}

/// Representation of [`RTCInboundRTPStreamStats`][1] when kind is audio.
///
/// [1]: https://tinyurl.com/28vmrtz6
#[derive(Debug)]
pub struct RTCInboundRTPStreamAudio(DartHandle);
impl RTCInboundRTPStreamAudio {
    /// Returns [`total_samples_received`][1] of this
    /// [`RTCInboundRTPStreamAudio`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn total_samples_received(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_audio_total_samples_received(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    /// Returns [`concealed_samples`][1] of this [`RTCInboundRTPStreamAudio`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn concealed_samples(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_audio_concealed_samples(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`silent_concealed_samples`][1] of this
    /// [`RTCInboundRTPStreamAudio`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn silent_concealed_samples(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_audio_silent_concealed_samples(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    /// Returns [`audio_level`][1] of this [`RTCInboundRTPStreamAudio`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn audio_level(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_audio_audio_level(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`total_audio_energy`][1] of this [`RTCInboundRTPStreamAudio`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn total_audio_energy(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_audio_total_audio_energy(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`total_samples_duration`][1] of this
    /// [`RTCInboundRTPStreamAudio`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
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

/// Representation of [`RTCInboundRTPStreamStats`][1] when kind is video.
///
/// [1]: https://tinyurl.com/28vmrtz6
#[derive(Debug)]
pub struct RTCInboundRTPStreamVideo(DartHandle);
impl RTCInboundRTPStreamVideo {
    /// Returns [`frames_decoded`][1] of this [`RTCInboundRTPStreamVideo`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn frames_decoded(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_frames_decoded(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`key_frames_decoded`][1] of this [`RTCInboundRTPStreamVideo`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn key_frames_decoded(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_key_frames_decoded(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`frame_width`][1] of this [`RTCInboundRTPStreamVideo`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn frame_width(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_frame_width(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`frame_height`][1] of this [`RTCInboundRTPStreamVideo`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn frame_height(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_frame_height(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`total_inter_frame_delay`][1] of this
    /// [`RTCInboundRTPStreamVideo`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn total_inter_frame_delay(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_total_inter_frame_delay(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    /// Returns [`frames_per_second`][1] of this [`RTCInboundRTPStreamVideo`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn frames_per_second(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_frames_per_second(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`frame_bit_depth`][1] of this [`RTCInboundRTPStreamVideo`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn frame_bit_depth(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_frame_bit_depth(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`fir_count`][1] of this [`RTCInboundRTPStreamVideo`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn fir_count(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_fir_count(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [`pli_count`][1] of this [`RTCInboundRTPStreamVideo`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn pli_count(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_pli_count(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [`concealment_events`][1] of this [`RTCInboundRTPStreamVideo`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn concealment_events(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_concealment_events(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`frames_received`][1] of this [`RTCInboundRTPStreamVideo`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn frames_received(&self) -> Option<i32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_video_frames_received(self.0.get())
                .unbox()
        })
        .unwrap()
    }
}

/// Representation of [`RTCInboundRTPStreamStats`][1] kind variants.
///
/// [1]: https://tinyurl.com/28vmrtz6
#[derive(Debug)]
pub enum RTCInboundRTPStreamMediaType {
    /// Stats when [RTCInboundRTPStreamStats] kind is audio.
    Audio(RTCInboundRTPStreamAudio),
    /// Stats when [RTCInboundRTPStreamStats] kind is video.
    Video(RTCInboundRTPStreamVideo),
}

impl From<Dart_Handle> for RTCInboundRTPStreamMediaType {
    fn from(handle: Dart_Handle) -> Self {
        unsafe {
            let kind = dart_string_into_rust(
                stats::rtc_inbound_rtp_stream_stats_media_type_class(handle),
            );
            match kind.as_str() {
                "RTCInboundRTPStreamAudio" => {
                    Self::Audio(RTCInboundRTPStreamAudio(DartHandle::new(
                        stats::rtc_inbound_rtp_stream_media_type_cast_to_audio(
                            handle,
                        ),
                    )))
                }
                _ => Self::Video(RTCInboundRTPStreamVideo(DartHandle::new(
                    stats::rtc_inbound_rtp_stream_media_type_cast_to_video(
                        handle,
                    ),
                ))),
            }
        }
    }
}

/// Representation of [`RTCInboundRTPStreamStats`][1].
///
/// [1]: https://tinyurl.com/28vmrtz6
#[derive(Debug)]
pub struct RTCInboundRTPStreamStats(DartHandle);
impl RTCInboundRTPStreamStats {
    /// Returns [`remote_id`][1] of this [`RTCInboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn remote_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_stats_remote_id(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [`bytes_received`][1] of this [`RTCInboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn bytes_received(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_stats_bytes_received(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`packets_received`][1] of this [`RTCInboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn packets_received(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_stats_packets_received(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`total_decode_time`] of this [`RTCInboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn total_decode_time(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_stats_total_decode_time(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`jitter_buffer_emitted_count`] of this
    /// [`RTCInboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn jitter_buffer_emitted_count(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_inbound_rtp_stream_stats_jitter_buffer_emitted_count(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    /// Returns [`media_type`] of this [`RTCInboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/28vmrtz6
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn media_type(&self) -> Option<DartHandle> {
        unsafe {
            stats::rtc_inbound_rtp_stream_stats_media_type(self.0.get()).unbox()
        }
        .try_into()
        .unwrap()
    }
}

impl From<&RTCInboundRTPStreamMediaType> for RtcInboundRtpStreamMediaType {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn from(stats: &RTCInboundRTPStreamMediaType) -> Self {
        match stats {
            RTCInboundRTPStreamMediaType::Audio(audio) => Self::Audio {
                voice_activity_flag: None, // TODO
                total_samples_received: audio.total_samples_received(),
                concealed_samples: audio.concealed_samples(),
                silent_concealed_samples: audio.silent_concealed_samples(),
                audio_level: audio.audio_level().map(Float),
                total_audio_energy: audio.total_audio_energy().map(Float),
                total_samples_duration: audio
                    .total_samples_duration()
                    .map(HighResTimeStamp),
            },
            RTCInboundRTPStreamMediaType::Video(video) => Self::Video {
                frames_decoded: video.frames_decoded().map(u64::from),
                key_frames_decoded: video.key_frames_decoded().map(u64::from),
                frame_width: video.frame_width().map(u64::from),
                frame_height: video.frame_height().map(u64::from),
                total_inter_frame_delay: video
                    .total_inter_frame_delay()
                    .map(Float),
                frames_per_second: video.frames_per_second().map(|v| v as u64),
                frame_bit_depth: video.frame_bit_depth().map(u64::from),
                fir_count: video.fir_count().map(u64::from),
                pli_count: video.pli_count().map(u64::from),
                sli_count: None, // TODO
                concealment_events: video.concealment_events(),
                frames_received: video.frames_received().map(|v| v as u64),
            },
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<RTCInboundRTPStreamStats> for RtcInboundRtpStreamStats {
    fn from(stats: RTCInboundRTPStreamStats) -> Self {
        let temp = stats
            .media_type()
            .map(|v| RTCInboundRTPStreamMediaType::from(v.get()));
        let media_specific_stats =
            temp.as_ref().map(RtcInboundRtpStreamMediaType::from);
        Self {
            track_id: stats.remote_id(),
            media_specific_stats,
            bytes_received: stats.bytes_received().unwrap(),
            packets_received: u64::from(stats.packets_received().unwrap()),
            packets_lost: None, // TODO
            jitter: None,       // TODO
            total_decode_time: stats.total_decode_time().map(HighResTimeStamp),
            jitter_buffer_emitted_count: stats.jitter_buffer_emitted_count(),
        }
    }
}

/// Representation of [`RTCIceCandidatePairStats`][1].
///
/// [1]: https://tinyurl.com/2p9ujsyc
#[derive(Debug)]
pub struct RTCIceCandidatePairStats(DartHandle);
impl RTCIceCandidatePairStats {
    /// Returns [`state`][1] of this [`RTCIceCandidatePairStats`].
    ///
    /// [1]: https://tinyurl.com/2p9ujsyc
    pub fn state(&self) -> RTCStatsIceCandidatePairState {
        unsafe {
            RTCStatsIceCandidatePairState::try_from(
                stats::rtc_ice_candidate_pair_stats_state(self.0.get()),
            )
            .unwrap()
        }
    }

    /// Returns [`nominated`][1] of this [`RTCIceCandidatePairStats`].
    ///
    /// [1]: https://tinyurl.com/2p9ujsyc
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn nominated(&self) -> Option<bool> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_pair_stats_nominated(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [`bytes_sent`][1] of this [`RTCIceCandidatePairStats`].
    ///
    /// [1]: https://tinyurl.com/2p9ujsyc
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn bytes_sent(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_pair_stats_bytes_sent(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [`bytes_received`][1] of this [`RTCIceCandidatePairStats`].
    ///
    /// [1]: https://tinyurl.com/2p9ujsyc
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn bytes_received(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_pair_stats_bytes_received(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`total_round_trip_time`][1] of this
    /// [`RTCIceCandidatePairStats`].
    ///
    /// [1]: https://tinyurl.com/2p9ujsyc
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn total_round_trip_time(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_pair_stats_total_round_trip_time(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    /// Returns [`current_round_trip_time`][1] of this
    /// [`RTCIceCandidatePairStats`].
    ///
    /// [1]: https://tinyurl.com/2p9ujsyc
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn current_round_trip_time(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_pair_stats_current_round_trip_time(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    /// Returns [`available_outgoing_bitrate`][1] of this
    /// [`RTCIceCandidatePairStats`].
    ///
    /// [1]: https://tinyurl.com/2p9ujsyc
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
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

#[allow(clippy::fallible_impl_from)]
impl From<RTCIceCandidatePairStats> for RtcIceCandidatePairStats {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn from(stats: RTCIceCandidatePairStats) -> Self {
        Self {
            state: NonExhaustive::Known(KnownIceCandidatePairState::from(
                stats.state(),
            )),
            nominated: stats.nominated().unwrap(),
            bytes_sent: stats.bytes_sent().unwrap(),
            bytes_received: stats.bytes_received().unwrap(),
            total_round_trip_time: stats
                .total_round_trip_time()
                .map(HighResTimeStamp),
            current_round_trip_time: stats
                .current_round_trip_time()
                .map(HighResTimeStamp),
            available_outgoing_bitrate: stats
                .available_outgoing_bitrate()
                .map(|v| v as u64),
        }
    }
}

/// Representation of [`RTCTransportStats`][1].
///
/// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtctransportstats
#[derive(Debug)]
pub struct RTCTransportStats(DartHandle);
impl RTCTransportStats {
    /// Returns [`packets_sent`][1] of this [`RTCTransportStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtctransportstats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn packets_sent(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_transport_stats_packets_sent(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [`packets_received`][1] of this [`RTCTransportStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtctransportstats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn packets_received(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_transport_stats_packets_received(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [`bytes_sent`][1] of this [`RTCTransportStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtctransportstats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn bytes_sent(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_transport_stats_bytes_sent(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [`bytes_received`][1] of this [`RTCTransportStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtctransportstats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
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
            ice_role: None, // TODO
        }
    }
}

/// Representation of [`RTCRemoteInboundRtpStreamStats`][1].
///
/// [1]: https://tinyurl.com/bddwwp9z
#[derive(Debug)]
pub struct RTCRemoteInboundRtpStreamStats(DartHandle);
impl RTCRemoteInboundRtpStreamStats {
    /// Returns [`local_id`][1] of this [`RTCRemoteInboundRtpStreamStats`].
    ///
    /// [1]: https://tinyurl.com/bddwwp9z
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn local_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_remote_inbound_rtp_stream_stats_local_id(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`round_trip_time`][1] of this
    /// [`RTCRemoteInboundRtpStreamStats`].
    ///
    /// [1]: https://tinyurl.com/bddwwp9z
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn round_trip_time(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_remote_inbound_rtp_stream_stats_round_trip_time(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    /// Returns [`fraction_lost`][1] of this [`RTCRemoteInboundRtpStreamStats`].
    ///
    /// [1]: https://tinyurl.com/bddwwp9z
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn fraction_lost(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_remote_inbound_rtp_stream_stats_fraction_lost(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    /// Returns [`round_trip_time_measurements`][1] of this
    /// [`RTCRemoteInboundRtpStreamStats`].
    ///
    /// [1]: https://tinyurl.com/bddwwp9z
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn round_trip_time_measurements(&self) -> Option<i32> {
        Option::try_from(unsafe {
            stats::round_trip_time_measurements(self.0.get()).unbox()
        })
        .unwrap()
    }
}

impl From<RTCRemoteInboundRtpStreamStats> for RtcRemoteInboundRtpStreamStats {
    fn from(stats: RTCRemoteInboundRtpStreamStats) -> Self {
        Self {
            local_id: stats.local_id(),
            jitter: None, // TODO
            round_trip_time: stats.round_trip_time().map(Float),
            fraction_lost: stats.fraction_lost().map(Float),
            reports_received: None, // TODO
            round_trip_time_measurements: stats
                .round_trip_time_measurements()
                .map(|v| Float(f64::from(v))),
        }
    }
}

/// Representation of [`RTCRemoteOutboundRtpStreamStats`][1].
///
/// [1]: https://tinyurl.com/ysm7jdny
#[derive(Debug)]
pub struct RTCRemoteOutboundRtpStreamStats(DartHandle);
impl RTCRemoteOutboundRtpStreamStats {
    /// Returns [`local_id`][1] of this [`RTCRemoteOutboundRtpStreamStats`].
    ///
    /// [1]: https://tinyurl.com/ysm7jdny
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn local_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_remote_outbound_rtp_stream_stats_local_id(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`remote_timestamp`][1] of this
    /// [`RTCRemoteOutboundRtpStreamStats`].
    ///
    /// [1]: https://tinyurl.com/ysm7jdny
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn remote_timestamp(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_remote_outbound_rtp_stream_stats_remote_timestamp(
                self.0.get(),
            )
            .unbox()
        })
        .unwrap()
    }

    /// Returns [`reports_sent`][1] of this [`RTCRemoteOutboundRtpStreamStats`].
    ///
    /// [1]: https://tinyurl.com/ysm7jdny
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
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
            remote_timestamp: stats.remote_timestamp().map(HighResTimeStamp),
            reports_sent: stats.reports_sent(),
        }
    }
}

/// Representation of [`RTCOutboundRTPStreamStats`][1].
///
/// [1]: https://tinyurl.com/4uw3fwsb
#[derive(Debug)]
pub struct RTCOutboundRTPStreamStats(DartHandle);
impl RTCOutboundRTPStreamStats {
    /// Returns [`track_id`][1] of this [`RTCOutboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/4uw3fwsb
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn track_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_track_id(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [`kind`][1] of this [`RTCOutboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/4uw3fwsb
    pub fn kind(&self) -> MediaKind {
        let index =
            unsafe { stats::rtc_outbound_rtp_stream_stats_kind(self.0.get()) };
        MediaKind::try_from(index).unwrap()
    }

    /// Returns [`frame_width`][1] of this [`RTCOutboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/4uw3fwsb
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn frame_width(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_frame_width(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`frame_height`][1] of this [`RTCOutboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/4uw3fwsb
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn frame_height(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_frame_height(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`frames_per_second`][1] of this [`RTCOutboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/4uw3fwsb
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn frames_per_second(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_frames_per_second(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`bytes_sent`][1] of this [`RTCOutboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/4uw3fwsb
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn bytes_sent(&self) -> Option<u64> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_bytes_sent(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`packets_sent`][1] of this [`RTCOutboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/4uw3fwsb
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn packets_sent(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_packets_sent(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`media_source_id`][1] of this [`RTCOutboundRTPStreamStats`].
    ///
    /// [1]: https://tinyurl.com/4uw3fwsb
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn media_source_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_outbound_rtp_stream_stats_media_source_id(self.0.get())
                .unbox()
        })
        .unwrap()
    }
}

impl From<&RTCOutboundRTPStreamStats> for RtcOutboundRtpStreamMediaType {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn from(stats: &RTCOutboundRTPStreamStats) -> Self {
        match stats.kind() {
            MediaKind::Audio => Self::Audio {
                total_samples_sent: None,  // TODO
                voice_activity_flag: None, // TODO
            },
            MediaKind::Video => Self::Video {
                frame_width: stats.frame_width().map(u64::from),
                frame_height: stats.frame_height().map(u64::from),
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
            packets_sent: stats.packets_sent().map(u64::from),
            media_source_id: stats.media_source_id(),
        }
    }
}

/// Representation of [`RTCVideoSourceStats`][1].
///
/// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats
#[derive(Debug)]
pub struct RTCVideoSourceStats(DartHandle);
impl RTCVideoSourceStats {
    /// Returns [width][1] of this [`RTCVideoSourceStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn width(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_video_source_stats_width(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [height][1] of this [`RTCVideoSourceStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn height(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_video_source_stats_height(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [frames][1] of this [`RTCVideoSourceStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn frames(&self) -> Option<u32> {
        Option::try_from(unsafe {
            stats::rtc_video_source_stats_frames(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [`frames_per_second`][1] of this [`RTCVideoSourceStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn frames_per_second(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_video_source_stats_frames_per_second(self.0.get())
                .unbox()
        })
        .unwrap()
    }
}

/// Representation of [`RTCAudioSourceStats`][1].
///
/// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
#[derive(Debug)]
pub struct RTCAudioSourceStats(DartHandle);
impl RTCAudioSourceStats {
    /// Returns [`audio_level`][1] of this [`RTCAudioSourceStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn audio_level(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_audio_source_stats_audio_level(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [`total_audio_energy`][1] of this [`RTCAudioSourceStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn total_audio_energy(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_audio_source_stats_total_audio_energy(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`total_samples_duration`][1] of this [`RTCAudioSourceStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn total_samples_duration(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_audio_source_stats_total_samples_duration(self.0.get())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`echo_return_loss`][1] of this [`RTCAudioSourceStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn echo_return_loss(&self) -> Option<f64> {
        Option::try_from(unsafe {
            stats::rtc_audio_source_stats_echo_return_loss(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [`echo_return_loss_enhancement`][1] of this
    /// [`RTCAudioSourceStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
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

/// Representation of [`RTCMediaSourceStats`][1] kind.
///
/// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcmediasourcestats
#[derive(Debug)]
pub enum RTCMediaSourceStatsType {
    /// Stats when kind is video.
    RTCVideoSourceStats(RTCVideoSourceStats),
    /// Stats when kind is audio.
    RTCAudioSourceStats(RTCAudioSourceStats),
}

impl From<DartHandle> for RTCMediaSourceStatsType {
    fn from(handle: DartHandle) -> Self {
        unsafe {
            let kind = dart_string_into_rust(
                stats::rtc_media_source_stats_class_type(handle.get()),
            );

            match kind.as_str() {
                "RTCAudioSourceStats" => Self::RTCAudioSourceStats(
                    RTCAudioSourceStats(DartHandle::new(
                        stats::cast_to_rtc_audio_source_stats(handle.get()),
                    )),
                ),
                "RTCVideoSourceStats" => Self::RTCVideoSourceStats(
                    RTCVideoSourceStats(DartHandle::new(
                        stats::cast_to_rtc_video_source_stats(handle.get()),
                    )),
                ),
                _ => unreachable!(),
            }
        }
    }
}

/// Representation of [`RTCMediaSourceStats`][1].
///
/// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcmediasourcestats
#[derive(Debug)]
pub struct RTCMediaSourceStats(DartHandle);
impl RTCMediaSourceStats {
    /// Returns [`track_identifier`][1] of this [`RTCMediaSourceStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcmediasourcestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn track_identifier(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_media_source_stats_track_identifier(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [`kind`][1] of this [`RTCMediaSourceStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcmediasourcestats
    pub fn kind(self) -> RTCMediaSourceStatsType {
        RTCMediaSourceStatsType::from(self.0)
    }
}

/// Representation of [`RTCIceCandidateStats`][1].
///
/// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
#[derive(Debug)]
pub enum RTCIceCandidateStats {
    /// Stats when [RTCIceCandidateStats] is local.
    RTCLocalIceCandidateStats(DartHandle),
    /// Stats when [RTCIceCandidateStats] is remote.
    RTCRemoteIceCandidateStats(DartHandle),
}

impl RTCIceCandidateStats {
    /// Returns [`Dart_Handle`] of this [`RTCIceCandidateStats`].
    fn get_handle(&self) -> Dart_Handle {
        match self {
            RTCIceCandidateStats::RTCRemoteIceCandidateStats(handle)
            | RTCIceCandidateStats::RTCLocalIceCandidateStats(handle) => {
                handle.get()
            }
        }
    }

    /// Returns [`transport_id`][1] of this [`RTCIceCandidateStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn transport_id(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_stats_transport_id(self.get_handle())
                .unbox()
        })
        .unwrap()
    }

    /// Returns [`address`][1] of this [`RTCIceCandidateStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn address(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_stats_address(self.get_handle()).unbox()
        })
        .unwrap()
    }

    /// Returns [`port`][1] of this [`RTCIceCandidateStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn port(&self) -> Option<i32> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_stats_priority(self.get_handle()).unbox()
        })
        .unwrap()
    }

    /// Returns [`protocol`][1] of this [`RTCIceCandidateStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
    pub fn protocol(&self) -> Protocol {
        Protocol::try_from(unsafe {
            stats::rtc_ice_candidate_stats_protocol(self.get_handle())
        })
        .unwrap()
    }

    /// Returns [`candidate_type`][1] of this [`RTCIceCandidateStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
    pub fn candidate_type(&self) -> CandidateType {
        let index = unsafe {
            stats::rtc_ice_candidate_stats_candidate_type(self.get_handle())
        };
        CandidateType::try_from(index).unwrap()
    }

    /// Returns [`priority`][1] of this [`RTCIceCandidateStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn priority(&self) -> Option<i32> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_stats_priority(self.get_handle()).unbox()
        })
        .unwrap()
    }

    /// Returns [`url`][1] of this [`RTCIceCandidateStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatestats
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn url(&self) -> Option<String> {
        Option::try_from(unsafe {
            stats::rtc_ice_candidate_stats_url(self.get_handle()).unbox()
        })
        .unwrap()
    }
}

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::fallible_impl_from)]
#[allow(clippy::cast_possible_truncation)]
impl From<&RTCIceCandidateStats> for RtcIceCandidateStats {
    fn from(stats: &RTCIceCandidateStats) -> Self {
        Self {
            transport_id: stats.transport_id(),
            address: stats.address(),
            port: stats.port().unwrap() as u16,
            protocol: NonExhaustive::Known(KnownProtocol::from(
                stats.protocol(),
            )),
            candidate_type: NonExhaustive::Known(KnownCandidateType::from(
                stats.candidate_type(),
            )),
            priority: stats.priority().unwrap() as u32,
            url: stats.url(),
            relay_protocol: None, // TODO
        }
    }
}

/// Representation of [`RTCStatsType`][1].
///
/// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstatstype
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
                "RTCAudioSourceStats" | "RTCVideoSourceStats" => {
                    Self::RTCMediaSourceStats(RTCMediaSourceStats(
                        DartHandle::new(stats::cast_to_rtc_media_source_stats(
                            inner,
                        )),
                    ))
                }
                "RTCLocalIceCandidateStats" => Self::RTCIceCandidateStats(
                    RTCIceCandidateStats::RTCLocalIceCandidateStats(
                        DartHandle::new(
                            stats::cast_to_rtc_ice_candidate_stats(inner),
                        ),
                    ),
                ),
                "RTCRemoteIceCandidateStats" => Self::RTCIceCandidateStats(
                    RTCIceCandidateStats::RTCRemoteIceCandidateStats(
                        DartHandle::new(
                            stats::cast_to_rtc_ice_candidate_stats(inner),
                        ),
                    ),
                ),
                "RTCOutboundRTPStreamStats" => Self::RTCOutboundRTPStreamStats(
                    RTCOutboundRTPStreamStats(DartHandle::new(
                        stats::cast_to_rtc_outbound_rtp_stream_stats(inner),
                    )),
                ),
                "RTCInboundRTPStreamStats" => Self::RTCInboundRTPStreamStats(
                    RTCInboundRTPStreamStats(DartHandle::new(
                        stats::cast_to_rtc_inbound_rtp_stream_stats(inner),
                    )),
                ),
                "RTCIceCandidatePairStats" => Self::RTCIceCandidatePairStats(
                    RTCIceCandidatePairStats(DartHandle::new(
                        stats::cast_to_rtc_ice_candidate_pair_stats(inner),
                    )),
                ),
                "RTCTransportStats" => {
                    Self::RTCTransportStats(RTCTransportStats(DartHandle::new(
                        stats::cast_to_rtc_transport_stats(inner),
                    )))
                }
                "RTCRemoteInboundRtpStreamStats" => {
                    Self::RTCRemoteInboundRtpStreamStats(
                        RTCRemoteInboundRtpStreamStats(DartHandle::new(
                            stats::cast_to_rtc_remote_inbound_rtp_stream_stats(
                                inner,
                            ),
                        )),
                    )
                }
                "RTCRemoteOutboundRtpStreamStats" => {
                    Self::RTCRemoteOutboundRtpStreamStats(
                        RTCRemoteOutboundRtpStreamStats(DartHandle::new(
                            stats::cast_to_rtc_remote_outbound_rtp_stream_stats(
                                inner,
                            ),
                        )),
                    )
                }
                _ => Self::Unimplenented,
            }
        }
    }
}

/// Representation of [RTCStats][1].
///
/// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
#[derive(Debug)]
pub struct RTCStats(pub DartHandle);

impl RTCStats {
    /// Returns [id][1] of this [`RTCStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
    pub fn id(&self) -> String {
        unsafe { dart_string_into_rust(stats::rtc_stats_id(self.0.get())) }
    }

    /// Returns [`timestamp_us`][1] of this [`RTCStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
    pub fn timestamp_us(&self) -> i64 {
        unsafe { stats::rtc_stats_timestamp_us(self.0.get()) }
    }

    /// Returns [kind][1] of this [`RTCStats`].
    ///
    /// [1]: https://www.w3.org/TR/webrtc-stats/#dom-rtcstats
    pub fn kind(&self) -> RTCStatsType {
        unsafe { RTCStatsType::from(stats::rtc_stats_kind(self.0.get())) }
    }
}

impl From<RTCMediaSourceStatsType>
    for medea_client_api_proto::stats::MediaKind
{
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
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
                    audio_level: stats.audio_level().map(Float),
                    total_audio_energy: stats.total_audio_energy().map(Float),
                    total_samples_duration: stats
                        .total_samples_duration()
                        .map(Float),
                }
            }
        }
    }
}

impl From<RTCMediaSourceStats> for MediaSourceStats {
    fn from(stats: RTCMediaSourceStats) -> Self {
        Self {
            track_identifier: stats.track_identifier(),
            kind: medea_client_api_proto::stats::MediaKind::from(stats.kind()),
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
                let candidate = RtcIceCandidateStats::from(&stats);
                match stats {
                    RTCIceCandidateStats::RTCLocalIceCandidateStats(_) => {
                        Self::LocalCandidate(Box::new(candidate))
                    }
                    RTCIceCandidateStats::RTCRemoteIceCandidateStats(_) => {
                        Self::RemoteCandidate(Box::new(candidate))
                    }
                }
            }
            RTCStatsType::Unimplenented => Self::Other,
        }
    }
}

impl From<RTCStats> for RtcStat {
    #[allow(clippy::cast_precision_loss)]
    fn from(stats: RTCStats) -> Self {
        let id = stats.id();
        let time = HighResTimeStamp(stats.timestamp_us() as f64);
        let kind = stats.kind();
        let stats_type = RtcStatsType::from(kind);
        Self {
            id: StatId(id),
            timestamp: time,
            stats: stats_type,
        }
    }
}
