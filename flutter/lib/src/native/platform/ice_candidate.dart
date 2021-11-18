import 'dart:ffi';

import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

import 'ice_candidate.g.dart' as bridge;

/// Registers functions allowing Rust to create Dart [RTCIceCandidate]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    constructNew: newRtcIceCandidate,
    candidate: candidate,
    sdpMLineIndex: sdpMLineIndex,
    sdpMid: sdpMid,
  );
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_IceCandidate__new')(
      Pointer.fromFunction<
          Handle Function(
              ForeignValue, ForeignValue, ForeignValue)>(newRtcIceCandidate));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_IceCandidate__candidate')(
      Pointer.fromFunction<Pointer Function(Handle)>(candidate));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_IceCandidate__sdp_m_line_index')(
      Pointer.fromFunction<Pointer Function(Handle)>(sdpMLineIndex));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_IceCandidate__sdp_mid')(
      Pointer.fromFunction<Pointer Function(Handle)>(sdpMid));
}

/// Returns the provided [RTCIceCandidate] [String].
Pointer candidate(Object iceCandidate) {
  iceCandidate as RTCIceCandidate;
  if (iceCandidate.candidate != null) {
    return ForeignValue.fromString(iceCandidate.candidate!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns SDP M line index of the provided [RTCIceCandidate].
Pointer sdpMLineIndex(Object iceCandidate) {
  iceCandidate as RTCIceCandidate;
  if (iceCandidate.sdpMlineIndex != null) {
    return ForeignValue.fromInt(iceCandidate.sdpMlineIndex!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns SDP MID of the provided [RTCIceCandidate].
Pointer sdpMid(Object iceCandidate) {
  iceCandidate as RTCIceCandidate;
  if (iceCandidate.sdpMid != null) {
    return ForeignValue.fromString(iceCandidate.sdpMid!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Creates a new [RTCIceCandidate] with the provided values.
Object newRtcIceCandidate(
    ForeignValue candidate, ForeignValue sdpMid, ForeignValue sdpMlineIndex) {
  var candidateArg = candidate.toDart();
  var sdpMidArg = sdpMid.toDart();
  var sdpMLineIndexArg = sdpMlineIndex.toDart();
  return RTCIceCandidate(candidateArg, sdpMidArg, sdpMLineIndexArg);
}
