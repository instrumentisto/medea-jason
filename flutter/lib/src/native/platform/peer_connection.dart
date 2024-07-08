import 'dart:convert';
import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/native_string.dart';
import '../ffi/foreign_value.dart';
import 'peer_connection.g.dart' as bridge;
import 'rtc_stats.dart';

/// Registers [PeerConnection] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    getStats: _getStats,
    setRemoteDescription: _setRemoteDescription,
    setLocalDescription: _setLocalDescription,
    addIceCandidate: _addIceCandidate,
    iceConnectionState: _iceConnectionState,
    connectionState: _connectionState,
    restartIce: _restartIce,
    rollback: _rollback,
    onTrack: _onTrack,
    onIceCandidate: _onIceCandidate,
    onIceCandidateError: _onIceCandidateError,
    onIceConnectionStateChange: _onIceConnectionStateChange,
    newPeer: _newPeer,
    addTransceiver: _addTransceiver,
    createOffer: _createOffer,
    createAnswer: _createAnswer,
    getTransceiverByMid: _getTransceiverByMid,
    onConnectionStateChange: _onConnectionStateChange,
    close: _close,
  );
}

/// Adds an [RtpTransceiverInit] to the provided [PeerConnection].
///
/// Returns [Future] which will be resolved into created [RtpTransceiver].
Future<RtpTransceiver> Function() _addTransceiver(
    Object peer, int kind, Object init) {
  peer as PeerConnection;
  init as RtpTransceiverInit;
  return () => peer.addTransceiver(MediaKind.values[kind], init);
}

/// Returns a newly created [PeerConnection] with the provided `iceServers`
/// [List].
Future<PeerConnection> Function() _newPeer(
    Object iceServers, bool isForceRelayed) {
  var servers = iceServers as List<dynamic>;
  var iceType = isForceRelayed ? IceTransportType.relay : IceTransportType.all;
  return () => PeerConnection.create(
      iceType, servers.map((e) => e as IceServer).toList());
}

/// Sets the provided [f] to the [PeerConnection.onTrack] callback.
void _onTrack(Object conn, Object f) {
  conn as PeerConnection;
  f as Function;
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

/// Returns JSON encoded [Array] of [RtcStats] from the provided
/// [PeerConnection].
Future<String> Function() _getStats(Object conn) {
  conn as PeerConnection;
  return () async {
    var stats = await conn.getStats();
    var statsToEncode = stats.map((stat) => stat.toMap()).toList();
    return jsonEncode(statsToEncode);
  };
}

/// Lookups an [RtpTransceiver] in the provided [PeerConnection] by the provided
/// [mid].
Future<RtpTransceiver?> Function() _getTransceiverByMid(
    Object peer, Pointer<Utf8> mid) {
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
Future<void> Function() _setRemoteDescription(
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
Future<void> Function() _setLocalDescription(
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
Future<String> Function() _createOffer(Object conn) {
  conn as PeerConnection;
  return () => conn.createOffer().then((val) => val.description);
}

/// Creates a new SDP answer for the provided [PeerConnection].
Future<String> Function() _createAnswer(Object conn) {
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
Future<void> Function() _addIceCandidate(Object conn, Object candidate) {
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
Future<void> Function() _rollback(Object conn) {
  conn as PeerConnection;
  return () => conn.setLocalDescription(
      SessionDescription(SessionDescriptionType.rollback, ''));
}

/// Returns all the [RtpTransceiver]s of the provided [PeerConnection].
Future<List<RtpTransceiver>> Function() getTransceivers(Object conn) {
  conn as PeerConnection;
  return () => conn.getTransceivers();
}

/// Closes the provided [PeerConnection].
void _close(Object conn) {
  conn as PeerConnection;
  conn.close();
}
