import 'dart:convert';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

extension RtcMediaSourceStatsMapConverter on RtcMediaSourceStats {
  Map<String, dynamic> toMap() {
    var kind;
    var additionalData = {};
    if (this is RtcAudioSourceStats) {
      var stat = this as RtcAudioSourceStats;
      kind = 'audio';
      additionalData = {
        'audioLevel': stat.audioLevel,
        'totalAudioEnergy': stat.totalAudioEnergy,
        'totalSamplesDuration': stat.totalSamplesDuration,
        'echoReturnLoss': stat.echoReturnLoss,
        'echoReturnLossEnhancement': stat.echoReturnLossEnhancement,
      };
    } else if (this is RtcVideoSourceStats) {
      var stat = this as RtcVideoSourceStats;
      kind = 'video';
      additionalData = {
        'width': stat.width,
        'height': stat.height,
        'frames': stat.frames,
        'framesPerSecond': stat.framesPerSecond,
      };
    } else {
      throw 'Unreachable';
    }
    return {
      'trackIdentifier': this.trackIdentifier,
      'kind': kind,
      ...additionalData
    };
  }
}

extension RtcIceCandidateStatsMapConverter on RtcIceCandidateStats {
  Map<String, dynamic> toMap() {
    var type;
    if (this is RtcLocalIceCandidateStats) {
      type = "local-candidate";
    } else if (this is RtcRemoteIceCandidateStats) {
      type = "remote-candidate";
    } else {
      throw 'Unreachable';
    }
    return {
      'type': type,
      'transportId': this.transportId,
      'address': this.address,
      'port': this.port,
      'protocol': this.protocol.toJsonString(),
      'candidateType': this.candidateType.toJsonString(),
      'priority': this.priority,
      'url': this.url,
      'relayProtocol': this.relayProtocol.toJsonString(),
    };
  }
}

extension CandidateTypeJsonStringConverter on CandidateType {
  String toJsonString() {
    return switch (this) {
      CandidateType.host => 'host',
      CandidateType.srflx => 'srflx',
      CandidateType.prflx => 'prflx',
      CandidateType.relay => 'relay',
    };
  }
}

extension ProtocolJsonStringConverter on Protocol? {
  String? toJsonString() {
    return switch (this) {
      Protocol.tcp => 'tcp',
      Protocol.udp => 'udp',
      null => null
    };
  }
}

extension RtcOutboundRtpStreamStatsMapConverter on RtcOutboundRtpStreamStats {
  Map<String, dynamic> toMap() {
    var additionalData = {};
    var mediaTypeString;
    if (this.mediaType is RtcOutboundRtpStreamStatsAudio) {
      var mediaType = this.mediaType as RtcOutboundRtpStreamStatsAudio;
      mediaTypeString = 'audio';
      additionalData = {
        'totalSamplesSent': mediaType.totalSamplesSent,
        'voiceActivityFlag': mediaType.voiceActivityFlag,
      };
    } else if (this.mediaType is RtcOutboundRtpStreamStatsVideo) {
      var mediaType = this.mediaType as RtcOutboundRtpStreamStatsVideo;
      mediaTypeString = 'video';
      additionalData = {
        'frameWidth': mediaType.frameWidth,
        'frameHeight': mediaType.frameHeight,
        'framesPerSecond': mediaType.framesPerSecond,
      };
    } else if (this.mediaType == null) {
      // It can be null, so do nothing.
    } else {
      throw 'Unreachable';
    }
    return {
      'trackId': this.trackId,
      'mediaType': mediaTypeString,
      'bytesSent': this.bytesSent,
      'packetsSent': this.packetsSent,
      'mediaSourceId': this.mediaSourceId,
      ...additionalData,
    };
  }
}

extension RtcInboundRtpStreamStatsMapConverter on RtcInboundRtpStreamStats {
  Map<String, dynamic> toMap() {
    var additionalData = {};
    var mediaTypeString;
    if (this.mediaType is RtcInboundRtpStreamAudio) {
      var mediaType = this.mediaType as RtcInboundRtpStreamAudio;
      mediaTypeString = 'audio';
      additionalData = {
        'totalSamplesReceived': mediaType.totalSamplesReceived,
        'concealedSamples': mediaType.concealedSamples,
        'silentConcealedSamples': mediaType.silentConcealedSamples,
        'audioLevel': mediaType.audioLevel,
        'totalAudioEnergy': mediaType.totalAudioEnergy,
        'totalSamplesDuration': mediaType.totalSamplesDuration,
        'voiceActivityFlag': mediaType.voiceActivityFlag,
      };
    } else if (this.mediaType is RtcInboundRtpStreamVideo) {
      mediaTypeString = 'video';
      var mediaType = this.mediaType as RtcInboundRtpStreamVideo;
      additionalData = {
        'framesDecoded': mediaType.framesDecoded,
        'keyFramesDecoded': mediaType.keyFramesDecoded,
        'frameWidth': mediaType.frameWidth,
        'frameHeight': mediaType.frameHeight,
        'totalInterFrameDelay': mediaType.totalInterFrameDelay,
        'framesPerSecond': mediaType.framesPerSecond,
        'firCount': mediaType.firCount,
        'pliCount': mediaType.pliCount,
        'concealmentEvents': mediaType.concealmentEvents,
        'framesReceived': mediaType.framesReceived,
        'sliCount': mediaType.sliCount,
      };
    } else if (this.mediaType == null) {
      // Do nothing, because it can be null
    } else {
      throw 'Unreachable';
    }
    return {
      'mediaType': mediaTypeString,
      'remoteId': this.remoteId,
      'bytesReceived': this.bytesReceived,
      'packetsReceived': this.packetsReceived,
      'totalDecodeTime': this.totalDecodeTime,
      'jitterBufferEmittedCount': this.jitterBufferEmittedCount,
      ...additionalData,
    };
  }
}

