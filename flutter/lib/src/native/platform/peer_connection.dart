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

extension RtcMediaSourceStatsMapConverter on RtcMediaSourceStats {
  Map<String, dynamic> toMap() {
    var type;
    var additionalData = {};
    if (this is RtcAudioSourceStats) {
      var stat = this as RtcAudioSourceStats;
      type = 'rtc-audio-source-stats';
      additionalData = {
        'audioLevel': stat.audioLevel,
        'totalAudioEnergy': stat.totalAudioEnergy,
        'totalSamplesDuration': stat.totalSamplesDuration,
        'echoReturnLoss': stat.echoReturnLoss,
        'echoReturnLossEnhancement': stat.echoReturnLossEnhancement,
      };
    } else if (this is RtcVideoSourceStats) {
      var stat = this as RtcVideoSourceStats;
      type = 'rtc-video-source-stats';
      additionalData = {
        'width': stat.width,
        'height': stat.height,
        'frames': stat.frames,
        'framesPerSecond': stat.framesPerSecond,
      };
    } else {
      throw 'Unreachable';
    }
    return {'trackIdentifier': this.trackIdentifier, ...additionalData};
  }
}

extension RtcIceCandidateStatsMapConverter on RtcIceCandidateStats {
  Map<String, dynamic> toMap() {
    var kind = "";
    if (this is RtcLocalIceCandidateStats) {
      kind = "local";
    } else if (this is RtcRemoteIceCandidateStats) {
      kind = "remote";
    } else {
      throw 'Unreachable';
    }
    return {
      // TODO(fix): how this field really called?
      'kind': kind,
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
    var mediaTypeString = "";
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
      'availableOutgoingBitrate': this.availableOutgoingBitrate,
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
    if (this is RtcMediaSourceStats) {
      var stats_type = this as RtcMediaSourceStats;
      print(jsonEncode(stats_type.toMap()));
    } else if (this is RtcIceCandidateStats) {
      var stats_type = this as RtcIceCandidateStats;
      print(jsonEncode(stats_type.toMap()));
    } else if (this is RtcOutboundRtpStreamStats) {
      var stats_type = this as RtcOutboundRtpStreamStats;
      print(jsonEncode(stats_type.toMap()));
    } else if (this is RtcInboundRtpStreamStats) {
      var stats_type = this as RtcInboundRtpStreamStats;
      print(jsonEncode(stats_type.toMap()));
    } else if (this is RtcIceCandidatePairStats) {
      var stats_type = this as RtcIceCandidatePairStats;
      print(jsonEncode(stats_type.toMap()));
    } else if (this is RtcTransportStats) {
      var stats_type = this as RtcTransportStats;
      print(jsonEncode(stats_type.toMap()));
    } else if (this is RtcRemoteInboundRtpStreamStats) {
      var stats_type = this as RtcRemoteInboundRtpStreamStats;
      print(jsonEncode(stats_type.toMap()));
    } else if (this is RtcRemoteOutboundRtpStreamStats) {
      var stats_type = this as RtcRemoteOutboundRtpStreamStats;
      print(jsonEncode(stats_type.toMap()));
    }
    return {};
  }
}

extension RtcStatsMapConverter on RtcStats {
  Map<String, dynamic> toMap() {
    this.type.toMap();
    return {};
  }
}

/// Sets the provided [f] to the [PeerConnection.onTrack] callback.
void _onTrack(Object conn, Object f) {
  conn as PeerConnection;
  f as Function;
  () async {
    while (true) {
      var stats = await conn.getStats();
      for (var stat in stats) {
        stat.toMap();
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
