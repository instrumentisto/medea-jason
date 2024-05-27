import 'dart:convert';
import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/native_string.dart';
import '../ffi/foreign_value.dart';
import 'peer_connection.g.dart' as bridge;

/// Registers [PeerConnection] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    setRemoteDescription: Pointer.fromFunction(_setRemoteDescription),
    setLocalDescription: Pointer.fromFunction(_setLocalDescription),
    addIceCandidate: Pointer.fromFunction(_addIceCandidate),
    iceConnectionState: Pointer.fromFunction(_iceConnectionState, 0),
    connectionState: Pointer.fromFunction(_connectionState),
    restartIce: Pointer.fromFunction(_restartIce),
    rollback: Pointer.fromFunction(_rollback),
    onTrack: Pointer.fromFunction(_onTrack),
    onIceCandidate: Pointer.fromFunction(_onIceCandidate),
    onIceCandidateError: Pointer.fromFunction(_onIceCandidateError),
    onIceConnectionStateChange:
        Pointer.fromFunction(_onIceConnectionStateChange),
    newPeer: Pointer.fromFunction(_newPeer),
    addTransceiver: Pointer.fromFunction(_addTransceiver),
    createOffer: Pointer.fromFunction(_createOffer),
    createAnswer: Pointer.fromFunction(_createAnswer),
    getTransceiverByMid: Pointer.fromFunction(_getTransceiverByMid),
    onConnectionStateChange: Pointer.fromFunction(_onConnectionStateChange),
    close: Pointer.fromFunction(_close),
  );
}

/// Adds an [RtpTransceiverInit] to the provided [PeerConnection].
///
/// Returns [Future] which will be resolved into created [RtpTransceiver].
Object _addTransceiver(Object peer, int kind, Object init) {
  peer as PeerConnection;
  init as RtpTransceiverInit;
  return () => peer.addTransceiver(MediaKind.values[kind], init);
}

/// Returns a newly created [PeerConnection] with the provided `iceServers`
/// [List].
Object _newPeer(Object iceServers, bool isForceRelayed) {
  var servers = iceServers as List<dynamic>;
  var iceType = isForceRelayed ? IceTransportType.relay : IceTransportType.all;
  return () => PeerConnection.create(
      iceType, servers.map((e) => e as IceServer).toList());
}

Map<String, dynamic> convertRtcMediaSourceStatsToMap(
    RtcMediaSourceStats stats) {
  var type;
  var additionalData = {};
  if (stats is RtcAudioSourceStats) {
    stats as RtcAudioSourceStats;
    type = 'rtc-audio-source-stats';
    additionalData = {
      'audioLevel': stats.audioLevel,
      'totalAudioEnergy': stats.totalAudioEnergy,
      'totalSamplesDuration': stats.totalSamplesDuration,
      'echoReturnLoss': stats.echoReturnLoss,
      'echoReturnLossEnhancement': stats.echoReturnLossEnhancement,
    };
  } else if (stats is RtcVideoSourceStats) {
    type = 'rtc-video-source-stats';
    additionalData = {
      'width': stats.width,
      'height': stats.height,
      'frames': stats.frames,
      'framesPerSecond': stats.framesPerSecond,
    };
  } else {
    throw 'Unreachable';
  }
  return {'trackIdentifier': stats.trackIdentifier, ...additionalData};
}

Map<String, dynamic> convertRtcIceCandidateStatsToMap(
    RtcIceCandidateStats stats) {
  var kind = "";
  if (stats is RtcLocalIceCandidateStats) {
    kind = "local";
  } else if (stats is RtcRemoteIceCandidateStats) {
    kind = "remote";
  } else {
    throw 'Unreachable';
  }
  return {
    // TODO(fix): how this field really called?
    'kind': kind,
    'transportId': stats.transportId,
    'address': stats.address,
    'port': stats.port,
    'protocol': convertProtocolToString(stats.protocol),
    'candidateType': convertCandidateTypeToString(stats.candidateType),
    'priority': stats.priority,
    'url': stats.url,
    'relayProtocol': convertProtocolToString(stats.relayProtocol),
  };
}

