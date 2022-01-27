import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:flutter_webrtc/src/model/ice_candidate.dart';
import 'package:flutter_webrtc/src/model/media_type.dart';
import 'package:flutter_webrtc/src/model/rtp_transceiver_init.dart';
import 'package:flutter_webrtc/src/model/transceiver_direction.dart';
import 'package:flutter_webrtc/src/model/peer_connection_config.dart';
import 'package:flutter_webrtc/src/model/session_description.dart';
import 'package:flutter_webrtc/src/model/peer_connections_states.dart';

import 'peer_connection.g.dart' as bridge;
import '../ffi/foreign_value.dart';

/// Registers [RTCPeerConnection] related functions in Rust.
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
  );
}

/// Adds [RTCRtpTransceiver] to the provided [RTCPeerConnection].
///
/// Returns [Future] which will be resolved into created [RTCRtpTransceiver].
Object _addTransceiver(PeerConnection peer, int kind, int direction) {
  return () => peer.addTransceiver(MediaType.values[kind],
      RtpTransceiverInit(TransceiverDirection.values[direction]));
}

/// Returns newly created [RTCPeerConnection] with a provided `iceServers`
/// [List].
Object _newPeer(Object iceServers) {
  var servers = iceServers as List<dynamic>;
  return () => PeerConnection.create(
      IceTransportType.all, servers.map((e) => e as IceServer).toList());
}

/// Adds subscription on [RTCPeerConnection.onTrack] to the provided
/// [RTCPeerConnection].
void _onTrack(PeerConnection conn, Function f) {
  conn.onTrack((track, transceiver) {
    f(track, transceiver);
  });
}

/// Add subscription on [RTCPeerConnection.onIceCandidate] to the provided
/// [RTCPeerConnection].
void _onIceCandidate(PeerConnection conn, Function f) {
  conn.onIceCandidate((e) {
    f(e);
  });
}

/// Adds subscription on [RTCPeerConnection.onIceConnectionState] to the
/// provided [RTCPeerConnection].
void _onIceConnectionStateChange(PeerConnection conn, Function f) {
  conn.onIceConnectionStateChange((e) {
    f(e.index);
  });
}

/// Adds subscription on [RTCPeerConnection.onConnectionState] to the
/// provided [RTCPeerConnection].
void _onConnectionStateChange(PeerConnection conn, Function f) {
  conn.onConnectionStateChange((e) {
    f(e.index);
  });
}

/// Lookups [RTCRtpTransceiver] in the provided [RTCPeerConnection] by the
/// provided [String].
Object _getTransceiverByMid(PeerConnection peer, Pointer<Utf8> mid) {
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
    PeerConnection conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  var sdpType;
  if (type.toDartString() == 'offer') {
    sdpType = SessionDescriptionType.offer;
  } else {
    sdpType = SessionDescriptionType.answer;
  }
  var desc = SessionDescription(sdpType, sdp.toDartString());
  return () => conn.setRemoteDescription(desc);
}

/// Sets local SDP offer in the provided [RTCPeerConnection].
Object _setLocalDescription(
    PeerConnection conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  var sdpType;
  if (type.toDartString() == 'offer') {
    sdpType = SessionDescriptionType.offer;
  } else {
    sdpType = SessionDescriptionType.answer;
  }
  return () =>
      conn.setLocalDescription(SessionDescription(sdpType, sdp.toDartString()));
}

/// Creates new SDP offer for the provided [RTCPeerConnection].
Object _createOffer(PeerConnection conn) {
  return () => conn.createOffer().then((val) => val.description);
}

/// Creates new SDP answer for the provided [RTCPeerConnection].
Object _createAnswer(PeerConnection conn) {
  return () => conn.createAnswer().then((val) => val.description);
}

/// Restarts ICE on the provided [RTCPeerConnection].
void _restartIce(PeerConnection conn) {
  conn.restartIce();
}

/// Adds provided [RTCIceCandidate] to the provided [RTCPeerConnection].
Object _addIceCandidate(PeerConnection conn, IceCandidate candidate) {
  return () => conn.addIceCandidate(candidate);
}

/// Returns current [RTCPeerConnection.connectionState] of the provided
/// [RTCPeerConnection].
Pointer _connectionState(PeerConnection conn) {
  if (conn.connectionState != null) {
    return ForeignValue.fromInt(conn.connectionState().index).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns current [RTCPeerConnection.iceConnectionState] of the provided
/// [RTCPeerConnection].
int _iceConnectionState(PeerConnection conn) {
  if (conn.iceConnectionState != null) {
    return conn.iceConnectionState().index;
  } else {
    return IceConnectionState.new_.index;
  }
}

/// Rollbacks local SDP offer of the provided [RTCPeerConnection].
Object _rollback(PeerConnection conn) {
  return () => conn.setLocalDescription(
      SessionDescription(SessionDescriptionType.rollback, ''));
}

/// Returns all [RTCRtpTransceiver]s of the provided [RTCPeerConnection].
Object getTransceivers(PeerConnection conn) {
  return () => conn.getTransceivers();
}

/// Closes the provided [RTCPeerConnection].
void _close(PeerConnection conn) {
  conn.close();
}
