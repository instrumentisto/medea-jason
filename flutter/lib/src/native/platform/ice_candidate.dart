import 'dart:ffi';

import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

import 'ice_candidate.g.dart' as bridge;

/// Registers functions allowing Rust to create Dart [RTCIceCandidate]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    init: Pointer.fromFunction(_new),
    candidate: Pointer.fromFunction(_candidate),
    sdpMLineIndex: Pointer.fromFunction(_sdpMLineIndex),
    sdpMid: Pointer.fromFunction(_sdpMid),
  );
}

/// Returns the provided [RTCIceCandidate] [String].
Pointer _candidate(RTCIceCandidate iceCandidate) {
  if (iceCandidate.candidate != null) {
    return ForeignValue.fromString(iceCandidate.candidate!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns SDP M line index of the provided [RTCIceCandidate].
Pointer _sdpMLineIndex(RTCIceCandidate iceCandidate) {
  if (iceCandidate.sdpMlineIndex != null) {
    return ForeignValue.fromInt(iceCandidate.sdpMlineIndex!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns SDP MID of the provided [RTCIceCandidate].
Pointer _sdpMid(RTCIceCandidate iceCandidate) {
  if (iceCandidate.sdpMid != null) {
    return ForeignValue.fromString(iceCandidate.sdpMid!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Creates a new [RTCIceCandidate] with the provided values.
Object _new(
    ForeignValue candidate, ForeignValue sdpMid, ForeignValue sdpMlineIndex) {
  var candidateArg = candidate.toDart();
  var sdpMidArg = sdpMid.toDart();
  var sdpMLineIndexArg = sdpMlineIndex.toDart();
  return RTCIceCandidate(candidateArg, sdpMidArg, sdpMLineIndexArg);
}
