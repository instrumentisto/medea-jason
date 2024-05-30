import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

extension RtcMediaSourceStatsMapConverter on RtcMediaSourceStats {
  /// Converts this [RtcMediaSourceStats] into convertable to JSON [Map].
  Map<String, dynamic> toMap() {
    String kind;
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
      'trackIdentifier': trackIdentifier,
      'kind': kind,
      ...additionalData
    };
  }
}

extension RtcIceCandidateStatsMapConverter on RtcIceCandidateStats {
  /// Converts this [RtcIceCandidateStats] into convertable to JSON [Map].
  Map<String, dynamic> toMap() {
    String type;
    if (this is RtcLocalIceCandidateStats) {
      type = 'local-candidate';
    } else if (this is RtcRemoteIceCandidateStats) {
      type = 'remote-candidate';
    } else {
      throw 'Unreachable';
    }
    return {
      'type': type,
      'transportId': transportId,
      'address': address,
      'port': port,
      'protocol': protocol.toJsonString(),
      'candidateType': candidateType.toJsonString(),
      'priority': priority,
      'url': url,
      'relayProtocol': relayProtocol.toJsonString(),
    };
  }
}

extension CandidateTypeJsonStringConverter on CandidateType {
  /// Converts this [CandidateType] into [String] suitable in the JSON reports.
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
  /// Converts this [Protocol] into [String] suitable in the JSON reports.
  String? toJsonString() {
    return switch (this) {
      Protocol.tcp => 'tcp',
      Protocol.udp => 'udp',
      null => null
    };
  }
}

extension RtcOutboundRtpStreamStatsMapConverter on RtcOutboundRtpStreamStats {
  /// Converts this [RtcOutboundRtpStreamStats] into convertable to JSON [Map].
  Map<String, dynamic> toMap() {
    var additionalData = {};
    String? mediaTypeString;
    if (mediaType is RtcOutboundRtpStreamStatsAudio) {
      var mediaType = this.mediaType as RtcOutboundRtpStreamStatsAudio;
      mediaTypeString = 'audio';
      additionalData = {
        'totalSamplesSent': mediaType.totalSamplesSent,
        'voiceActivityFlag': mediaType.voiceActivityFlag,
      };
    } else if (mediaType is RtcOutboundRtpStreamStatsVideo) {
      var mediaType = this.mediaType as RtcOutboundRtpStreamStatsVideo;
      mediaTypeString = 'video';
      additionalData = {
        'frameWidth': mediaType.frameWidth,
        'frameHeight': mediaType.frameHeight,
        'framesPerSecond': mediaType.framesPerSecond,
      };
    } else {
      throw 'Unreachable';
    }
    return {
      'trackId': trackId,
      'kind': mediaTypeString,
      'mediaType': mediaTypeString,
      'bytesSent': bytesSent,
      'packetsSent': packetsSent,
      'mediaSourceId': mediaSourceId,
      ...additionalData,
    };
  }
}

extension RtcInboundRtpStreamStatsMapConverter on RtcInboundRtpStreamStats {
  /// Converts this [RtcInboundRtpStreamStats] into convertable to JSON [Map].
  Map<String, dynamic> toMap() {
    var additionalData = {};
    String? mediaTypeString;
    if (mediaType is RtcInboundRtpStreamAudio) {
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
    } else if (mediaType is RtcInboundRtpStreamVideo) {
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
    } else {
      throw 'Unreachable';
    }
    return {
      'kind': mediaTypeString,
      'mediaType': mediaTypeString,
      'remoteId': remoteId,
      'bytesReceived': bytesReceived,
      'packetsReceived': packetsReceived,
      'totalDecodeTime': totalDecodeTime,
      'jitterBufferEmittedCount': jitterBufferEmittedCount,
      ...additionalData,
    };
  }
}