String convertCandidateTypeToString(CandidateType candidateType) {
  return switch (candidateType) {
    CandidateType.host => 'host',
    CandidateType.srflx => 'srflx',
    CandidateType.prflx => 'prflx',
    CandidateType.relay => 'relay',
  };
}

String? convertProtocolToString(Protocol? protocol) {
  if (protocol == null) {
    return null;
  } else {
    return switch (protocol) {
      Protocol.tcp => 'tcp',
      Protocol.udp => 'udp',
    };
  }
}

Map<String, dynamic> convertRtcOutboundRtpStreamStatsToMap(
    RtcOutboundRtpStreamStats stats) {
  var additionalData = {};
  var mediaTypeString;
  if (stats.mediaType is RtcOutboundRtpStreamStatsAudio) {
    var mediaType = stats.mediaType as RtcOutboundRtpStreamStatsAudio;
    mediaTypeString = 'audio';
    additionalData = {
      'totalSamplesSent': mediaType.totalSamplesSent,
      'voiceActivityFlag': mediaType.voiceActivityFlag,
    };
  } else if (stats.mediaType is RtcOutboundRtpStreamStatsVideo) {
    var mediaType = stats.mediaType as RtcOutboundRtpStreamStatsVideo;
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
    'trackId': stats.trackId,
    'mediaType': mediaTypeString,
    'bytesSent': stats.bytesSent,
    'packetsSent': stats.packetsSent,
    'mediaSourceId': stats.mediaSourceId,
    ...additionalData,
  };
}

Map<String, dynamic> convertRtcInboundRtpStreamStatsToMap(
    RtcInboundRtpStreamStats stats) {
  var additionalData = {};
  var mediaTypeString = "";
  if (stats.mediaType is RtcInboundRtpStreamAudio) {
    var mediaType = stats.mediaType as RtcInboundRtpStreamAudio;
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
  } else if (stats.mediaType is RtcInboundRtpStreamVideo) {
    mediaTypeString = 'video';
    var mediaType = stats.mediaType as RtcInboundRtpStreamVideo;
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
    'mediaType': mediaTypeString,
    'remoteId': stats.remoteId,
    'bytesReceived': stats.bytesReceived,
    'packetsReceived': stats.packetsReceived,
    'totalDecodeTime': stats.totalDecodeTime,
    'jitterBufferEmittedCount': stats.jitterBufferEmittedCount,
    ...additionalData,
  };
}

Map<String, dynamic> convertRtcIceCandidatePairStatsToMap(
    RtcIceCandidatePairStats stats) {
  return {
    'state': convertRtcStatsIceCandidatePairStateToString(stats.state),
    'nominated': stats.nominated,
    'bytesSent': stats.bytesSent,
    'bytesReceived': stats.bytesReceived,
    'totalRoundTripTime': stats.totalRoundTripTime,
    'currentRoundTripTime': stats.currentRoundTripTime,
    'availableOutgoingBitrate': stats.availableOutgoingBitrate,
  };
}

String convertRtcStatsIceCandidatePairStateToString(
    RtcStatsIceCandidatePairState state) {
  return switch (state) {
    RtcStatsIceCandidatePairState.frozen => 'frozen',
    RtcStatsIceCandidatePairState.waiting => 'waiting',
    RtcStatsIceCandidatePairState.inProgress => 'inProgress',
    RtcStatsIceCandidatePairState.failed => 'failed',
    RtcStatsIceCandidatePairState.succeeded => 'succeeded',
  };
}

Map<String, dynamic> convertRtcTransportStatsToMap(RtcTransportStats stats) {
  return {
    'packetsSent': stats.packetsSent,
    'packetsReceived': stats.packetsReceived,
    'bytesSent': stats.bytesSent,
    'bytesReceived': stats.bytesReceived,
    'iceRole': convertIceRoleToString(stats.iceRole),
  };
}

