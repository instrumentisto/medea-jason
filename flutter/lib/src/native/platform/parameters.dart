import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/native_string.dart';
import '../ffi/foreign_value.dart';
import 'parameters.g.dart' as bridge;

void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(dl,
      encodings: Pointer.fromFunction(_encodings),
      setEncoding: Pointer.fromFunction(_setEncodings));
}

Object _encodings(RtpParameters parameters) {
  return () => parameters.encodings();
}

Object _setEncodings(
    RtpParameters parameters, SendEncodingParameters encoding) {
  return () => parameters.setEncodings(encoding);
}
