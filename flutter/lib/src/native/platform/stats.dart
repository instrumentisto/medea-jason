import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'stats.g.dart' as bridge;

void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    rtcStatsKind: Pointer.fromFunction(_rtcStatsKind),
    rtcStatsTimestampUs: Pointer.fromFunction(_rtcStatsTimestampUs, 0),
    rtcStatsId: Pointer.fromFunction(_rtcStatsId),
    rtcStatsType: Pointer.fromFunction(_rtcStatsType),
    rtcMediaSourceStatsTrackIdentifier:
        Pointer.fromFunction(_rtcMediaSourceStatsTrackIdentifier),
    rtcIceCandidateStatsTransportId:
        Pointer.fromFunction(_rtcIceCandidateStatsTransportId),
    rtcIceCandidateStatsAddress:
        Pointer.fromFunction(_rtcIceCandidateStatsAddress),
    rtcIceCandidateStatsPort: Pointer.fromFunction(_rtcIceCandidateStatsPort),
    rtcIceCandidateStatsProtocol:
        Pointer.fromFunction(_rtcIceCandidateStatsProtocol, 0),
    rtcIceCandidateStatsCandidateType:
        Pointer.fromFunction(_rtcIceCandidateStatsCandidateType, 0),
    rtcIceCandidateStatsPriority:
        Pointer.fromFunction(_rtcIceCandidateStatsPriority),
    rtcIceCandidateStatsUrl: Pointer.fromFunction(_rtcIceCandidateStatsUrl),
    rtcOutboundRtpStreamStatsTrackId:
        Pointer.fromFunction(_rtcOutboundRtpStreamStatsTrackId),
    rtcOutboundRtpStreamStatsKind:
        Pointer.fromFunction(_rtcOutboundRtpStreamStatsKind, 0),
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
    rtcInboundRtpStreamAudioTotalSamplesReceived:
        Pointer.fromFunction(_rtcInboundRtpStreamAudioTotalSamplesReceived),
    rtcInboundRtpStreamAudioConcealedSamples:
        Pointer.fromFunction(_rtcInboundRtpStreamAudioConcealedSamples),
    rtcInboundRtpStreamAudioSilentConcealedSamples:
        Pointer.fromFunction(_rtcInboundRtpStreamAudioSilentConcealedSamples),
    rtcInboundRtpStreamAudioAudioLevel:
        Pointer.fromFunction(_rtcInboundRtpStreamAudioAudioLevel),
    rtcInboundRtpStreamAudioTotalAudioEnergy:
        Pointer.fromFunction(_rtcInboundRtpStreamAudioTotalAudioEnergy),
    rtcInboundRtpStreamAudioTotalSamplesDuration:
        Pointer.fromFunction(_rtcInboundRtpStreamAudioTotalSamplesDuration),
    rtcInboundRtpStreamVideoFramesDecoded:
        Pointer.fromFunction(_rtcInboundRtpStreamVideoFramesDecoded),
    rtcInboundRtpStreamVideoKeyFramesDecoded:
        Pointer.fromFunction(_rtcInboundRtpStreamVideoKeyFramesDecoded),
    rtcInboundRtpStreamVideoFrameWidth:
        Pointer.fromFunction(_rtcInboundRtpStreamVideoFrameWidth),
    rtcInboundRtpStreamVideoFrameHeight:
        Pointer.fromFunction(_rtcInboundRtpStreamVideoFrameHeight),
    rtcInboundRtpStreamVideoTotalInterFrameDelay:
        Pointer.fromFunction(_rtcInboundRtpStreamVideoTotalInterFrameDelay),
    rtcInboundRtpStreamVideoFramesPerSecond:
        Pointer.fromFunction(_rtcInboundRtpStreamVideoFramesPerSecond),
    rtcInboundRtpStreamVideoFrameBitDepth:
        Pointer.fromFunction(_rtcInboundRtpStreamVideoFrameBitDepth),
    rtcInboundRtpStreamVideoFirCount:
        Pointer.fromFunction(_rtcInboundRtpStreamVideoFirCount),
    rtcInboundRtpStreamVideoPliCount:
        Pointer.fromFunction(_rtcInboundRtpStreamVideoPliCount),
    rtcInboundRtpStreamVideoConcealmentEvents:
        Pointer.fromFunction(_rtcInboundRtpStreamVideoConcealmentEvents),
    rtcInboundRtpStreamVideoFramesReceived:
        Pointer.fromFunction(_rtcInboundRtpStreamVideoFramesReceived),
    rtcIceCandidatePairStatsState:
        Pointer.fromFunction(_rtcIceCandidatePairStatsState, 0),
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
    rtcStatsCastToRtcInboundRtpStreamStats:
        Pointer.fromFunction(_rtcStatsCastToRtcInboundRtpStreamStats),
    rtcStatsCastToRtcOutboundRtpStreamStats:
        Pointer.fromFunction(_rtcStatsCastToRtcOutboundRtpStreamStats),
    rtcInboundRtpStreamMediaTypeCastToAudio:
        Pointer.fromFunction(_rtcInboundRtpStreamMediaTypeCastToAudio),
    rtcInboundRtpStreamMediaTypeCastToVideo:
        Pointer.fromFunction(_rtcInboundRtpStreamMediaTypeCastToVideo),
    rtcInboundRtpStreamStatsMediaType:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsMediaType),
    rtcInboundRtpStreamStatsMediaTypeClass:
        Pointer.fromFunction(_rtcInboundRtpStreamStatsMediaTypeClass),
    rtcMediaSourceStatsClassType:
        Pointer.fromFunction(_rtcMediaSourceStatsClassType),
  );
}

