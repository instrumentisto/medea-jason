import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'package:ffi/ffi.dart';

import '../ffi/foreign_value.dart';

/// Registers [RTCPeerConnection] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__set_remote_description')(
      Pointer.fromFunction<
          Handle Function(
              Handle, Pointer<Utf8>, Pointer<Utf8>)>(setRemoteDescription));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__set_local_description')(
      Pointer.fromFunction<
          Handle Function(
              Handle, Pointer<Utf8>, Pointer<Utf8>)>(setLocalDescription));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__add_ice_candidate')(
      Pointer.fromFunction<Handle Function(Handle, Handle)>(addIceCandidate));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__ice_connection_state')(
      Pointer.fromFunction<ForeignValue Function(Handle)>(iceConnectionState));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__connection_state')(
      Pointer.fromFunction<Pointer Function(Handle)>(connectionState));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__restart_ice')(
      Pointer.fromFunction<Void Function(Handle)>(restartIce));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__rollback')(
      Pointer.fromFunction<Handle Function(Handle)>(rollback));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__on_track')(
      Pointer.fromFunction<Void Function(Handle, Handle)>(onTrack));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__on_ice_candidate')(
      Pointer.fromFunction<Void Function(Handle, Handle)>(onIceCandidate));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__on_ice_connection_state_change')(
      Pointer.fromFunction<Void Function(Handle, Handle)>(
          onIceConnectionStateChange));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__on_connection_state_change')(
      Pointer.fromFunction<Void Function(Handle, Handle)>(
          onConnectionStateChange));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__new_peer')(
      Pointer.fromFunction<Handle Function(Handle)>(newPeer));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__add_transceiver')(
      Pointer.fromFunction<Handle Function(Handle, Int64, Int64)>(
          addTransceiver));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__get_transceiver_by_mid')(
      Pointer.fromFunction<Handle Function(Handle, Pointer<Utf8>)>(
          getTransceiverByMid));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__create_offer')(
      Pointer.fromFunction<Handle Function(Handle)>(createOffer));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__create_answer')(
      Pointer.fromFunction<Handle Function(Handle)>(createAnswer));
}

/// Adds [RTCRtpTransceiver] to the provided [RTCPeerConnection].
///
/// Returns [Future] which will be resolved into created [RTCRtpTransceiver].
Object addTransceiver(RTCPeerConnection peer, int kind, int direction) {
  return peer.addTransceiver(
    kind: RTCRtpMediaType.values[kind],
    init: RTCRtpTransceiverInit(direction: TransceiverDirection.SendRecv),
  );
}

/// Returns newly created [RTCPeerConnection] with a provided `iceServers` [List].
Object newPeer(Object iceServers) {
  return createPeerConnection(
      {'iceServers': iceServers, 'sdpSemantics': 'unified-plan'});
}

/// Adds subscription on [RTCPeerConnection.onTrack] to the provided [RTCPeerConnection].
void onTrack(RTCPeerConnection conn, Function f) {
  conn.onTrack = (e) {
    f(e.track, e.transceiver);
  };
}

/// Add subscription on [RTCPeerConnection.onIceCandidate] to the provided [RTCPeerConnection].
void onIceCandidate(RTCPeerConnection conn, Function f) {
  conn.onIceCandidate = (e) {
    f(e);
  };
}

/// Adds subscription on [RTCPeerConnection.onIceConnectionState] to the provided [RTCPeerConnection].
void onIceConnectionStateChange(RTCPeerConnection conn, Function f) {
  conn.onIceConnectionState = (e) {
    f(e.index);
  };
}

/// Adds subscription on [RTCPeerConnection.onConnectionState] to the provided [RTCPeerConnection].
void onConnectionStateChange(RTCPeerConnection conn, Function f) {
  conn.onConnectionState = (e) {
    f(e.index);
  };
}

/// Lookups [RTCRtpTransceiver] in the provided [RTCPeerConnection] by the provided [String].
Object getTransceiverByMid(RTCPeerConnection peer, Pointer<Utf8> mid) {
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
    RTCPeerConnection conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  var desc = RTCSessionDescription(sdp.toDartString(), type.toDartString());
  return conn.setRemoteDescription(desc);
}

/// Sets local SDP offer in the provided [RTCPeerConnection].
Object setLocalDescription(
    RTCPeerConnection conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  return conn.setLocalDescription(
      RTCSessionDescription(sdp.toDartString(), type.toDartString()));
}

/// Creates new SDP offer for the provided [RTCPeerConnection].
Object createOffer(RTCPeerConnection conn) {
  return conn.createOffer({}).then((val) => val.sdp);
}

/// Creates new SDP answer for the provided [RTCPeerConnection].
Object createAnswer(RTCPeerConnection conn) {
  return conn.createAnswer({}).then((val) => val.sdp);
}

/// Restarts ICE on the provided [RTCPeerConnection].
void restartIce(RTCPeerConnection conn) {
  throw UnimplementedError('PeerConnection.restartIce');
}

/// Adds provided [RTCIceCandidate] to the provided [RTCPeerConnection].
Object addIceCandidate(RTCPeerConnection conn, RTCIceCandidate candidate) {
  return conn.addCandidate(candidate);
}

/// Returns current [RTCPeerConnection.connectionState] of the provided [RTCPeerConnection].
Pointer connectionState(RTCPeerConnection conn) {
  if (conn.connectionState != null) {
    return ForeignValue.fromInt(conn.connectionState!.index).intoBoxed();
  } else {
    return ForeignValue.none().intoBoxed();
  }
}

/// Returns current [RTCPeerConnection.iceConnectionState] of the provided [RTCPeerConnection].
ForeignValue iceConnectionState(RTCPeerConnection conn) {
  if (conn.iceConnectionState != null) {
    return ForeignValue.fromInt(conn.iceConnectionState!.index).ref;
  } else {
    return ForeignValue.none().ref;
  }
}

/// Rollbacks local SDP offer of the provided [RTCPeerConnection].
Object rollback(RTCPeerConnection conn) {
  return conn.setLocalDescription(RTCSessionDescription(null, 'rollback'));
}

/// Returns all [RTCRtpTransceiver]s of this [RTCPeerConnection].
Object getTransceivers(RTCPeerConnection conn) {
  return conn.getTransceivers();
}
