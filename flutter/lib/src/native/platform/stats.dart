import 'dart:ffi';
import 'dart:html';

import 'package:flutter/foundation.dart';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'stats.g.dart' as bridge;

void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    rtcStatsKind: Pointer.fromFunction(_rtcStatsKind),
    rtcStatsTimestampUs: Pointer.fromFunction(_rtcStatsTimestampUs),
    rtcStatsId: Pointer.fromFunction(_rtcStatsId),
    rtcMediaSourceStatsTrackIdentifier:
        Pointer.fromFunction(_rtcMediaSourceStatsTrackIdentifier),
    rtcIceCandidateStatsTransportId:
        Pointer.fromFunction(_rtcIceCandidateStatsTransportId),
    rtcIceCandidateStatsAddress:
        Pointer.fromFunction(_rtcIceCandidateStatsAddress),
    rtcIceCandidateStatsPort: Pointer.fromFunction(_rtcIceCandidateStatsPort),
    rtcIceCandidateStatsProtocol:
        Pointer.fromFunction(_rtcIceCandidateStatsProtocol),
    rtcIceCandidateStatsCandidateType:
        Pointer.fromFunction(_rtcIceCandidateStatsCandidateType),
    rtcIceCandidateStatsPriority:
        Pointer.fromFunction(_rtcIceCandidateStatsPriority),
    rtcIceCandidateStatsUrl: Pointer.fromFunction(_rtcIceCandidateStatsUrl),
    rtcOutboundRtpStreamStatsTrackId:
        Pointer.fromFunction(_rtcOutboundRtpStreamStatsTrackId),
    rtcOutboundRtpStreamStatsKind:
        Pointer.fromFunction(_rtcOutboundRtpStreamStatsKind),
    rtcOutboundRtpStreamStatsBytesSent:
        Pointer.fromFunction(_rtcOutboundRtpStreamStatsBytesSent),
    rtcOutboundRtpStreamStatsPacketsSent:
        Pointer.fromFunction(_rtcOutboundRtpStreamStatsPacketsSent),
    rtcOutboundRtpStreamStatsMediaSourceId:
        Pointer.fromFunction(_rtcOutboundRtpStreamStatsMediaSourceId),
    rtcOutboundRtpStreamStatsFrameWidth:
        Pointer.fromFunction(_rtcOutboundRtpStreamStatsFrameWidth),
    rtcOutboundRtpStreamStatsFrameHeight:
        Pointer.fromFunction(_rtcOutboundRtpStreamStatsFrameHeight),
    rtcOutboundRtpStreamStatsFramesPerSecond:
        Pointer.fromFunction(_rtcOutboundRtpStreamStatsFramesPerSecond),
    rtcInboundRtpStreamStatsRemoteId:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsRemoteId),
    rtcInboundRtpStreamStatsBytesReceived:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsBytesReceived),
    rtcInboundRtpStreamStatsPacketsReceived:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsPacketsReceived),
    rtcInboundRtpStreamStatsTotalDecodeTime:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsTotalDecodeTime),
    rtcInboundRtpStreamStatsJitterBufferEmittedCount:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsJitterBufferEmittedCount),
    rtcInboundRtpStreamStatsTotalSamplesReceived:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsTotalSamplesReceived),
    rtcInboundRtpStreamStatsConcealedSamples:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsConcealedSamples),
    rtcInboundRtpStreamStatsSilentConcealedSamples:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsSilentConcealedSamples),
    rtcInboundRtpStreamStatsAudioLevel:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsAudioLevel),
    rtcInboundRtpStreamStatsTotalAudioEnergy:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsTotalAudioEnergy),
    rtcInboundRtpStreamStatsTotalSamplesDuration:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsTotalSamplesDuration),
    rtcInboundRtpStreamStatsFramesDecoded:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsFramesDecoded),
    rtcInboundRtpStreamStatsKeyFramesDecoded:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsKeyFramesDecoded),
    rtcInboundRtpStreamStatsFrameWidth:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsFrameWidth),
    rtcInboundRtpStreamStatsFrameHeight:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsFrameHeight),
    rtcInboundRtpStreamStatsTotalInterFrameDelay:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsTotalInterFrameDelay),
    rtcInboundRtpStreamStatsFramesPerSecond:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsFramesPerSecond),
    rtcInboundRtpStreamStatsFrameBitDepth:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsFrameBitDepth),
    rtcInboundRtpStreamStatsFirCount:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsFirCount),
    rtcInboundRtpStreamStatsPliCount:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsPliCount),
    rtcInboundRtpStreamStatsConcealmentEvents:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsConcealmentEvents),
    rtcInboundRtpStreamStatsFramesReceived:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsFramesReceived),
    rtcIceCandidatePairStatsState:
        Pointer.fromFunction(_rtcIceCandidatePairStatsState),
    rtcIceCandidatePairStatsNominated:
        Pointer.fromFunction(_rtcIceCandidatePairStatsNominated),
    rtcIceCandidatePairStatsBytesSent:
        Pointer.fromFunction(_rtcIceCandidatePairStatsBytesSent),
    rtcIceCandidatePairStatsBytesReceived:
        Pointer.fromFunction(_rtcIceCandidatePairStatsBytesReceived),
    rtcIceCandidatePairStatsTotalRoundTripTime:
        Pointer.fromFunction(_rtcIceCandidatePairStatsTotalRoundTripTime),
    rtcIceCandidatePairStatsCurrentRoundTripTime:
        Pointer.fromFunction(_rtcIceCandidatePairStatsCurrentRoundTripTime),
    rtcIceCandidatePairStatsAvailableOutgoingBitrate:
        Pointer.fromFunction(_rtcIceCandidatePairStatsAvailableOutgoingBitrate),
    rtcTransportStatsPacketsSent:
        Pointer.fromFunction(_rtcTransportStatsPacketsSent),
    rtcTransportStatsPacketsReceived:
        Pointer.fromFunction(_rtcTransportStatsPacketsReceived),
    rtcTransportStatsBytesSent:
        Pointer.fromFunction(_rtcTransportStatsBytesSent),
    rtcTransportStatsBytesReceived:
        Pointer.fromFunction(_rtcTransportStatsBytesReceived),
    rtcRemoteInboundRtpStreamStatsLocalId:
        Pointer.fromFunction(_rtcRemoteInboundRtpStreamStatsLocalId),
    rtcRemoteInboundRtpStreamStatsRoundTripTime:
        Pointer.fromFunction(_rtcRemoteInboundRtpStreamStatsRoundTripTime),
    rtcRemoteInboundRtpStreamStatsFractionLost:
        Pointer.fromFunction(_rtcRemoteInboundRtpStreamStatsFractionLost),
    rtcRemoteInboundRtpStreamStatsRoundTripTimeMeasurements:
        Pointer.fromFunction(
            _rtcRemoteInboundRtpStreamStatsRoundTripTimeMeasurements),
    rtcRemoteOutboundRtpStreamStatsLocalId:
        Pointer.fromFunction(_rtcRemoteOutboundRtpStreamStatsLocalId),
    rtcRemoteOutboundRtpStreamStatsRemoteTimestamp:
        Pointer.fromFunction(_rtcRemoteOutboundRtpStreamStatsRemoteTimestamp),
    rtcRemoteOutboundRtpStreamStatsReportsSent:
        Pointer.fromFunction(_rtcRemoteOutboundRtpStreamStatsReportsSent),
    rtcVideoSourceStatsWidth: Pointer.fromFunction(_rtcVideoSourceStatsWidth),
    rtcVideoSourceStatsHeight: Pointer.fromFunction(_rtcVideoSourceStatsHeight),
    rtcVideoSourceStatsFrames: Pointer.fromFunction(_rtcVideoSourceStatsFrames),
    rtcVideoSourceStatsFramesPerSecond:
        Pointer.fromFunction(_rtcVideoSourceStatsFramesPerSecond),
    rtcAudioSourceStatsAudioLevel:
        Pointer.fromFunction(_rtcAudioSourceStatsAudioLevel),
    rtcAudioSourceStatsTotalAudioEnergy:
        Pointer.fromFunction(_rtcAudioSourceStatsTotalAudioEnergy),
    rtcAudioSourceStatsTotalSamplesDuration:
        Pointer.fromFunction(_rtcAudioSourceStatsTotalSamplesDuration),
    rtcAudioSourceStatsEchoReturnLoss:
        Pointer.fromFunction(_rtcAudioSourceStatsEchoReturnLoss),
    rtcAudioSourceStatsEchoReturnLossEnhancement:
        Pointer.fromFunction(_rtcAudioSourceStatsEchoReturnLossEnhancement),
    rtcStatsCastToRtcMediaSourceStats:
        Pointer.fromFunction(_rtcStatsCastToRtcMediaSourceStats),
    rtcMediaSourceStatsCastToRtcVideoSourceStats:
        Pointer.fromFunction(_rtcMediaSourceStatsCastToRtcVideoSourceStats),
    rtcMediaSourceStatsCastToRtcAudioSourceStats:
        Pointer.fromFunction(_rtcMediaSourceStatsCastToRtcAudioSourceStats),
    rtcStatsCastToRtcIceCandidateStats:
        Pointer.fromFunction(_rtcStatsCastToRtcIceCandidateStats),
    rtcStatsCastToRtcIceCandidatePairStats:
        Pointer.fromFunction(_rtcStatsCastToRtcIceCandidatePairStats),
    rtcStatsCastToRtcTransportStats:
        Pointer.fromFunction(_rtcStatsCastToRtcTransportStats),
    rtcStatsCastToRtcRemoteInboundRtpStreamStats:
        Pointer.fromFunction(_rtcStatsCastToRtcRemoteInboundRtpStreamStats),
    rtcStatsCastToRtcRemoteOutboundRtpStreamStats:
        Pointer.fromFunction(_rtcStatsCastToRtcRemoteOutboundRtpStreamStats),
    rtcStatsType: Pointer.fromFunction(_rtcStatsType),
    rtcStatsCastToRtcInboundRtpStreamStats:
        Pointer.fromFunction(_rtcStatsCastToRtcInboundRtpStreamStats),
    rtcStatsCastToRtcOutboundRtpStreamStats:
        Pointer.fromFunction(_rtcStatsCastToRtcOutboundRtpStreamStats),
  );
}

