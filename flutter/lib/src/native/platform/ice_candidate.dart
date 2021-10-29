import 'dart:ffi';

import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register_IceCandidate__new')(Pointer.fromFunction<
          Handle Function(ForeignValue, ForeignValue, ForeignValue)>(
      newRtcIceCandidate));
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

Pointer candidate(RTCIceCandidate iceCandidate) {
  if (iceCandidate.candidate != null) {
    return ForeignValue.fromString(iceCandidate.candidate!).intoBoxed();
  } else {
    return ForeignValue.none().intoBoxed();
  }
}

Pointer sdpMLineIndex(RTCIceCandidate iceCandidate) {
  if (iceCandidate.sdpMlineIndex != null) {
    return ForeignValue.fromInt(iceCandidate.sdpMlineIndex!).intoBoxed();
  } else {
    return ForeignValue.none().intoBoxed();
  }
}

Pointer sdpMid(RTCIceCandidate iceCandidate) {
  if (iceCandidate.sdpMid != null) {
    return ForeignValue.fromString(iceCandidate.sdpMid!).intoBoxed();
  } else {
    return ForeignValue.none().intoBoxed();
  }
}

Object newRtcIceCandidate(ForeignValue candidate, ForeignValue sdpMid, ForeignValue sdpMlineIndex) {
  var candidateArg = candidate.toDart();
  var sdpMidArg = sdpMid.toDart();
  var sdpMLineIndexArg = sdpMlineIndex.toDart();
  return RTCIceCandidate(candidateArg, sdpMidArg, sdpMLineIndexArg);
}