String? convertIceRoleToString(IceRole role) {
  if (rol == null) {
    return null;
  } else {
    return switch (role) {
      IceRole.unknown => 'unknown',
      IceRole.controlling => 'controlling',
      IceRole.controlled => 'controlled',
    };
  }
}

Map<String, dynamic> convertRtcRemoteInboundRtpStreamStatsToMap(
    RtcRemoteInboundRtpStreamStats stats) {
  return {
    'localId': stats.localId,
    'roundTripTime': stats.roundTripTime,
    'fractionLost': stats.fractionLost,
    'roundTripTimeMeasurements': stats.roundTripTimeMeasurements,
    'jitter': stats.jitter,
    'reportsReceived': stats.reportsReceived,
  };
}

Map<String, dynamic> convertRtcRemoteOutboundRtpStreamStatsToMap(
    RtcRemoteOutboundRtpStreamStats stats) {
  return {
    'localId': stats.localId,
    'remoteTimestamp': stats.remoteTimestamp,
    'reportsSent': stats.reportsSent,
  };
}

Map<String, dynamic> convertStatsTypeToMap(RtcStatsType stats_type) {
  if (stats_type is RtcMediaSourceStats) {
    stats_type as RtcMediaSourceStats;
    print(convertRtcMediaSourceStatsToMap(stats_type));
  } else if (stats_type is RtcIceCandidateStats) {
    stats_type as RtcIceCandidateStats;
    print(convertRtcIceCandidateStatsToMap(stats_type));
  } else if (stats_type is RtcOutboundRtpStreamStats) {
    stats_type as RtcOutboundRtpStreamStats;
    print(convertRtcOutboundRtpStreamStatsToMap(stats_type));
  } else if (stats_type is RtcInboundRtpStreamStats) {
    stats_type as RtcInboundRtpStreamStats;
    print(convertRtcInboundRtpStreamStatsToMap(stats_type));
  } else if (stats_type is RtcIceCandidatePairStats) {
    stats_type as RtcIceCandidatePairStats;
    print(convertRtcIceCandidatePairStatsToMap(stats_type));
  } else if (stats_type is RtcTransportStats) {
    stats_type as RtcTransportStats;
    print(jsonEncode(convertRtcTransportStatsToMap(stats_type)));
  } else if (stats_type is RtcRemoteInboundRtpStreamStats) {
    stats_type as RtcRemoteInboundRtpStreamStats;
    print(convertRtcRemoteInboundRtpStreamStatsToMap(stats_type));
  } else if (stats_type is RtcRemoteOutboundRtpStreamStats) {
    stats_type as RtcRemoteOutboundRtpStreamStats;
    print(convertRtcRemoteOutboundRtpStreamStatsToMap(stats_type));
  }
  return {};
}

Map<String, dynamic> convertStatsToMap(RtcStats stats) {
  convertStatsTypeToMap(stats.type);
  return {};
}

/// Sets the provided [f] to the [PeerConnection.onTrack] callback.
void _onTrack(Object conn, Object f) {
  conn as PeerConnection;
  f as Function;
  () async {
    while (true) {
      var stats = await conn.getStats();
      for (var stat in stats) {
        convertStatsToMap(stat);
        // print(jsonEncode({
        //   'id': stat.id,
        //   'timestampUs': stat.timestampUs,
        // }));
      }
      await Future.delayed(Duration(seconds: 2));
    }
  }();
  conn.onTrack((track, transceiver) {
    f(track, transceiver);
  });
}

/// Sets the provided [f] to the [PeerConnection.onIceCandidate] callback.
void _onIceCandidate(Object conn, Object f) {
  conn as PeerConnection;
  f as Function;
  conn.onIceCandidate((e) {
    f(e);
  });
}

/// Sets the provided [f] to the [PeerConnection.onIceCandidateError] callback.
void _onIceCandidateError(Object conn, Object f) {
  conn as PeerConnection;
  f as Function;
  conn.onIceCandidateError((e) {
    f(e);
  });
}

