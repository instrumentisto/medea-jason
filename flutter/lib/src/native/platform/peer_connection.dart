import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/native_string.dart';
import '../ffi/foreign_value.dart';
import 'peer_connection.g.dart' as bridge;
import 'stats_ffi.dart';

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
    onIceConnectionStateChange:
        Pointer.fromFunction(_onIceConnectionStateChange),
    newPeer: Pointer.fromFunction(_newPeer),
    addTransceiver: Pointer.fromFunction(_addTransceiver),
    createOffer: Pointer.fromFunction(_createOffer),
    createAnswer: Pointer.fromFunction(_createAnswer),
    getTransceiverByMid: Pointer.fromFunction(_getTransceiverByMid),
    onConnectionStateChange: Pointer.fromFunction(_onConnectionStateChange),
    close: Pointer.fromFunction(_close),
    getStats: Pointer.fromFunction(_getStats),
    toOwnedStats: Pointer.fromFunction(_toOwnedStats),
  );
}

/// Adds an [RtpTransceiver] to the provided [PeerConnection].
///
/// Returns [Future] which will be resolved into created [RtpTransceiver].
Object _addTransceiver(PeerConnection peer, int kind, int direction) {
  return () => peer.addTransceiver(MediaKind.values[kind],
      RtpTransceiverInit(TransceiverDirection.values[direction]));
}

/// Returns a newly created [PeerConnection] with the provided `iceServers`
/// [List].
Object _newPeer(Object iceServers, bool isForceRelayed) {
  var servers = iceServers as List<dynamic>;
  var iceType = isForceRelayed ? IceTransportType.relay : IceTransportType.all;
  return () => PeerConnection.create(
      iceType, servers.map((e) => e as IceServer).toList());
}

/// Sets the provided [f] to the [PeerConnection.onTrack] callback.
void _onTrack(PeerConnection conn, Function f) {
  conn.onTrack((track, transceiver) {
    f(track, transceiver);
  });
}

/// Sets the provided [f] to the [PeerConnection.onIceCandidate] callback.
void _onIceCandidate(PeerConnection conn, Function f) {
  conn.onIceCandidate((e) {
    f(e);
  });
}

/// Sets the provided [f] to the [PeerConnection.onIceConnectionStateChange]
/// callback.
void _onIceConnectionStateChange(PeerConnection conn, Function f) {
  conn.onIceConnectionStateChange((e) {
    f(e.index);
  });
}

/// Sets the provided [f] to the [PeerConnection.onConnectionStateChange]
/// callback.
void _onConnectionStateChange(PeerConnection conn, Function f) {
  conn.onConnectionStateChange((e) {
    f(e.index);
  });
}

/// Lookups an [RtpTransceiver] in the provided [PeerConnection] by the provided
/// [mid].
Object _getTransceiverByMid(PeerConnection peer, Pointer<Utf8> mid) {
  return () => peer.getTransceivers().then((transceivers) {
        var mMid = mid.nativeStringToDartString();
        for (var transceiver in transceivers) {
          if (transceiver.mid == mMid) {
            return transceiver;
          }
        }
      });
}

/// Sets a remote SDP offer in the provided [PeerConnection].
Object _setRemoteDescription(
    PeerConnection conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  var sdpType;
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
    PeerConnection conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  var sdpType;
  if (type.nativeStringToDartString() == 'offer') {
    sdpType = SessionDescriptionType.offer;
  } else {
    sdpType = SessionDescriptionType.answer;
  }
  return () => conn.setLocalDescription(
      SessionDescription(sdpType, sdp.nativeStringToDartString()));
}

/// Creates a new SDP offer for the provided [PeerConnection].
Object _createOffer(PeerConnection conn) {
  return () => conn.createOffer().then((val) => val.description);
}

/// Creates a new SDP answer for the provided [PeerConnection].
Object _createAnswer(PeerConnection conn) {
  return () => conn.createAnswer().then((val) => val.description);
}

/// Marks the given [PeerConnection] to create descriptions that will restart
/// ICE on the next [PeerConnection.createOffer] call.
void _restartIce(PeerConnection conn) {
  conn.restartIce();
}

/// Adds the specified [IceCandidate] to the provided [PeerConnection].
Object _addIceCandidate(PeerConnection conn, IceCandidate candidate) {
  return () => conn.addIceCandidate(candidate);
}

/// Returns the current [PeerConnection.connectionState] of the provided
/// [PeerConnection].
Pointer _connectionState(PeerConnection conn) {
  return ForeignValue.fromInt(conn.connectionState().index).intoRustOwned();
}

/// Returns the current [PeerConnection.iceConnectionState] of the provided
/// [PeerConnection].
int _iceConnectionState(PeerConnection conn) {
  return conn.iceConnectionState().index;
}

/// Rollbacks the local SDP offer of the provided [PeerConnection].
Object _rollback(PeerConnection conn) {
  return () => conn.setLocalDescription(
      SessionDescription(SessionDescriptionType.rollback, ''));
}

/// Returns all the [RtpTransceiver]s of the provided [PeerConnection].
Object getTransceivers(PeerConnection conn) {
  return () => conn.getTransceivers();
}

/// Returns all the [RTCStats] of the provided [PeerConnection].
Object _getStats(PeerConnection conn) {
  return () => conn.getStats();
}

/// Transfers [RTCFfiStats] ownership to Rust.
///
/// Frees Dart side [RTCFfiStats].
Pointer<NativeType> _toOwnedStats(RTCStats stats) {
  var ffi = RTCFfiStats.fromDartStats(stats);
  return ffi.intoRustOwned();
}

/// Closes the provided [PeerConnection].
void _close(PeerConnection conn) {
  conn.close();
}
