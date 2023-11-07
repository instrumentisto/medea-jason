import 'dart:ffi';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'parameters.g.dart' as bridge;

void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(dl,
      encodings: Pointer.fromFunction(_encodings),
      setEncoding: Pointer.fromFunction(_setEncodings));
}

/// Returns [SendEncodingParameters] from this [RtpParameters].
Object _encodings(RtpParameters parameters) {
  return () => parameters.encodings();
}

/// Sets [SendEncodingParameters] into this [RtpParameters].
Object _setEncodings(
    RtpParameters parameters, SendEncodingParameters encoding) {
  return () => parameters.setEncodings(encoding);
}