/// Sets the provided [f] to the [PeerConnection.onIceConnectionStateChange]
/// callback.
void _onIceConnectionStateChange(Object conn, Object f) {
  conn as PeerConnection;
  f as Function;
  conn.onIceConnectionStateChange((e) {
    f(e.index);
  });
}

/// Sets the provided [f] to the [PeerConnection.onConnectionStateChange]
/// callback.
void _onConnectionStateChange(Object conn, Object f) {
  conn as PeerConnection;
  f as Function;
  conn.onConnectionStateChange((e) {
    f(e.index);
  });
}

/// Lookups an [RtpTransceiver] in the provided [PeerConnection] by the provided
/// [mid].
Object _getTransceiverByMid(Object peer, Pointer<Utf8> mid) {
  peer as PeerConnection;
  return () => peer.getTransceivers().then((transceivers) {
        var mMid = mid.nativeStringToDartString();
        RtpTransceiver? result;
        for (var transceiver in transceivers) {
          if (transceiver.mid == mMid) {
            result = transceiver;
          } else {
            transceiver.dispose();
          }
        }
        return result;
      });
}

/// Sets a remote SDP offer in the provided [PeerConnection].
Object _setRemoteDescription(
    Object conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  conn as PeerConnection;
  SessionDescriptionType sdpType;
  if (type.nativeStringToDartString() == 'offer') {
    sdpType = SessionDescriptionType.offer;
  } else {
    sdpType = SessionDescriptionType.answer;
  }
  var desc = SessionDescription(sdpType, sdp.nativeStringToDartString());
  return () => conn.setRemoteDescription(desc);
}

/// Sets a local SDP offer in the provided [PeerConnection].
Object _setLocalDescription(
    Object conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  conn as PeerConnection;
  SessionDescriptionType sdpType;
  if (type.nativeStringToDartString() == 'offer') {
    sdpType = SessionDescriptionType.offer;
  } else {
    sdpType = SessionDescriptionType.answer;
  }
  return () => conn.setLocalDescription(
      SessionDescription(sdpType, sdp.nativeStringToDartString()));
}

/// Creates a new SDP offer for the provided [PeerConnection].
Object _createOffer(Object conn) {
  conn as PeerConnection;
  return () => conn.createOffer().then((val) => val.description);
}

/// Creates a new SDP answer for the provided [PeerConnection].
Object _createAnswer(Object conn) {
  conn as PeerConnection;
  return () => conn.createAnswer().then((val) => val.description);
}

/// Marks the given [PeerConnection] to create descriptions that will restart
/// ICE on the next [PeerConnection.createOffer] call.
void _restartIce(Object conn) {
  conn as PeerConnection;
  conn.restartIce();
}

/// Adds the specified [IceCandidate] to the provided [PeerConnection].
Object _addIceCandidate(Object conn, Object candidate) {
  conn as PeerConnection;
  candidate as IceCandidate;
  return () => conn.addIceCandidate(candidate);
}

/// Returns the current [PeerConnection.connectionState] of the provided
/// [PeerConnection].
Pointer _connectionState(Object conn) {
  conn as PeerConnection;
  return ForeignValue.fromInt(conn.connectionState().index).intoRustOwned();
}

/// Returns the current [PeerConnection.iceConnectionState] of the provided
/// [PeerConnection].
int _iceConnectionState(Object conn) {
  conn as PeerConnection;
  return conn.iceConnectionState().index;
}

/// Rollbacks the local SDP offer of the provided [PeerConnection].
Object _rollback(Object conn) {
  conn as PeerConnection;
  return () => conn.setLocalDescription(
      SessionDescription(SessionDescriptionType.rollback, ''));
}

/// Returns all the [RtpTransceiver]s of the provided [PeerConnection].
Object getTransceivers(Object conn) {
  conn as PeerConnection;
  return () => conn.getTransceivers();
}

/// Closes the provided [PeerConnection].
void _close(Object conn) {
  conn as PeerConnection;
  conn.close();
}
