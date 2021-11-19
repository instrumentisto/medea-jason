import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'package:ffi/ffi.dart';

import 'peer_connection.g.dart' as bridge;
import '../ffi/foreign_value.dart';

/// Registers [RTCPeerConnection] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    setRemoteDescription: _setRemoteDescription,
    setLocalDescription: _setLocalDescription,
    addIceCandidate: _addIceCandidate,
    iceConnectionState: _iceConnectionState,
    connectionState: _connectionState,
    restartIce: _restartIce,
    rollback: _rollback,
    onTrack: _onTrack,
    onIceCandidate: _onIceCandidate,
    onIceConnectionStateChange: _onIceConnectionStateChange,
    newPeer: _newPeer,
    addTransceiver: _addTransceiver,
    createOffer: _createOffer,
    createAnswer: _createAnswer,
    getTransceiverByMid: _getTransceiverByMid,
    onConnectionStateChange: _onConnectionStateChange,
  );
}

/// Adds [RTCRtpTransceiver] to the provided [RTCPeerConnection].
///
/// Returns [Future] which will be resolved into created [RTCRtpTransceiver].
Object _addTransceiver(Object peer, int kind, int direction) {
  peer as RTCPeerConnection;
  return peer.addTransceiver(
    kind: RTCRtpMediaType.values[kind],
    init: RTCRtpTransceiverInit(direction: TransceiverDirection.SendRecv),
  );
}

/// Returns newly created [RTCPeerConnection] with a provided `iceServers`
/// [List].
Object _newPeer(Object iceServers) {
  return createPeerConnection(
      {'iceServers': iceServers, 'sdpSemantics': 'unified-plan'});
}

/// Adds subscription on [RTCPeerConnection.onTrack] to the provided
/// [RTCPeerConnection].
void _onTrack(Object conn, Object f) {
  conn as RTCPeerConnection;
  f as Function;
  conn.onTrack = (e) {
    f(e.track, e.transceiver);
  };
}

/// Add subscription on [RTCPeerConnection.onIceCandidate] to the provided
/// [RTCPeerConnection].
void _onIceCandidate(Object conn, Object f) {
  conn as RTCPeerConnection;
  f as Function;
  conn.onIceCandidate = (e) {
    f(e);
  };
}

/// Adds subscription on [RTCPeerConnection.onIceConnectionState] to the
/// provided [RTCPeerConnection].
void _onIceConnectionStateChange(Object conn, Object f) {
  conn as RTCPeerConnection;
  f as Function;
  conn.onIceConnectionState = (e) {
    f(e.index);
  };
}

/// Adds subscription on [RTCPeerConnection.onConnectionState] to the
/// provided [RTCPeerConnection].
void _onConnectionStateChange(Object conn, Object f) {
  conn as RTCPeerConnection;
  f as Function;
  conn.onConnectionState = (e) {
    f(e.index);
  };
}

/// Lookups [RTCRtpTransceiver] in the provided [RTCPeerConnection] by the
/// provided [String].
Object _getTransceiverByMid(Object peer, Pointer<Utf8> mid) {
  peer as RTCPeerConnection;
  return peer.getTransceivers().then((transceivers) {
    var mMid = mid.toDartString();
    for (var transceiver in transceivers) {
      if (transceiver.mid == mMid) {
        return transceiver;
      }
    }
  });
}

/// Sets remote SDP offer in the provided [RTCPeerConnection].
Object _setRemoteDescription(
    Object conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  conn as RTCPeerConnection;
  var desc = RTCSessionDescription(sdp.toDartString(), type.toDartString());
  return conn.setRemoteDescription(desc);
}

/// Sets local SDP offer in the provided [RTCPeerConnection].
Object _setLocalDescription(
    Object conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  conn as RTCPeerConnection;
  return conn.setLocalDescription(
      RTCSessionDescription(sdp.toDartString(), type.toDartString()));
}

/// Creates new SDP offer for the provided [RTCPeerConnection].
Object _createOffer(Object conn) {
  conn as RTCPeerConnection;
  return conn.createOffer({}).then((val) => val.sdp);
}

/// Creates new SDP answer for the provided [RTCPeerConnection].
Object _createAnswer(Object conn) {
  conn as RTCPeerConnection;
  return conn.createAnswer({}).then((val) => val.sdp);
}

/// Restarts ICE on the provided [RTCPeerConnection].
void _restartIce(Object conn) {
  conn as RTCPeerConnection;
  throw UnimplementedError('PeerConnection.restartIce');
}

/// Adds provided [RTCIceCandidate] to the provided [RTCPeerConnection].
Object _addIceCandidate(Object conn, Object candidate) {
  conn as RTCPeerConnection;
  candidate as RTCIceCandidate;
  return conn.addCandidate(candidate);
}

/// Returns current [RTCPeerConnection.connectionState] of the provided
/// [RTCPeerConnection].
Pointer _connectionState(Object conn) {
  conn as RTCPeerConnection;
  if (conn.connectionState != null) {
    return ForeignValue.fromInt(conn.connectionState!.index).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns current [RTCPeerConnection.iceConnectionState] of the provided
/// [RTCPeerConnection].
int _iceConnectionState(Object conn) {
  conn as RTCPeerConnection;
  if (conn.iceConnectionState != null) {
    return conn.iceConnectionState!.index;
  } else {
    return RTCIceConnectionState.RTCIceConnectionStateNew.index;
  }
}

/// Rollbacks local SDP offer of the provided [RTCPeerConnection].
Object _rollback(Object conn) {
  conn as RTCPeerConnection;
  return conn.setLocalDescription(RTCSessionDescription(null, 'rollback'));
}

/// Returns all [RTCRtpTransceiver]s of the provided [RTCPeerConnection].
Object getTransceivers(Object conn) {
  conn as RTCPeerConnection;
  return conn.getTransceivers();
}
