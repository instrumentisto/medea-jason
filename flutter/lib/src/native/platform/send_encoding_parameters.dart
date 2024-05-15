import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/native_string.dart';
import 'send_encoding_parameters.g.dart' as bridge;

/// Registers [SendEncodingParameters] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    newSendEncodingParameters: Pointer.fromFunction(_newSendEncodingParameters),
    getRid: Pointer.fromFunction(_getRid),
    setActive: Pointer.fromFunction(_setActive),
    setMaxBitrate: Pointer.fromFunction(_setMaxBitrate),
    setScaleResolutionDownBy: Pointer.fromFunction(_setScaleResolutionDownBy),
    setScalabilityMode: Pointer.fromFunction(_setScalabilityMode),
  );
}

/// Creates new [SendEncodingParameters].
Object _newSendEncodingParameters(Pointer<Utf8> rid, bool active) {
  return SendEncodingParameters.create(rid.nativeStringToDartString(), active);
}

/// Returns [SendEncodingParameters.rid] from the provided
/// [SendEncodingParameters].
Pointer<Utf8> _getRid(Object encoding) {
  encoding as SendEncodingParameters;
  return encoding.rid.toNativeUtf8();
}

/// Sets [SendEncodingParameters.active] of the provided
/// [SendEncodingParameters].
void _setActive(Object encoding, bool active) {
  encoding as SendEncodingParameters;
  encoding.active = active;
}

/// Sets [SendEncodingParameters.maxBitrate] of the provided
/// [SendEncodingParameters].
void _setMaxBitrate(Object encoding, int maxBitrate) {
  encoding as SendEncodingParameters;
  encoding.maxBitrate = maxBitrate;
}

/// Sets [SendEncodingParameters.scaleResolutionDownBy] of the provided
/// [SendEncodingParameters].
void _setScaleResolutionDownBy(Object encoding, int scaleResolutionDownBy) {
  encoding as SendEncodingParameters;
  encoding.scaleResolutionDownBy = scaleResolutionDownBy.toDouble();
}

/// Sets [SendEncodingParameters.scalabilityMode] of the provided
/// [SendEncodingParameters].
void _setScalabilityMode(Object encoding, Pointer<Utf8> scalabilityMode) {
  encoding as SendEncodingParameters;
  encoding.scalabilityMode = scalabilityMode.nativeStringToDartString();
}
