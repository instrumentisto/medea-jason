import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'ice_candidate.g.dart' as bridge;

/// Registers functions allowing Rust to create Dart [IceCandidate]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    init: Pointer.fromFunction(_new),
    candidate: Pointer.fromFunction(_candidate),
    sdpMLineIndex: Pointer.fromFunction(_sdpMLineIndex, 0),
    sdpMid: Pointer.fromFunction(_sdpMid),
  );
}

/// Returns the provided [IceCandidate] [String].
Pointer<Utf8> _candidate(Object iceCandidate) {
  iceCandidate as webrtc.IceCandidate;
  return iceCandidate.candidate.toNativeUtf8();
}

/// Returns SDP M line index of the provided [IceCandidate].
int _sdpMLineIndex(Object iceCandidate) {
  iceCandidate as webrtc.IceCandidate;
  return iceCandidate.sdpMLineIndex;
}

/// Returns SDP MID of the provided [IceCandidate].
Pointer<Utf8> _sdpMid(Object iceCandidate) {
  iceCandidate as webrtc.IceCandidate;

  return iceCandidate.sdpMid.toNativeUtf8();
}

/// Creates a new [IceCandidate] with the provided values.
Object _new(
    ForeignValue candidate, ForeignValue sdpMid, ForeignValue sdpMlineIndex) {
  var candidateArg = candidate.toDart();
  var sdpMidArg = sdpMid.toDart();
  var sdpMLineIndexArg = sdpMlineIndex.toDart();
  return webrtc.IceCandidate(sdpMidArg, sdpMLineIndexArg, candidateArg);
}
