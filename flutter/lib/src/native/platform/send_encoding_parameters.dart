import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/native_string.dart';
import 'send_encoding_parameters.g.dart' as bridge;

/// Registers an [RtpTransceiver] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    newSendEncodingParameters: Pointer.fromFunction(_newSendEncodingParameters),
    setMaxBitrate: Pointer.fromFunction(_setMaxBitrate),
    setScaleResolutionDownBy: Pointer.fromFunction(_setScaleResolutionDownBy),
  );
}

Object _newSendEncodingParameters(Pointer<Utf8> rid, bool active) {
  print('zzzzzzzzzzzz');
  var r = rid.nativeStringToDartString();
  print('xxxxxxxxxxxx');
  return SendEncodingParameters(r, active);
}

void _setMaxBitrate(SendEncodingParameters encoding, int maxBitrate) {
  encoding.maxBitrate = maxBitrate;
}

void _setScaleResolutionDownBy(
    SendEncodingParameters encoding, int scaleResolutionDownBy) {
  encoding.scaleResolutionDownBy = scaleResolutionDownBy.toDouble();
}