extension RtcIceCandidatePairStatsMapConverter on RtcIceCandidatePairStats {
  /// Converts this [RtcIceCandidatePairStats] into convertable to JSON [Map].
  Map<String, dynamic> toMap() {
    return {
      'state': state.toJsonString(),
      'nominated': nominated,
      'bytesSent': bytesSent,
      'bytesReceived': bytesReceived,
      'totalRoundTripTime': totalRoundTripTime,
      'currentRoundTripTime': currentRoundTripTime,
      'availableOutgoingBitrate': availableOutgoingBitrate?.toInt(),
    };
  }
}

extension RtcStatsIceCandidatePairStateJsonStringConverter
    on RtcStatsIceCandidatePairState {
  /// Converts this [RtcStatsIceCandidatePairState] into [String] suitable in the JSON reports.
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
  /// Converts this [RtcTransportStats] into convertable to JSON [Map].
  Map<String, dynamic> toMap() {
    return {
      'packetsSent': packetsSent,
      'packetsReceived': packetsReceived,
      'bytesSent': bytesSent,
      'bytesReceived': bytesReceived,
      'iceRole': iceRole.toJsonString(),
    };
  }
}

extension IceRoleJsonStringConverter on IceRole? {
  /// Converts this [IceRole] into [String] suitable in the JSON reports.
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
  /// Converts this [RtcRemoteInboundRtpStreamStats] into convertable to JSON [Map].
  Map<String, dynamic> toMap() {
    return {
      'localId': localId,
      'roundTripTime': roundTripTime,
      'fractionLost': fractionLost,
      'roundTripTimeMeasurements': roundTripTimeMeasurements,
      'jitter': jitter,
      'reportsReceived': reportsReceived,
    };
  }
}

extension RtcRemoteOutboundRtpStreamStatsMapConverter
    on RtcRemoteOutboundRtpStreamStats {
  /// Converts this [RtcRemoteOutboundRtpStreamStats] into convertable to JSON [Map].
  Map<String, dynamic> toMap() {
    return {
      'localId': localId,
      'remoteTimestamp': remoteTimestamp,
      'reportsSent': reportsSent,
    };
  }
}

extension RtcStatsTypeMapConverter on RtcStatsType {
  /// Converts this [RtcStatsType] into convertable to JSON [Map].
  Map<String, dynamic> toMap() {
    String? statsName;
    var additionalData = {};

    if (this is RtcMediaSourceStats) {
      var statsType = this as RtcMediaSourceStats;
      statsName = 'media-source';
      additionalData = statsType.toMap();
    } else if (this is RtcIceCandidateStats) {
      var statsType = this as RtcIceCandidateStats;
      additionalData = statsType.toMap();
    } else if (this is RtcOutboundRtpStreamStats) {
      var statsType = this as RtcOutboundRtpStreamStats;
      statsName = 'outbound-rtp';
      additionalData = statsType.toMap();
    } else if (this is RtcInboundRtpStreamStats) {
      var statsType = this as RtcInboundRtpStreamStats;
      statsName = 'inbound-rtp';
      additionalData = statsType.toMap();
    } else if (this is RtcIceCandidatePairStats) {
      var statsType = this as RtcIceCandidatePairStats;
      statsName = 'candidate-pair';
      additionalData = statsType.toMap();
    } else if (this is RtcTransportStats) {
      var statsType = this as RtcTransportStats;
      statsName = 'transport';
      additionalData = statsType.toMap();
    } else if (this is RtcRemoteInboundRtpStreamStats) {
      var statsType = this as RtcRemoteInboundRtpStreamStats;
      statsName = 'remote-inbound-rtp';
      additionalData = statsType.toMap();
    } else if (this is RtcRemoteOutboundRtpStreamStats) {
      var statsType = this as RtcRemoteOutboundRtpStreamStats;
      statsName = 'remote-outbound-rtp';
      additionalData = statsType.toMap();
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
  /// Converts this [RtcStats] into convertable to JSON [Map].
  Map<String, dynamic> toMap() {
    return {
      'id': id,
      'timestamp':
          timestampUs.toDouble() / 1000.0, // convert microsecs to millisecs
      ...type.toMap(),
    };
  }
}