extension RtcIceCandidatePairStatsMapConverter on RtcIceCandidatePairStats {
  Map<String, dynamic> toMap() {
    return {
      'state': this.state.toJsonString(),
      'nominated': this.nominated,
      'bytesSent': this.bytesSent,
      'bytesReceived': this.bytesReceived,
      'totalRoundTripTime': this.totalRoundTripTime,
      'currentRoundTripTime': this.currentRoundTripTime,
      'availableOutgoingBitrate': this.availableOutgoingBitrate?.toInt(),
    };
  }
}

extension RtcStatsIceCandidatePairStateJsonStringConverter
    on RtcStatsIceCandidatePairState {
  String toJsonString() {
    return switch (this) {
      RtcStatsIceCandidatePairState.frozen => 'frozen',
      RtcStatsIceCandidatePairState.waiting => 'waiting',
      RtcStatsIceCandidatePairState.inProgress => 'inProgress',
      RtcStatsIceCandidatePairState.failed => 'failed',
      RtcStatsIceCandidatePairState.succeeded => 'succeeded',
    };
  }
}

extension RtcTransportStatsMapConverter on RtcTransportStats {
  Map<String, dynamic> toMap() {
    return {
      'packetsSent': this.packetsSent,
      'packetsReceived': this.packetsReceived,
      'bytesSent': this.bytesSent,
      'bytesReceived': this.bytesReceived,
      'iceRole': this.iceRole.toJsonString(),
    };
  }
}

extension IceRoleJsonStringConverter on IceRole? {
  String? toJsonString() {
    return switch (this) {
      IceRole.unknown => 'unknown',
      IceRole.controlling => 'controlling',
      IceRole.controlled => 'controlled',
      null => null
    };
  }
}

extension RtcRemoteInboundRtpStreamStatsMapConverter
    on RtcRemoteInboundRtpStreamStats {
  Map<String, dynamic> toMap() {
    return {
      'localId': this.localId,
      'roundTripTime': this.roundTripTime,
      'fractionLost': this.fractionLost,
      'roundTripTimeMeasurements': this.roundTripTimeMeasurements,
      'jitter': this.jitter,
      'reportsReceived': this.reportsReceived,
    };
  }
}

extension RtcRemoteOutboundRtpStreamStatsMapConverter
    on RtcRemoteOutboundRtpStreamStats {
  Map<String, dynamic> toMap() {
    return {
      'localId': this.localId,
      'remoteTimestamp': this.remoteTimestamp,
      'reportsSent': this.reportsSent,
    };
  }
}

extension RtcStatsTypeMapConverter on RtcStatsType {
  Map<String, dynamic> toMap() {
    var statsName;
    var additionalData = {};

    if (this is RtcMediaSourceStats) {
      var stats_type = this as RtcMediaSourceStats;
      statsName = 'media-source';
      additionalData = stats_type.toMap();
    } else if (this is RtcIceCandidateStats) {
      var stats_type = this as RtcIceCandidateStats;
      additionalData = stats_type.toMap();
    } else if (this is RtcOutboundRtpStreamStats) {
      var stats_type = this as RtcOutboundRtpStreamStats;
      statsName = 'outbound-rtp';
      additionalData = stats_type.toMap();
    } else if (this is RtcInboundRtpStreamStats) {
      var stats_type = this as RtcInboundRtpStreamStats;
      statsName = 'inbound-rtp';
      additionalData = stats_type.toMap();
    } else if (this is RtcIceCandidatePairStats) {
      var stats_type = this as RtcIceCandidatePairStats;
      statsName = 'candidate-pair';
      additionalData = stats_type.toMap();
    } else if (this is RtcTransportStats) {
      var stats_type = this as RtcTransportStats;
      statsName = 'transport';
      additionalData = stats_type.toMap();
    } else if (this is RtcRemoteInboundRtpStreamStats) {
      var stats_type = this as RtcRemoteInboundRtpStreamStats;
      statsName = 'remote-inbound-rtp';
      additionalData = stats_type.toMap();
    } else if (this is RtcRemoteOutboundRtpStreamStats) {
      var stats_type = this as RtcRemoteOutboundRtpStreamStats;
      statsName = 'remote-outbound-rtp';
      additionalData = stats_type.toMap();
    } else {
      throw 'Unreachable';
    }

    return {
      'type': statsName,
      ...additionalData,
    };
  }
}

extension RtcStatsMapConverter on RtcStats {
  Map<String, dynamic> toMap() {
    return {
      'id': this.id,
      'timestamp': this.timestampUs.toDouble() /
          1000.0, // convert microsecs in millisecs
      ...this.type.toMap(),
    };
  }
}
