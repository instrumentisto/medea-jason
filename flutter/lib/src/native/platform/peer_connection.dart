import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:flutter_webrtc/src/model/ice_candidate.dart';
import 'package:flutter_webrtc/src/model/media_stream_track_state.dart';
import 'package:flutter_webrtc/src/model/rtp_transceiver_init.dart';
import 'package:flutter_webrtc/src/model/transceiver_direction.dart';
import 'package:flutter_webrtc/src/model/peer_connection_config.dart';
import 'package:flutter_webrtc/src/model/session_description.dart';

import 'peer_connection.g.dart' as bridge;
import '../ffi/foreign_value.dart';

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
  );
}

/// Adds [RtpTransceiver] to the provided [PeerConnection].
///
/// Returns [Future] which will be resolved into created [RtpTransceiver].
Object _addTransceiver(PeerConnection peer, int kind, int direction) {
  return () => peer.addTransceiver(MediaKind.values[kind],
      RtpTransceiverInit(TransceiverDirection.values[direction]));
}

/// Returns newly created [PeerConnection] with a provided `iceServers`
/// [List].
Object _newPeer(Object iceServers) {
  var servers = iceServers as List<dynamic>;
  return () => PeerConnection.create(
      IceTransportType.all, servers.map((e) => e as IceServer).toList());
}

/// Adds subscription on [PeerConnection.onTrack] to the provided
/// [PeerConnection].
void _onTrack(PeerConnection conn, Function f) {
  conn.onTrack((track, transceiver) {
    f(track, transceiver);
  });
}

/// Add subscription on [PeerConnection.onIceCandidate] to the provided
/// [PeerConnection].
void _onIceCandidate(PeerConnection conn, Function f) {
  conn.onIceCandidate((e) {
    f(e);
  });
}

/// Adds subscription on [PeerConnection.onIceConnectionStateChange] to the
/// provided [PeerConnection].
void _onIceConnectionStateChange(PeerConnection conn, Function f) {
  conn.onIceConnectionStateChange((e) {
    f(e.index);
  });
}

/// Adds subscription on [PeerConnection.onConnectionStateChange] to the
/// provided [PeerConnection].
void _onConnectionStateChange(PeerConnection conn, Function f) {
  conn.onConnectionStateChange((e) {
    f(e.index);
  });
}

/// Lookups [RtpTransceiver] in the provided [PeerConnection] by the
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

/// Sets remote SDP offer in the provided [PeerConnection].
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

/// Sets local SDP offer in the provided [PeerConnection].
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

/// Creates new SDP offer for the provided [PeerConnection].
Object _createOffer(PeerConnection conn) {
  return () => conn.createOffer().then((val) => val.description);
}

/// Creates new SDP answer for the provided [PeerConnection].
Object _createAnswer(PeerConnection conn) {
  return () => conn.createAnswer().then((val) => val.description);
}

/// Restarts ICE on the provided [PeerConnection].
void _restartIce(PeerConnection conn) {
  conn.restartIce();
}

/// Adds provided [IceCandidate] to the provided [PeerConnection].
Object _addIceCandidate(PeerConnection conn, IceCandidate candidate) {
  return () => conn.addIceCandidate(candidate);
}

/// Returns current [PeerConnection.connectionState] of the provided
/// [PeerConnection].
Pointer _connectionState(PeerConnection conn) {
  return ForeignValue.fromInt(conn.connectionState().index).intoRustOwned();
}

/// Returns current [PeerConnection.iceConnectionState] of the provided
/// [PeerConnection].
int _iceConnectionState(PeerConnection conn) {
  return conn.iceConnectionState().index;
}

/// Rollbacks local SDP offer of the provided [PeerConnection].
Object _rollback(PeerConnection conn) {
  return () => conn.setLocalDescription(
      SessionDescription(SessionDescriptionType.rollback, ''));
}

/// Returns all [RtpTransceiver]s of the provided [PeerConnection].
Object getTransceivers(PeerConnection conn) {
  return () => conn.getTransceivers();
}

/// Closes the provided [PeerConnection].
void _close(PeerConnection conn) {
  conn.close();
}
