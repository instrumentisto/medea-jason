import 'dart:ffi';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'parameters.g.dart' as bridge;

/// Registers [RtpParameters] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(dl,
      encodings: Pointer.fromFunction(_encodings),
      setEncoding: Pointer.fromFunction(_setEncodings));
}

/// Returns [SendEncodingParameters] from the provided [RtpParameters].
Object _encodings(Object parameters) {
  parameters as RtpParameters;
  return () => parameters.encodings;
}

/// Sets the provided [SendEncodingParameters] into the provided
/// [RtpParameters].
Object _setEncodings(
    Object parameters, Object encoding) {
  parameters as RtpParameters;
  encoding as SendEncodingParameters;
  return () => parameters.encodings.add(encoding);
}