/// Returns [RTCStatsType] of the provided [RTCStats].
Object _rtcStatsKind(RTCStats stats) {
  return stats.type;
}

/// Returns timestampUs of the provided [RTCStats].
int _rtcStatsTimestampUs(RTCStats stats) {
  return stats.timestampUs;
}

/// Returns id of the provided [RTCStats].
Pointer<Utf8> _rtcStatsId(RTCStats stats) {
  return stats.id.toNativeUtf8();
}

/// Returns runtime type of the provided [RTCMediaSourceStats].
Pointer<Utf8> _rtcMediaSourceStatsClassType(RTCMediaSourceStats stats) {
  return stats.runtimeType.toString().toNativeUtf8();
}

/// Returns trackIdentifier of the provided [RTCMediaSourceStats].
Pointer _rtcMediaSourceStatsTrackIdentifier(RTCMediaSourceStats stats) {
  if (stats.trackIdentifier != null) {
    return ForeignValue.fromString(stats.trackIdentifier!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns transportId of the provided [RTCIceCandidateStats].
Pointer _rtcIceCandidateStatsTransportId(RTCIceCandidateStats stats) {
  if (stats.transportId != null) {
    return ForeignValue.fromString(stats.transportId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns address of the provided [RTCIceCandidateStats].
Pointer _rtcIceCandidateStatsAddress(RTCIceCandidateStats stats) {
  if (stats.address != null) {
    return ForeignValue.fromString(stats.address!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns port of the provided [RTCIceCandidateStats].
Pointer _rtcIceCandidateStatsPort(RTCIceCandidateStats stats) {
  if (stats.port != null) {
    return ForeignValue.fromInt(stats.port!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns index of [Protocol] of the provided [RTCIceCandidateStats].
int _rtcIceCandidateStatsProtocol(RTCIceCandidateStats stats) {
  return stats.protocol.index;
}

/// Returns index of [CandidateType] of the provided [RTCIceCandidateStats].
int _rtcIceCandidateStatsCandidateType(RTCIceCandidateStats stats) {
  return stats.candidateType.index;
}

/// Returns priority of the provided [RTCIceCandidateStats].
Pointer _rtcIceCandidateStatsPriority(RTCIceCandidateStats stats) {
  if (stats.priority != null) {
    return ForeignValue.fromInt(stats.priority!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns url of the provided [RTCIceCandidateStats].
Pointer _rtcIceCandidateStatsUrl(RTCIceCandidateStats stats) {
  if (stats.url != null) {
    return ForeignValue.fromString(stats.url!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns trackId of the provided [RTCOutboundRTPStreamStats].
Pointer _rtcOutboundRtpStreamStatsTrackId(RTCOutboundRTPStreamStats stats) {
  if (stats.trackId != null) {
    return ForeignValue.fromString(stats.trackId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns index of [TrackKind] of the provided [RTCOutboundRTPStreamStats].
int _rtcOutboundRtpStreamStatsKind(RTCOutboundRTPStreamStats stats) {
  return stats.kind.index;
}

/// Returns bytesSent of the provided [RTCOutboundRTPStreamStats].
Pointer _rtcOutboundRtpStreamStatsBytesSent(RTCOutboundRTPStreamStats stats) {
  if (stats.bytesSent != null) {
    return ForeignValue.fromInt(stats.bytesSent!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns packetsSent of the provided [RTCOutboundRTPStreamStats].
Pointer _rtcOutboundRtpStreamStatsPacketsSent(RTCOutboundRTPStreamStats stats) {
  if (stats.packetsSent != null) {
    return ForeignValue.fromInt(stats.packetsSent!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns mediaSourceId of the provided [RTCOutboundRTPStreamStats].
Pointer _rtcOutboundRtpStreamStatsMediaSourceId(
    RTCOutboundRTPStreamStats stats) {
  if (stats.mediaSourceId != null) {
    return ForeignValue.fromString(stats.mediaSourceId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns frameWidth of the provided [RTCOutboundRTPStreamStats].
Pointer _rtcOutboundRtpStreamStatsFrameWidth(RTCOutboundRTPStreamStats stats) {
  if (stats.frameWidth != null) {
    return ForeignValue.fromInt(stats.frameWidth!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns frameHeight of the provided [RTCOutboundRTPStreamStats].
Pointer _rtcOutboundRtpStreamStatsFrameHeight(RTCOutboundRTPStreamStats stats) {
  if (stats.frameHeight != null) {
    return ForeignValue.fromInt(stats.frameHeight!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns framesPerSecond of the provided [RTCOutboundRTPStreamStats].
Pointer _rtcOutboundRtpStreamStatsFramesPerSecond(
    RTCOutboundRTPStreamStats stats) {
  if (stats.framesPerSecond != null) {
    return ForeignValue.fromDouble(stats.framesPerSecond!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns remoteId of the provided [RTCInboundRTPStreamStats].
Pointer _rtcInboundRtpStreamStatsRemoteId(RTCInboundRTPStreamStats stats) {
  if (stats.remoteId != null) {
    return ForeignValue.fromString(stats.remoteId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns bytesReceived of the provided [RTCInboundRTPStreamStats].
Pointer _rtcInboundRtpStreamStatsBytesReceived(RTCInboundRTPStreamStats stats) {
  if (stats.bytesReceived != null) {
    return ForeignValue.fromInt(stats.bytesReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns packetsReceived of the provided [RTCInboundRTPStreamStats].
Pointer _rtcInboundRtpStreamStatsPacketsReceived(
    RTCInboundRTPStreamStats stats) {
  if (stats.packetsReceived != null) {
    return ForeignValue.fromInt(stats.packetsReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns totalDecodeTime of the provided [RTCInboundRTPStreamStats].
Pointer _rtcInboundRtpStreamStatsTotalDecodeTime(
    RTCInboundRTPStreamStats stats) {
  if (stats.totalDecodeTime != null) {
    return ForeignValue.fromDouble(stats.totalDecodeTime!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns jitterBufferEmittedCount of the provided [RTCInboundRTPStreamStats].
Pointer _rtcInboundRtpStreamStatsJitterBufferEmittedCount(
    RTCInboundRTPStreamStats stats) {
  if (stats.jitterBufferEmittedCount != null) {
    return ForeignValue.fromInt(stats.jitterBufferEmittedCount!)
        .intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns totalSamplesReceived of the provided [RTCInboundRTPStreamAudio].
Pointer _rtcInboundRtpStreamAudioTotalSamplesReceived(
    RTCInboundRTPStreamAudio stats) {
  if (stats.totalSamplesReceived != null) {
    return ForeignValue.fromInt(stats.totalSamplesReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns concealedSamples of the provided [RTCInboundRTPStreamAudio].
Pointer _rtcInboundRtpStreamAudioConcealedSamples(
    RTCInboundRTPStreamAudio stats) {
  if (stats.concealedSamples != null) {
    return ForeignValue.fromInt(stats.concealedSamples!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns silentConcealedSamples of the provided [RTCInboundRTPStreamAudio].
Pointer _rtcInboundRtpStreamAudioSilentConcealedSamples(
    RTCInboundRTPStreamAudio stats) {
  if (stats.silentConcealedSamples != null) {
    return ForeignValue.fromInt(stats.silentConcealedSamples!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns audioLevel of the provided [RTCInboundRTPStreamAudio].
Pointer _rtcInboundRtpStreamAudioAudioLevel(RTCInboundRTPStreamAudio stats) {
  if (stats.audioLevel != null) {
    return ForeignValue.fromDouble(stats.audioLevel!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns totalAudioEnergy of the provided [RTCInboundRTPStreamAudio].
Pointer _rtcInboundRtpStreamAudioTotalAudioEnergy(
    RTCInboundRTPStreamAudio stats) {
  if (stats.totalAudioEnergy != null) {
    return ForeignValue.fromDouble(stats.totalAudioEnergy!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns totalSamplesDuration of the provided [RTCInboundRTPStreamAudio].
Pointer _rtcInboundRtpStreamAudioTotalSamplesDuration(
    RTCInboundRTPStreamAudio stats) {
  if (stats.totalSamplesDuration != null) {
    return ForeignValue.fromDouble(stats.totalSamplesDuration!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns framesDecoded of the provided [RTCInboundRTPStreamVideo].
Pointer _rtcInboundRtpStreamVideoFramesDecoded(RTCInboundRTPStreamVideo stats) {
  if (stats.framesDecoded != null) {
    return ForeignValue.fromInt(stats.framesDecoded!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns keyFramesDecoded of the provided [RTCInboundRTPStreamVideo].
Pointer _rtcInboundRtpStreamVideoKeyFramesDecoded(
    RTCInboundRTPStreamVideo stats) {
  if (stats.keyFramesDecoded != null) {
    return ForeignValue.fromInt(stats.keyFramesDecoded!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns frameWidth of the provided [RTCInboundRTPStreamVideo].
Pointer _rtcInboundRtpStreamVideoFrameWidth(RTCInboundRTPStreamVideo stats) {
  if (stats.frameWidth != null) {
    return ForeignValue.fromInt(stats.frameWidth!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns frameHeight of the provided [RTCInboundRTPStreamVideo].
Pointer _rtcInboundRtpStreamVideoFrameHeight(RTCInboundRTPStreamVideo stats) {
  if (stats.frameHeight != null) {
    return ForeignValue.fromInt(stats.frameHeight!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns totalInterFrameDelay of the provided [RTCInboundRTPStreamVideo].
Pointer _rtcInboundRtpStreamVideoTotalInterFrameDelay(
    RTCInboundRTPStreamVideo stats) {
  if (stats.totalInterFrameDelay != null) {
    return ForeignValue.fromDouble(stats.totalInterFrameDelay!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns framesPerSecond of the provided [RTCInboundRTPStreamVideo].
Pointer _rtcInboundRtpStreamVideoFramesPerSecond(
    RTCInboundRTPStreamVideo stats) {
  if (stats.framesPerSecond != null) {
    return ForeignValue.fromDouble(stats.framesPerSecond!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns frameBitDepth of the provided [RTCInboundRTPStreamVideo].
Pointer _rtcInboundRtpStreamVideoFrameBitDepth(RTCInboundRTPStreamVideo stats) {
  if (stats.frameBitDepth != null) {
    return ForeignValue.fromInt(stats.frameBitDepth!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns firCount of the provided [RTCInboundRTPStreamVideo].
Pointer _rtcInboundRtpStreamVideoFirCount(RTCInboundRTPStreamVideo stats) {
  if (stats.firCount != null) {
    return ForeignValue.fromInt(stats.firCount!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns pliCount of the provided [RTCInboundRTPStreamVideo].
Pointer _rtcInboundRtpStreamVideoPliCount(RTCInboundRTPStreamVideo stats) {
  if (stats.pliCount != null) {
    return ForeignValue.fromInt(stats.pliCount!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns concealmentEvents of the provided [RTCInboundRTPStreamVideo].
Pointer _rtcInboundRtpStreamVideoConcealmentEvents(
    RTCInboundRTPStreamVideo stats) {
  if (stats.concealmentEvents != null) {
    return ForeignValue.fromInt(stats.concealmentEvents!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns framesReceived of the provided [RTCInboundRTPStreamVideo].
Pointer _rtcInboundRtpStreamVideoFramesReceived(
    RTCInboundRTPStreamVideo stats) {
  if (stats.framesReceived != null) {
    return ForeignValue.fromInt(stats.framesReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns index of [RTCStatsIceCandidatePairState] of the provided [RTCIceCandidatePairStats].
int _rtcIceCandidatePairStatsState(RTCIceCandidatePairStats stats) {
  return stats.state.index;
}

/// Returns nominated of the provided [RTCIceCandidatePairStats].
Pointer _rtcIceCandidatePairStatsNominated(RTCIceCandidatePairStats stats) {
  if (stats.nominated != null) {
    return ForeignValue.fromBool(stats.nominated!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns bytesSent of the provided [RTCIceCandidatePairStats].
Pointer _rtcIceCandidatePairStatsBytesSent(RTCIceCandidatePairStats stats) {
  if (stats.bytesSent != null) {
    return ForeignValue.fromInt(stats.bytesSent!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns bytesReceived of the provided [RTCIceCandidatePairStats].
Pointer _rtcIceCandidatePairStatsBytesReceived(RTCIceCandidatePairStats stats) {
  if (stats.bytesReceived != null) {
    return ForeignValue.fromInt(stats.bytesReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns totalRoundTripTime of the provided [RTCIceCandidatePairStats].
Pointer _rtcIceCandidatePairStatsTotalRoundTripTime(
    RTCIceCandidatePairStats stats) {
  if (stats.totalRoundTripTime != null) {
    return ForeignValue.fromDouble(stats.totalRoundTripTime!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns currentRoundTripTime of the provided [RTCIceCandidatePairStats].
Pointer _rtcIceCandidatePairStatsCurrentRoundTripTime(
    RTCIceCandidatePairStats stats) {
  if (stats.currentRoundTripTime != null) {
    return ForeignValue.fromDouble(stats.currentRoundTripTime!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns availableOutgoingBitrate of the provided [RTCIceCandidatePairStats].
Pointer _rtcIceCandidatePairStatsAvailableOutgoingBitrate(
    RTCIceCandidatePairStats stats) {
  if (stats.availableOutgoingBitrate != null) {
    return ForeignValue.fromDouble(stats.availableOutgoingBitrate!)
        .intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns packetsSent of the provided [RTCTransportStats].
Pointer _rtcTransportStatsPacketsSent(RTCTransportStats stats) {
  if (stats.packetsSent != null) {
    return ForeignValue.fromInt(stats.packetsSent!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns packetsReceived of the provided [RTCTransportStats].
Pointer _rtcTransportStatsPacketsReceived(RTCTransportStats stats) {
  if (stats.packetsReceived != null) {
    return ForeignValue.fromInt(stats.packetsReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns bytesSent of the provided [RTCTransportStats].
Pointer _rtcTransportStatsBytesSent(RTCTransportStats stats) {
  if (stats.bytesSent != null) {
    return ForeignValue.fromInt(stats.bytesSent!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns bytesReceived of the provided [RTCTransportStats].
Pointer _rtcTransportStatsBytesReceived(RTCTransportStats stats) {
  if (stats.bytesReceived != null) {
    return ForeignValue.fromInt(stats.bytesReceived!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns localId of the provided [RTCRemoteInboundRtpStreamStats].
Pointer _rtcRemoteInboundRtpStreamStatsLocalId(
    RTCRemoteInboundRtpStreamStats stats) {
  if (stats.localId != null) {
    return ForeignValue.fromString(stats.localId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns roundTripTime of the provided [RTCRemoteInboundRtpStreamStats].
Pointer _rtcRemoteInboundRtpStreamStatsRoundTripTime(
    RTCRemoteInboundRtpStreamStats stats) {
  if (stats.roundTripTime != null) {
    return ForeignValue.fromDouble(stats.roundTripTime!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns fractionLost of the provided [RTCRemoteInboundRtpStreamStats].
Pointer _rtcRemoteInboundRtpStreamStatsFractionLost(
    RTCRemoteInboundRtpStreamStats stats) {
  if (stats.fractionLost != null) {
    return ForeignValue.fromDouble(stats.fractionLost!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns roundTripTimeMeasurements of the provided [RTCRemoteInboundRtpStreamStats].
Pointer _rtcRemoteInboundRtpStreamStatsRoundTripTimeMeasurements(
    RTCRemoteInboundRtpStreamStats stats) {
  if (stats.roundTripTimeMeasurements != null) {
    return ForeignValue.fromInt(stats.roundTripTimeMeasurements!)
        .intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns localId of the provided [RTCRemoteOutboundRtpStreamStats].
Pointer _rtcRemoteOutboundRtpStreamStatsLocalId(
    RTCRemoteOutboundRtpStreamStats stats) {
  if (stats.localId != null) {
    return ForeignValue.fromString(stats.localId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns remoteTimestamp of the provided [RTCRemoteOutboundRtpStreamStats].
Pointer _rtcRemoteOutboundRtpStreamStatsRemoteTimestamp(
    RTCRemoteOutboundRtpStreamStats stats) {
  if (stats.remoteTimestamp != null) {
    return ForeignValue.fromDouble(stats.remoteTimestamp!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns reportsSent of the provided [RTCRemoteOutboundRtpStreamStats].
Pointer _rtcRemoteOutboundRtpStreamStatsReportsSent(
    RTCRemoteOutboundRtpStreamStats stats) {
  if (stats.reportsSent != null) {
    return ForeignValue.fromInt(stats.reportsSent!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns width of the provided [RTCVideoSourceStats].
Pointer _rtcVideoSourceStatsWidth(RTCVideoSourceStats stats) {
  if (stats.width != null) {
    return ForeignValue.fromInt(stats.width!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns height of the provided [RTCVideoSourceStats].
Pointer _rtcVideoSourceStatsHeight(RTCVideoSourceStats stats) {
  if (stats.height != null) {
    return ForeignValue.fromInt(stats.height!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns frames of the provided [RTCVideoSourceStats].
Pointer _rtcVideoSourceStatsFrames(RTCVideoSourceStats stats) {
  if (stats.frames != null) {
    return ForeignValue.fromInt(stats.frames!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns framesPerSecond of the provided [RTCVideoSourceStats].
Pointer _rtcVideoSourceStatsFramesPerSecond(RTCVideoSourceStats stats) {
  if (stats.framesPerSecond != null) {
    return ForeignValue.fromDouble(stats.framesPerSecond!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns audioLevel of the provided [RTCAudioSourceStats].
Pointer _rtcAudioSourceStatsAudioLevel(RTCAudioSourceStats stats) {
  if (stats.audioLevel != null) {
    return ForeignValue.fromDouble(stats.audioLevel!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns totalAudioEnergy of the provided [RTCAudioSourceStats].
Pointer _rtcAudioSourceStatsTotalAudioEnergy(RTCAudioSourceStats stats) {
  if (stats.totalAudioEnergy != null) {
    return ForeignValue.fromDouble(stats.totalAudioEnergy!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns totalSamplesDuration of the provided [RTCAudioSourceStats].
Pointer _rtcAudioSourceStatsTotalSamplesDuration(RTCAudioSourceStats stats) {
  if (stats.totalSamplesDuration != null) {
    return ForeignValue.fromDouble(stats.totalSamplesDuration!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns echoReturnLoss of the provided [RTCAudioSourceStats].
Pointer _rtcAudioSourceStatsEchoReturnLoss(RTCAudioSourceStats stats) {
  if (stats.echoReturnLoss != null) {
    return ForeignValue.fromDouble(stats.echoReturnLoss!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns echoReturnLossEnhancement of the provided [RTCAudioSourceStats].
Pointer _rtcAudioSourceStatsEchoReturnLossEnhancement(
    RTCAudioSourceStats stats) {
  if (stats.echoReturnLossEnhancement != null) {
    return ForeignValue.fromDouble(stats.echoReturnLossEnhancement!)
        .intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns runtime type of the provided [RTCStatsType].
Pointer<Utf8> _rtcStatsType(RTCStatsType stats) {
  return stats.runtimeType.toString().toNativeUtf8();
}

/// Cast the provided [RTCStatsType]
/// to a [RTCInboundRTPStreamStats].
Object _rtcStatsCastToRtcInboundRtpStreamStats(RTCStatsType stats) {
  return stats as RTCInboundRTPStreamStats;
}

/// Cast the provided [RTCStatsType]
/// to a [RTCOutboundRTPStreamStats].
Object _rtcStatsCastToRtcOutboundRtpStreamStats(RTCStatsType stats) {
  return stats as RTCOutboundRTPStreamStats;
}

/// Cast the provided [RTCStatsType]
/// to a [RTCMediaSourceStats].
Object _rtcStatsCastToRtcMediaSourceStats(RTCStatsType stats) {
  return stats as RTCMediaSourceStats;
}

/// Cast the provided [RTCStatsType]
/// to a [RTCVideoSourceStats].
Object _rtcMediaSourceStatsCastToRtcVideoSourceStats(
    RTCMediaSourceStats stats) {
  return stats as RTCVideoSourceStats;
}

/// Cast the provided [RTCStatsType]
/// to a [RTCAudioSourceStats].
Object _rtcMediaSourceStatsCastToRtcAudioSourceStats(
    RTCMediaSourceStats stats) {
  return stats as RTCAudioSourceStats;
}

/// Cast the provided [RTCStatsType]
/// to a [RTCIceCandidateStats].
Object _rtcStatsCastToRtcIceCandidateStats(RTCStatsType stats) {
  return stats as RTCIceCandidateStats;
}

/// Cast the provided [RTCStatsType]
/// to a [RTCIceCandidatePairStats].
Object _rtcStatsCastToRtcIceCandidatePairStats(RTCStatsType stats) {
  return stats as RTCIceCandidatePairStats;
}

/// Cast the provided [RTCStatsType]
/// to a [RTCTransportStats].
Object _rtcStatsCastToRtcTransportStats(RTCStatsType stats) {
  return stats as RTCTransportStats;
}

/// Cast the provided [RTCStatsType]
/// to a [RTCRemoteInboundRtpStreamStats].
Object _rtcStatsCastToRtcRemoteInboundRtpStreamStats(RTCStatsType stats) {
  return stats as RTCRemoteInboundRtpStreamStats;
}

/// Cast the provided [RTCStatsType]
/// to a [RTCRemoteOutboundRtpStreamStats].
Object _rtcStatsCastToRtcRemoteOutboundRtpStreamStats(RTCStatsType stats) {
  return stats as RTCRemoteOutboundRtpStreamStats;
}

/// Cast the provided [RTCInboundRTPStreamMediaType]
/// to a [RTCInboundRTPStreamAudio].
Object _rtcInboundRtpStreamMediaTypeCastToAudio(
    RTCInboundRTPStreamMediaType stats) {
  return stats as RTCInboundRTPStreamAudio;
}

/// Cast the provided [RTCInboundRTPStreamMediaType]
/// to a [RTCInboundRTPStreamVideo].
Object _rtcInboundRtpStreamMediaTypeCastToVideo(
    RTCInboundRTPStreamMediaType stats) {
  return stats as RTCInboundRTPStreamVideo;
}

/// Returns [RTCInboundRTPStreamMediaType] of the provided [RTCInboundRTPStreamMediaType].
Object _rtcInboundRtpStreamStatsMediaType(RTCInboundRTPStreamStats stats) {
  if (stats.mediaType == null) {
    return ForeignValue.none().intoRustOwned();
  } else {
    return ForeignValue.fromHandle(stats.mediaType!).intoRustOwned();
  }
}

/// Returns runtime type of the provided [RTCInboundRTPStreamMediaType].
Pointer<Utf8> _rtcInboundRtpStreamStatsMediaTypeClass(
    RTCInboundRTPStreamMediaType stats) {
  return stats.runtimeType.toString().toNativeUtf8();
}
