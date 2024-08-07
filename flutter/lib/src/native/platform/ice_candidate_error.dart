import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import 'ice_candidate_error.g.dart' as bridge;

/// Registers functions allowing Rust to create Dart [IceCandidateErrorEvent]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    address: _address,
    port: _port,
    url: _url,
    errorCode: _errorCode,
    errorText: _errorText,
  );
}

/// Returns the local IP address used to communicate with a [STUN]/[TURN]
/// server.
///
/// [STUN]: https://webrtcglossary.com/stun
/// [TURN]: https://webrtcglossary.com/turn
Pointer<Utf8> _address(Object error) {
  error as webrtc.IceCandidateErrorEvent;
  return error.address.toNativeUtf8();
}

/// Returns the port used to communicate with a [STUN]/[TURN] server.
///
/// [STUN]: https://webrtcglossary.com/stun
/// [TURN]: https://webrtcglossary.com/turn
int _port(Object error) {
  error as webrtc.IceCandidateErrorEvent;
  return error.port;
}

/// Returns the URL identifying the [STUN]/[TURN] server for which the failure
/// occurred.
///
/// [STUN]: https://webrtcglossary.com/stun
/// [TURN]: https://webrtcglossary.com/turn
Pointer<Utf8> _url(Object error) {
  error as webrtc.IceCandidateErrorEvent;
  return error.url.toNativeUtf8();
}

/// Returns the Numeric [STUN] error code returned by the [STUN]/[TURN] server.
///
/// If no host candidate can reach the server, this error code will be set to
/// the value `701`, which is outside the [STUN] error code range. This error is
/// only fired once per server URL while in the `RTCIceGatheringState` of
/// "gathering".
///
/// [STUN]: https://webrtcglossary.com/stun
/// [TURN]: https://webrtcglossary.com/turn
int _errorCode(Object error) {
  error as webrtc.IceCandidateErrorEvent;
  return error.errorCode;
}

/// [STUN] reason text returned by the [STUN]/[TURN] server.
///
/// If the server could not be reached, this reason test will be set to an
/// implementation-specific value providing details about the error.
///
/// [STUN]: https://webrtcglossary.com/stun
/// [TURN]: https://webrtcglossary.com/turn
Pointer<Utf8> _errorText(Object error) {
  error as webrtc.IceCandidateErrorEvent;
  return error.errorText.toNativeUtf8();
}
