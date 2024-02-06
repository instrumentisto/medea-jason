import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'ice_candidate_error.g.dart' as bridge;

/// Registers functions allowing Rust to create Dart [IceCandidateErrorEvent]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    address: Pointer.fromFunction(_address),
    port: Pointer.fromFunction(_port, 0),
    url: Pointer.fromFunction(_url),
    errorCode: Pointer.fromFunction(_errorCode, 0),
    errorText: Pointer.fromFunction(_errorText),
  );
}

/// Returns local IP address used to communicate with a STUN or TURN
/// server.
Pointer<Utf8> _address(webrtc.IceCandidateErrorEvent error) {
  return error.address.toNativeUtf8();
}

/// Port used to communicate with a STUN or TURN server.
int _port(webrtc.IceCandidateErrorEvent error) {
  return error.port;
}

/// Returns STUN or TURN URL identifying the STUN or TURN server for
/// which the failure occurred.
Pointer<Utf8> _url(webrtc.IceCandidateErrorEvent error) {
  return error.url.toNativeUtf8();
}

/// Returns numeric STUN error code returned by the STUN or TURN server.
/// If no host candidate can reach the server, `errorCode` will
/// be set to the value 701 which is outside the STUN error code
/// range. This error is only fired once per server URL while in
/// the `RTCIceGatheringState` of "gathering".
int _errorCode(webrtc.IceCandidateErrorEvent error) {
  return error.errorCode;
}

/// Returns STUN reason text returned by the STUN or TURN server. If the
/// server could not be reached, `errorText` will be set to an
/// implementation-specific value providing details about the
/// error.
Pointer<Utf8> _errorText(webrtc.IceCandidateErrorEvent error) {
  return error.errorText.toNativeUtf8();
}
