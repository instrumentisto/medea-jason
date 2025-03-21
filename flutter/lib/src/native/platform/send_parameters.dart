import 'dart:ffi';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'send_parameters.g.dart' as bridge;

/// Registers [RtpParameters] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(dl, encodings: _encodings);
}

/// Returns [SendEncodingParameters] from the provided [RtpParameters].
List<SendEncodingParameters> _encodings(Object parameters) {
  parameters as RtpParameters;
  return parameters.encodings;
}
