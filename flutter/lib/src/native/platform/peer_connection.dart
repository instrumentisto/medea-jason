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
