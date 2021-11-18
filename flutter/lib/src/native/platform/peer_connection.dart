import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'package:ffi/ffi.dart';

import 'peer_connection.g.dart' as bridge;
import '../ffi/foreign_value.dart';

/// Registers [RTCPeerConnection] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    setRemoteDescription: setRemoteDescription,
    setLocalDescription: setLocalDescription,
    addIceCandidate: addIceCandidate,
    iceConnectionState: iceConnectionState,
    connectionState: connectionState,
    restartIce: restartIce,
    rollback: rollback,
    onTrack: onTrack,
    onIceCandidate: onIceCandidate,
    onIceConnectionStateChange: onIceConnectionStateChange,
    newPeer: newPeer,
    addTransceiver: addTransceiver,
    createOffer: createOffer,
    createAnswer: createAnswer,
    getTransceiverByMid: getTransceiverByMid,
    onConnectionStateChange: onConnectionStateChange,
  );
}

/// Adds [RTCRtpTransceiver] to the provided [RTCPeerConnection].
///
/// Returns [Future] which will be resolved into created [RTCRtpTransceiver].
Object addTransceiver(Object peer, int kind, int direction) {
  peer as RTCPeerConnection;
  return peer.addTransceiver(
    kind: RTCRtpMediaType.values[kind],
    init: RTCRtpTransceiverInit(direction: TransceiverDirection.SendRecv),
  );
}

/// Returns newly created [RTCPeerConnection] with a provided `iceServers`
/// [List].
Object newPeer(Object iceServers) {
  return createPeerConnection(
      {'iceServers': iceServers, 'sdpSemantics': 'unified-plan'});
}

/// Adds subscription on [RTCPeerConnection.onTrack] to the provided
/// [RTCPeerConnection].
void onTrack(Object conn, Object f) {
  conn as RTCPeerConnection;
  f as Function;
  conn.onTrack = (e) {
    f(e.track, e.transceiver);
  };
}

/// Add subscription on [RTCPeerConnection.onIceCandidate] to the provided
/// [RTCPeerConnection].
void onIceCandidate(Object conn, Object f) {
  conn as RTCPeerConnection;
  f as Function;
  conn.onIceCandidate = (e) {
    f(e);
  };
}

/// Adds subscription on [RTCPeerConnection.onIceConnectionState] to the
/// provided [RTCPeerConnection].
void onIceConnectionStateChange(Object conn, Object f) {
  conn as RTCPeerConnection;
  f as Function;
  conn.onIceConnectionState = (e) {
    f(e.index);
  };
}

/// Adds subscription on [RTCPeerConnection.onConnectionState] to the
/// provided [RTCPeerConnection].
void onConnectionStateChange(Object conn, Object f) {
  conn as RTCPeerConnection;
  f as Function;
  conn.onConnectionState = (e) {
    f(e.index);
  };
}

/// Lookups [RTCRtpTransceiver] in the provided [RTCPeerConnection] by the
/// provided [String].
Object getTransceiverByMid(Object peer, Pointer<Utf8> mid) {
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
Object setRemoteDescription(
    Object conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  conn as RTCPeerConnection;
  var desc = RTCSessionDescription(sdp.toDartString(), type.toDartString());
  return conn.setRemoteDescription(desc);
}

/// Sets local SDP offer in the provided [RTCPeerConnection].
Object setLocalDescription(Object conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  conn as RTCPeerConnection;
  return conn.setLocalDescription(
      RTCSessionDescription(sdp.toDartString(), type.toDartString()));
}

/// Creates new SDP offer for the provided [RTCPeerConnection].
Object createOffer(Object conn) {
  conn as RTCPeerConnection;
  return conn.createOffer({}).then((val) => val.sdp);
}

/// Creates new SDP answer for the provided [RTCPeerConnection].
Object createAnswer(Object conn) {
  conn as RTCPeerConnection;
  return conn.createAnswer({}).then((val) => val.sdp);
}

/// Restarts ICE on the provided [RTCPeerConnection].
void restartIce(Object conn) {
  conn as RTCPeerConnection;
  throw UnimplementedError('PeerConnection.restartIce');
}

/// Adds provided [RTCIceCandidate] to the provided [RTCPeerConnection].
Object addIceCandidate(Object conn, Object candidate) {
  conn as RTCPeerConnection;
  candidate as RTCIceCandidate;
  return conn.addCandidate(candidate);
}

/// Returns current [RTCPeerConnection.connectionState] of the provided
/// [RTCPeerConnection].
Pointer connectionState(Object conn) {
  conn as RTCPeerConnection;
  if (conn.connectionState != null) {
    return ForeignValue.fromInt(conn.connectionState!.index).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns current [RTCPeerConnection.iceConnectionState] of the provided
/// [RTCPeerConnection].
int iceConnectionState(Object conn) {
  conn as RTCPeerConnection;
  if (conn.iceConnectionState != null) {
    return conn.iceConnectionState!.index;
  } else {
    return RTCIceConnectionState.RTCIceConnectionStateNew.index;
  }
}

/// Rollbacks local SDP offer of the provided [RTCPeerConnection].
Object rollback(Object conn) {
  conn as RTCPeerConnection;
  return conn.setLocalDescription(RTCSessionDescription(null, 'rollback'));
}

/// Returns all [RTCRtpTransceiver]s of the provided [RTCPeerConnection].
Object getTransceivers(Object conn) {
  conn as RTCPeerConnection;
  return conn.getTransceivers();
}
