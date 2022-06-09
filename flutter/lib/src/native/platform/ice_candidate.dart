import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:flutter_webrtc/src/model/ice.dart';

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
Pointer<Utf8> _candidate(IceCandidate iceCandidate) {
  return iceCandidate.candidate.toNativeUtf8();
}

/// Returns SDP M line index of the provided [IceCandidate].
int _sdpMLineIndex(IceCandidate iceCandidate) {
  return iceCandidate.sdpMLineIndex;
}

/// Returns SDP MID of the provided [IceCandidate].
Pointer<Utf8> _sdpMid(IceCandidate iceCandidate) {
  return iceCandidate.sdpMid.toNativeUtf8();
}

/// Creates a new [IceCandidate] with the provided values.
Object _new(
    ForeignValue candidate, ForeignValue sdpMid, ForeignValue sdpMlineIndex) {
  var candidateArg = candidate.toDart();
  var sdpMidArg = sdpMid.toDart();
  var sdpMLineIndexArg = sdpMlineIndex.toDart();
  return IceCandidate(sdpMidArg, sdpMLineIndexArg, candidateArg);
}