Object _rtcStatsKind(RTCStats stats) {
  return stats.type;
}

Int64 _rtcStatsTimestampUs(RTCStats stats) {
  return (stats.timestampUs as Int64);
}

Pointer<Utf8> _rtcStatsId(RTCStats stats) {
  return stats.id.toNativeUtf8();
}

Pointer _rtcMediaSourceStatsTrackIdentifier(RTCMediaSourceStats stats) {
  if (stats.trackIdentifier != null) {
    return ForeignValue.fromString(stats.trackIdentifier!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcIceCandidateStatsTransportId(RTCIceCandidateStats stats) {
  if (stats.transportId != null) {
    return ForeignValue.fromString(stats.transportId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcIceCandidateStatsAddress(RTCIceCandidateStats stats) {
  if (stats.address != null) {
    return ForeignValue.fromString(stats.address!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcIceCandidateStatsPort(RTCIceCandidateStats stats) {
  if (stats.port != null) {
    return ForeignValue.fromInt(stats.port!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcIceCandidateStatsProtocol(RTCIceCandidateStats stats) {
  if (stats.protocol != null) {
    return ForeignValue.fromString(stats.protocol!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Object _rtcIceCandidateStatsCandidateType(RTCIceCandidateStats stats) {
  return stats.candidateType;
}

Pointer _rtcIceCandidateStatsPriority(RTCIceCandidateStats stats) {
  if (stats.priority != null) {
    return ForeignValue.fromInt(stats.priority!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcIceCandidateStatsUrl(RTCIceCandidateStats stats) {
  if (stats.url != null) {
    return ForeignValue.fromString(stats.url!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcOutboundRtpStreamStatsTrackId(RTCOutboundRTPStreamStats stats) {
  if (stats.trackId != null) {
    return ForeignValue.fromString(stats.trackId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Object _rtcOutboundRtpStreamStatsKind(RTCOutboundRTPStreamStats stats) {
  return stats.kind;
}

Pointer _rtcOutboundRtpStreamStatsBytesSent(RTCOutboundRTPStreamStats stats) {
  if (stats.bytesSent != null) {
    return ForeignValue.fromInt(stats.bytesSent!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcOutboundRtpStreamStatsPacketsSent(RTCOutboundRTPStreamStats stats) {
  if (stats.packetsSent != null) {
    return ForeignValue.fromInt(stats.packetsSent!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcOutboundRtpStreamStatsMediaSourceId(
    RTCOutboundRTPStreamStats stats) {
  if (stats.mediaSourceId != null) {
    return ForeignValue.fromString(stats.mediaSourceId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcOutboundRtpStreamStatsFrameWidth(RTCOutboundRTPStreamStats stats) {
  if (stats.frameWidth != null) {
    return ForeignValue.fromInt(stats.frameWidth!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcOutboundRtpStreamStatsFrameHeight(RTCOutboundRTPStreamStats stats) {
  if (stats.frameHeight != null) {
    return ForeignValue.fromInt(stats.frameHeight!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcOutboundRtpStreamStatsFramesPerSecond(
    RTCOutboundRTPStreamStats stats) {
  if (stats.framesPerSecond != null) {
    return ForeignValue.fromDouble(stats.framesPerSecond!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsRemoteId(RTCInboundRTPStreamStats stats) {
  if (stats.remoteId != null) {
    return ForeignValue.fromString(stats.remoteId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsBytesReceived(RTCInboundRTPStreamStats stats) {
  if (stats.bytesReceived != null) {
    return ForeignValue.fromInt(stats.bytesReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsPacketsReceived(
    RTCInboundRTPStreamStats stats) {
  if (stats.packetsReceived != null) {
    return ForeignValue.fromInt(stats.packetsReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsTotalDecodeTime(
    RTCInboundRTPStreamStats stats) {
  if (stats.totalDecodeTime != null) {
    return ForeignValue.fromDouble(stats.totalDecodeTime!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsJitterBufferEmittedCount(
    RTCInboundRTPStreamStats stats) {
  if (stats.jitterBufferEmittedCount != null) {
    return ForeignValue.fromInt(stats.jitterBufferEmittedCount!)
        .intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsTotalSamplesReceived(
    RTCInboundRTPStreamStats stats) {
  if (stats.totalSamplesReceived != null) {
    return ForeignValue.fromInt(stats.totalSamplesReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsConcealedSamples(
    RTCInboundRTPStreamStats stats) {
  if (stats.concealedSamples != null) {
    return ForeignValue.fromInt(stats.concealedSamples!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsSilentConcealedSamples(
    RTCInboundRTPStreamStats stats) {
  if (stats.silentConcealedSamples != null) {
    return ForeignValue.fromInt(stats.silentConcealedSamples!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsAudioLevel(RTCInboundRTPStreamStats stats) {
  if (stats.audioLevel != null) {
    return ForeignValue.fromDouble(stats.audioLevel!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsTotalAudioEnergy(
    RTCInboundRTPStreamStats stats) {
  if (stats.totalAudioEnergy != null) {
    return ForeignValue.fromDouble(stats.totalAudioEnergy!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsTotalSamplesDuration(
    RTCInboundRTPStreamStats stats) {
  if (stats.totalSamplesDuration != null) {
    return ForeignValue.fromDouble(stats.totalSamplesDuration!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsFramesDecoded(RTCInboundRTPStreamStats stats) {
  if (stats.framesDecoded != null) {
    return ForeignValue.fromInt(stats.framesDecoded!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsKeyFramesDecoded(
    RTCInboundRTPStreamStats stats) {
  if (stats.keyFramesDecoded != null) {
    return ForeignValue.fromInt(stats.keyFramesDecoded!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsFrameWidth(RTCInboundRTPStreamStats stats) {
  if (stats.frameWidth != null) {
    return ForeignValue.fromInt(stats.frameWidth!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsFrameHeight(RTCInboundRTPStreamStats stats) {
  if (stats.frameHeight != null) {
    return ForeignValue.fromInt(stats.frameHeight!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsTotalInterFrameDelay(
    RTCInboundRTPStreamStats stats) {
  if (stats.totalInterFrameDelay != null) {
    return ForeignValue.fromDouble(stats.totalInterFrameDelay!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsFramesPerSecond(
    RTCInboundRTPStreamStats stats) {
  if (stats.framesPerSecond != null) {
    return ForeignValue.fromDouble(stats.framesPerSecond!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsFrameBitDepth(RTCInboundRTPStreamStats stats) {
  if (stats.frameBitDepth != null) {
    return ForeignValue.fromInt(stats.frameBitDepth!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsFirCount(RTCInboundRTPStreamStats stats) {
  if (stats.firCount != null) {
    return ForeignValue.fromInt(stats.firCount!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsPliCount(RTCInboundRTPStreamStats stats) {
  if (stats.pliCount != null) {
    return ForeignValue.fromInt(stats.pliCount!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsConcealmentEvents(
    RTCInboundRTPStreamStats stats) {
  if (stats.concealmentEvents != null) {
    return ForeignValue.fromInt(stats.concealmentEvents!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcInboundRtpStreamStatsFramesReceived(
    RTCInboundRTPStreamStats stats) {
  if (stats.framesReceived != null) {
    return ForeignValue.fromInt(stats.framesReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Object _rtcIceCandidatePairStatsState(RTCIceCandidatePairStats stats) {
  return stats.state;
}

Pointer _rtcIceCandidatePairStatsNominated(RTCIceCandidatePairStats stats) {
  if (stats.nominated != null) {
    return ForeignValue.fromBool(stats.nominated!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcIceCandidatePairStatsBytesSent(RTCIceCandidatePairStats stats) {
  if (stats.bytesSent != null) {
    return ForeignValue.fromInt(stats.bytesSent!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcIceCandidatePairStatsBytesReceived(RTCIceCandidatePairStats stats) {
  if (stats.bytesReceived != null) {
    return ForeignValue.fromInt(stats.bytesReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcIceCandidatePairStatsTotalRoundTripTime(
    RTCIceCandidatePairStats stats) {
  if (stats.totalRoundTripTime != null) {
    return ForeignValue.fromDouble(stats.totalRoundTripTime!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcIceCandidatePairStatsCurrentRoundTripTime(
    RTCIceCandidatePairStats stats) {
  if (stats.currentRoundTripTime != null) {
    return ForeignValue.fromDouble(stats.currentRoundTripTime!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcIceCandidatePairStatsAvailableOutgoingBitrate(
    RTCIceCandidatePairStats stats) {
  if (stats.availableOutgoingBitrate != null) {
    return ForeignValue.fromDouble(stats.availableOutgoingBitrate!)
        .intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcTransportStatsPacketsSent(RTCTransportStats stats) {
  if (stats.packetsSent != null) {
    return ForeignValue.fromInt(stats.packetsSent!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcTransportStatsPacketsReceived(RTCTransportStats stats) {
  if (stats.packetsReceived != null) {
    return ForeignValue.fromInt(stats.packetsReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcTransportStatsBytesSent(RTCTransportStats stats) {
  if (stats.bytesSent != null) {
    return ForeignValue.fromInt(stats.bytesSent!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcTransportStatsBytesReceived(RTCTransportStats stats) {
  if (stats.bytesReceived != null) {
    return ForeignValue.fromInt(stats.bytesReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcRemoteInboundRtpStreamStatsLocalId(
    RTCRemoteInboundRtpStreamStats stats) {
  if (stats.localId != null) {
    return ForeignValue.fromString(stats.localId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcRemoteInboundRtpStreamStatsRoundTripTime(
    RTCRemoteInboundRtpStreamStats stats) {
  if (stats.roundTripTime != null) {
    return ForeignValue.fromDouble(stats.roundTripTime!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcRemoteInboundRtpStreamStatsFractionLost(
    RTCRemoteInboundRtpStreamStats stats) {
  if (stats.fractionLost != null) {
    return ForeignValue.fromDouble(stats.fractionLost!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcRemoteInboundRtpStreamStatsRoundTripTimeMeasurements(
    RTCRemoteInboundRtpStreamStats stats) {
  if (stats.roundTripTimeMeasurements != null) {
    return ForeignValue.fromInt(stats.roundTripTimeMeasurements!)
        .intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcRemoteOutboundRtpStreamStatsLocalId(
    RTCRemoteOutboundRtpStreamStats stats) {
  if (stats.localId != null) {
    return ForeignValue.fromString(stats.localId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcRemoteOutboundRtpStreamStatsRemoteTimestamp(
    RTCRemoteOutboundRtpStreamStats stats) {
  if (stats.remoteTimestamp != null) {
    return ForeignValue.fromDouble(stats.remoteTimestamp!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcRemoteOutboundRtpStreamStatsReportsSent(
    RTCRemoteOutboundRtpStreamStats stats) {
  if (stats.reportsSent != null) {
    return ForeignValue.fromInt(stats.reportsSent!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcVideoSourceStatsWidth(RTCVideoSourceStats stats) {
  if (stats.width != null) {
    return ForeignValue.fromInt(stats.width!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcVideoSourceStatsHeight(RTCVideoSourceStats stats) {
  if (stats.height != null) {
    return ForeignValue.fromInt(stats.height!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcVideoSourceStatsFrames(RTCVideoSourceStats stats) {
  if (stats.frames != null) {
    return ForeignValue.fromInt(stats.frames!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcVideoSourceStatsFramesPerSecond(RTCVideoSourceStats stats) {
  if (stats.framesPerSecond != null) {
    return ForeignValue.fromDouble(stats.framesPerSecond!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcAudioSourceStatsAudioLevel(RTCAudioSourceStats stats) {
  if (stats.audioLevel != null) {
    return ForeignValue.fromDouble(stats.audioLevel!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcAudioSourceStatsTotalAudioEnergy(RTCAudioSourceStats stats) {
  if (stats.totalAudioEnergy != null) {
    return ForeignValue.fromDouble(stats.totalAudioEnergy!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcAudioSourceStatsTotalSamplesDuration(RTCAudioSourceStats stats) {
  if (stats.totalSamplesDuration != null) {
    return ForeignValue.fromDouble(stats.totalSamplesDuration!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcAudioSourceStatsEchoReturnLoss(RTCAudioSourceStats stats) {
  if (stats.echoReturnLoss != null) {
    return ForeignValue.fromDouble(stats.echoReturnLoss!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer _rtcAudioSourceStatsEchoReturnLossEnhancement(
    RTCAudioSourceStats stats) {
  if (stats.echoReturnLoss != null) {
    return ForeignValue.fromDouble(stats.echoReturnLoss!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

Pointer<Utf8> _rtcStatsType(RTCStats stats) {
  return stats.type.runtimeType.toString().toNativeUtf8();
}

Object _rtcStatsCastToRtcInboundRtpStreamStats(RTCStatsType stats) {
  return stats as RTCInboundRTPStreamStats;
}

Object _rtcStatsCastToRtcOutboundRtpStreamStats(RTCStatsType stats) {
  return stats as RTCOutboundRTPStreamStats;
}

Object _rtcStatsCastToRtcMediaSourceStats(RTCStatsType stats) {
  return stats as RTCMediaSourceStats;
}

Object _rtcMediaSourceStatsCastToRtcVideoSourceStats(
    RTCMediaSourceStats stats) {
  return stats as RTCVideoSourceStats;
}

Object _rtcMediaSourceStatsCastToRtcAudioSourceStats(
    RTCMediaSourceStats stats) {
  return stats as RTCAudioSourceStats;
}

Object _rtcStatsCastToRtcIceCandidateStats(RTCStatsType stats) {
  return stats as RTCIceCandidateStats;
}

Object _rtcStatsCastToRtcIceCandidatePairStats(RTCStatsType stats) {
  return stats as RTCIceCandidatePairStats;
}

Object _rtcStatsCastToRtcTransportStats(RTCStatsType stats) {
  return stats as RTCTransportStats;
}

Object _rtcStatsCastToRtcRemoteInboundRtpStreamStats(RTCStatsType stats) {
  return stats as RTCRemoteInboundRtpStreamStats;
}

Object _rtcStatsCastToRtcRemoteOutboundRtpStreamStats(RTCStatsType stats) {
  return stats as RTCRemoteOutboundRtpStreamStats;
}
