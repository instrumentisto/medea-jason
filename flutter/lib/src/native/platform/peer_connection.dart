import 'package:flutter_webrtc/flutter_webrtc.dart';
import '../ffi/foreign_value.dart';
import 'dart:ffi';
import 'package:ffi/ffi.dart';

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
      Pointer.fromFunction<ForeignValue Function(Handle)>(connectionState));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__restart_ice')(
      Pointer.fromFunction<Void Function(Handle)>(restartIce));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__rollback')(
      Pointer.fromFunction<Void Function(Handle)>(rollback));

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
      Pointer.fromFunction<Handle Function()>(newPeer));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_RtcPeerConnection__add_transceiver')(
      Pointer.fromFunction<Handle Function(Handle, Int32, Int32)>(
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

Object addTransceiver(RTCPeerConnection peer, int kind, int direction) {
  return peer.addTransceiver(
    kind: RTCRtpMediaType.values[kind],
    init: RTCRtpTransceiverInit(direction: TransceiverDirection.SendRecv),
  );
}

Object newPeer() {
  return createPeerConnection({
    'iceServers': [
      {'url': 'stun:stun.l.google.com:19302'},
    ],
    'sdpSemantics': 'unified-plan'
  });
}

void onTrack(RTCPeerConnection conn, Function f) {
  conn.onTrack = (e) {
    f(e.track, e.transceiver);
  };
}

void onIceCandidate(RTCPeerConnection conn, Function f) {
  conn.onIceCandidate = (e) {
    f(e);
  };
}

void onIceConnectionStateChange(RTCPeerConnection conn, Function f) {
  conn.onIceConnectionState = (e) {
    f(e.index);
  };
}

void onConnectionStateChange(RTCPeerConnection conn, Function f) {
  conn.onConnectionState = (e) {
    f(e.index);
  };
}

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

Object setRemoteDescription(
    RTCPeerConnection conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  var desc = RTCSessionDescription(sdp.toDartString(), type.toDartString());
  return conn.setRemoteDescription(desc);
}

Object setLocalDescription(
    RTCPeerConnection conn, Pointer<Utf8> type, Pointer<Utf8> sdp) {
  return conn.setLocalDescription(
      RTCSessionDescription(sdp.toDartString(), type.toDartString()));
}

Object createOffer(RTCPeerConnection conn) {
  return conn.createOffer({}).then((val) => val.sdp);
}

Object createAnswer(RTCPeerConnection conn) {
  return conn.createAnswer({}).then((val) => val.sdp);
}

void restartIce(RTCPeerConnection conn) {
  throw UnimplementedError('PeerConnection.restartIce');
}

Object addIceCandidate(RTCPeerConnection conn, RTCIceCandidate candidate) {
  return conn.addCandidate(candidate);
}

ForeignValue connectionState(RTCPeerConnection conn) {
  if (conn.connectionState != null) {
    return ForeignValue.fromInt(conn.connectionState!.index).ref;
  } else {
    return ForeignValue.none().ref;
  }
}

ForeignValue iceConnectionState(RTCPeerConnection conn) {
  if (conn.iceConnectionState != null) {
    return ForeignValue.fromInt(conn.iceConnectionState!.index).ref;
  } else {
    return ForeignValue.none().ref;
  }
}

void rollback(RTCPeerConnection conn) {
  conn.setLocalDescription(RTCSessionDescription(null, "rollback"));
}

Object getTransceivers(RTCPeerConnection conn) {
  return conn.getTransceivers();
}
