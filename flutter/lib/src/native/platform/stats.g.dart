import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Handle Function(Handle)>> rtcStatsKind,
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> rtcStatsType,
  required Pointer<NativeFunction<Int64 Function(Handle)>> rtcStatsTimestampUs,
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> rtcStatsId,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcMediaSourceStatsTrackIdentifier,
  required Pointer<NativeFunction<Handle Function(Handle)>>
      rtcStatsCastToRtcMediaSourceStats,
  required Pointer<NativeFunction<Handle Function(Handle)>>
      rtcMediaSourceStatsCastToRtcVideoSourceStats,
  required Pointer<NativeFunction<Handle Function(Handle)>>
      rtcMediaSourceStatsCastToRtcAudioSourceStats,
  required Pointer<NativeFunction<Handle Function(Handle)>>
      rtcStatsCastToRtcIceCandidateStats,
  required Pointer<NativeFunction<Handle Function(Handle)>>
      rtcStatsCastToRtcIceCandidatePairStats,
  required Pointer<NativeFunction<Handle Function(Handle)>>
      rtcStatsCastToRtcTransportStats,
  required Pointer<NativeFunction<Handle Function(Handle)>>
      rtcStatsCastToRtcRemoteInboundRtpStreamStats,
  required Pointer<NativeFunction<Handle Function(Handle)>>
      rtcStatsCastToRtcRemoteOutboundRtpStreamStats,
  required Pointer<NativeFunction<Handle Function(Handle)>>
      rtcStatsCastToRtcInboundRtpStreamStats,
  required Pointer<NativeFunction<Handle Function(Handle)>>
      rtcStatsCastToRtcOutboundRtpStreamStats,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidateStatsTransportId,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidateStatsAddress,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidateStatsPort,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidateStatsProtocol,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidateStatsCandidateType,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidateStatsPriority,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidateStatsUrl,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcOutboundRtpStreamStatsTrackId,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcOutboundRtpStreamStatsKind,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcOutboundRtpStreamStatsBytesSent,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcOutboundRtpStreamStatsPacketsSent,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcOutboundRtpStreamStatsMediaSourceId,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcOutboundRtpStreamStatsFrameWidth,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcOutboundRtpStreamStatsFrameHeight,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcOutboundRtpStreamStatsFramesPerSecond,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsRemoteId,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsBytesReceived,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsPacketsReceived,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsTotalDecodeTime,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsJitterBufferEmittedCount,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsTotalSamplesReceived,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsConcealedSamples,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsSilentConcealedSamples,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsAudioLevel,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsTotalAudioEnergy,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsTotalSamplesDuration,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsFramesDecoded,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsKeyFramesDecoded,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsFrameWidth,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsFrameHeight,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsTotalInterFrameDelay,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsFramesPerSecond,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsFrameBitDepth,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsFirCount,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsPliCount,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsConcealmentEvents,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcInboundRtpStreamStatsFramesReceived,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidatePairStatsState,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidatePairStatsNominated,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidatePairStatsBytesSent,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidatePairStatsBytesReceived,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidatePairStatsTotalRoundTripTime,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidatePairStatsCurrentRoundTripTime,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcIceCandidatePairStatsAvailableOutgoingBitrate,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcTransportStatsPacketsSent,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcTransportStatsPacketsReceived,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcTransportStatsBytesSent,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcTransportStatsBytesReceived,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcRemoteInboundRtpStreamStatsLocalId,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcRemoteInboundRtpStreamStatsRoundTripTime,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcRemoteInboundRtpStreamStatsFractionLost,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcRemoteInboundRtpStreamStatsRoundTripTimeMeasurements,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcRemoteOutboundRtpStreamStatsLocalId,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcRemoteOutboundRtpStreamStatsRemoteTimestamp,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcRemoteOutboundRtpStreamStatsReportsSent,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcVideoSourceStatsWidth,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcVideoSourceStatsHeight,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcVideoSourceStatsFrames,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcVideoSourceStatsFramesPerSecond,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcAudioSourceStatsAudioLevel,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcAudioSourceStatsTotalAudioEnergy,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcAudioSourceStatsTotalSamplesDuration,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcAudioSourceStatsEchoReturnLoss,
  required Pointer<NativeFunction<Pointer Function(Handle)>>
      rtcAudioSourceStatsEchoReturnLossEnhancement,
}) {
  dl.lookupFunction<
      Void Function(
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer),
      void Function(
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer)>('register_stats')(
    rtcStatsKind,
    rtcStatsType,
    rtcStatsTimestampUs,
    rtcStatsId,
    rtcMediaSourceStatsTrackIdentifier,
    rtcStatsCastToRtcMediaSourceStats,
    rtcMediaSourceStatsCastToRtcVideoSourceStats,
    rtcMediaSourceStatsCastToRtcAudioSourceStats,
    rtcStatsCastToRtcIceCandidateStats,
    rtcStatsCastToRtcIceCandidatePairStats,
    rtcStatsCastToRtcTransportStats,
    rtcStatsCastToRtcRemoteInboundRtpStreamStats,
    rtcStatsCastToRtcRemoteOutboundRtpStreamStats,
    rtcStatsCastToRtcInboundRtpStreamStats,
    rtcStatsCastToRtcOutboundRtpStreamStats,
    rtcIceCandidateStatsTransportId,
    rtcIceCandidateStatsAddress,
    rtcIceCandidateStatsPort,
    rtcIceCandidateStatsProtocol,
    rtcIceCandidateStatsCandidateType,
    rtcIceCandidateStatsPriority,
    rtcIceCandidateStatsUrl,
    rtcOutboundRtpStreamStatsTrackId,
    rtcOutboundRtpStreamStatsKind,
    rtcOutboundRtpStreamStatsBytesSent,
    rtcOutboundRtpStreamStatsPacketsSent,
    rtcOutboundRtpStreamStatsMediaSourceId,
    rtcOutboundRtpStreamStatsFrameWidth,
    rtcOutboundRtpStreamStatsFrameHeight,
    rtcOutboundRtpStreamStatsFramesPerSecond,
    rtcInboundRtpStreamStatsRemoteId,
    rtcInboundRtpStreamStatsBytesReceived,
    rtcInboundRtpStreamStatsPacketsReceived,
    rtcInboundRtpStreamStatsTotalDecodeTime,
    rtcInboundRtpStreamStatsJitterBufferEmittedCount,
    rtcInboundRtpStreamStatsTotalSamplesReceived,
    rtcInboundRtpStreamStatsConcealedSamples,
    rtcInboundRtpStreamStatsSilentConcealedSamples,
    rtcInboundRtpStreamStatsAudioLevel,
    rtcInboundRtpStreamStatsTotalAudioEnergy,
    rtcInboundRtpStreamStatsTotalSamplesDuration,
    rtcInboundRtpStreamStatsFramesDecoded,
    rtcInboundRtpStreamStatsKeyFramesDecoded,
    rtcInboundRtpStreamStatsFrameWidth,
    rtcInboundRtpStreamStatsFrameHeight,
    rtcInboundRtpStreamStatsTotalInterFrameDelay,
    rtcInboundRtpStreamStatsFramesPerSecond,
    rtcInboundRtpStreamStatsFrameBitDepth,
    rtcInboundRtpStreamStatsFirCount,
    rtcInboundRtpStreamStatsPliCount,
    rtcInboundRtpStreamStatsConcealmentEvents,
    rtcInboundRtpStreamStatsFramesReceived,
    rtcIceCandidatePairStatsState,
    rtcIceCandidatePairStatsNominated,
    rtcIceCandidatePairStatsBytesSent,
    rtcIceCandidatePairStatsBytesReceived,
    rtcIceCandidatePairStatsTotalRoundTripTime,
    rtcIceCandidatePairStatsCurrentRoundTripTime,
    rtcIceCandidatePairStatsAvailableOutgoingBitrate,
    rtcTransportStatsPacketsSent,
    rtcTransportStatsPacketsReceived,
    rtcTransportStatsBytesSent,
    rtcTransportStatsBytesReceived,
    rtcRemoteInboundRtpStreamStatsLocalId,
    rtcRemoteInboundRtpStreamStatsRoundTripTime,
    rtcRemoteInboundRtpStreamStatsFractionLost,
    rtcRemoteInboundRtpStreamStatsRoundTripTimeMeasurements,
    rtcRemoteOutboundRtpStreamStatsLocalId,
    rtcRemoteOutboundRtpStreamStatsRemoteTimestamp,
    rtcRemoteOutboundRtpStreamStatsReportsSent,
    rtcVideoSourceStatsWidth,
    rtcVideoSourceStatsHeight,
    rtcVideoSourceStatsFrames,
    rtcVideoSourceStatsFramesPerSecond,
    rtcAudioSourceStatsAudioLevel,
    rtcAudioSourceStatsTotalAudioEnergy,
    rtcAudioSourceStatsTotalSamplesDuration,
    rtcAudioSourceStatsEchoReturnLoss,
    rtcAudioSourceStatsEchoReturnLossEnhancement,
  );
}
