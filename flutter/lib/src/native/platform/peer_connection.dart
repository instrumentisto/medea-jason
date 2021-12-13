import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'package:ffi/ffi.dart';

import 'peer_connection.g.dart' as bridge;
import '../ffi/foreign_value.dart';

/// Registers [RTCPeerConnection] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  // TODO(evdokimovs): implement dispose function
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
  );
}

/// Adds [RTCRtpTransceiver] to the provided [RTCPeerConnection].
///
/// Returns [Future] which will be resolved into created [RTCRtpTransceiver].
Object _addTransceiver(RTCPeerConnection peer, int kind, int direction) {
  return () => peer.addTransceiver(
    kind: RTCRtpMediaType.values[kind],
    init: RTCRtpTransceiverInit(direction: TransceiverDirection.SendRecv),
  );
}

/// Returns newly created [RTCPeerConnection] with a provided `iceServers`
/// [List].
Object _newPeer(Object iceServers) {
  return () => createPeerConnection(
      {'iceServers': iceServers, 'sdpSemantics': 'unified-plan'});
}

/// Adds subscription on [RTCPeerConnection.onTrack] to the provided
/// [RTCPeerConnection].
void _onTrack(RTCPeerConnection conn, Function f) {
  conn.onTrack = (e) {
    f(e.track, e.transceiver);
  };
}

/// Add subscription on [RTCPeerConnection.onIceCandidate] to the provided
/// [RTCPeerConnection].
void _onIceCandidate(RTCPeerConnection conn, Function f) {
  conn.onIceCandidate = (e) {
    f(e);
  };
}

/// Adds subscription on [RTCPeerConnection.onIceConnectionState] to the
/// provided [RTCPeerConnection].
void _onIceConnectionStateChange(RTCPeerConnection conn, Function f) {
  conn.onIceConnectionState = (e) {
    f(e.index);
  };
}

/// Adds subscription on [RTCPeerConnection.onConnectionState] to the
/// provided [RTCPeerConnection].
void _onConnectionStateChange(RTCPeerConnection conn, Function f) {
  conn.onConnectionState = (e) {
    f(e.index);
  };
}

void dispose(RTCPeerConnection conn) {
  conn.dispose();
}

/// Lookups [RTCRtpTransceiver] in the provided [RTCPeerConnection] by the
/// provided [String].
Object _getTransceiverByMid(RTCPeerConnection peer, Pointer<Utf8> mid) {
  return () => peer.getTransceivers().then((transceivers) {
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
    RTCPeerConnection conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  var desc = RTCSessionDescription(sdp.toDartString(), type.toDartString());
  return () => conn.setRemoteDescription(desc);
}

/// Sets local SDP offer in the provided [RTCPeerConnection].
Object _setLocalDescription(
    RTCPeerConnection conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  return () => conn.setLocalDescription(
      RTCSessionDescription(sdp.toDartString(), type.toDartString()));
}

/// Creates new SDP offer for the provided [RTCPeerConnection].
Object _createOffer(RTCPeerConnection conn) {
  return () => conn.createOffer({}).then((val) => val.sdp);
}

/// Creates new SDP answer for the provided [RTCPeerConnection].
Object _createAnswer(RTCPeerConnection conn) {
  return () => conn.createAnswer({}).then((val) => val.sdp);
}

/// Restarts ICE on the provided [RTCPeerConnection].
void _restartIce(RTCPeerConnection conn) {
  throw UnimplementedError('PeerConnection.restartIce');
}

/// Adds provided [RTCIceCandidate] to the provided [RTCPeerConnection].
Object _addIceCandidate(RTCPeerConnection conn, RTCIceCandidate candidate) {
  return () => conn.addCandidate(candidate);
}

/// Returns current [RTCPeerConnection.connectionState] of the provided
/// [RTCPeerConnection].
Pointer _connectionState(RTCPeerConnection conn) {
  if (conn.connectionState != null) {
    return ForeignValue.fromInt(conn.connectionState!.index).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns current [RTCPeerConnection.iceConnectionState] of the provided
/// [RTCPeerConnection].
int _iceConnectionState(RTCPeerConnection conn) {
  if (conn.iceConnectionState != null) {
    return conn.iceConnectionState!.index;
  } else {
    return RTCIceConnectionState.RTCIceConnectionStateNew.index;
  }
}

/// Rollbacks local SDP offer of the provided [RTCPeerConnection].
Object _rollback(RTCPeerConnection conn) {
  return () => conn.setLocalDescription(RTCSessionDescription(null, 'rollback'));
}

/// Returns all [RTCRtpTransceiver]s of the provided [RTCPeerConnection].
Object getTransceivers(RTCPeerConnection conn) {
  return () => conn.getTransceivers();
}
