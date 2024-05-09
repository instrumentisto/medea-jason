import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/native_string.dart';
import 'send_encoding_parameters.g.dart' as bridge;

/// Registers an [SendEncodingParameters] related functions in Rust.
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

/// Creates a new [SendEncodingParameters].
Object _newSendEncodingParameters(Pointer<Utf8> rid, bool active) {
  return SendEncodingParameters.create(rid.nativeStringToDartString(), active);
}

/// Returns [SendEncodingParameters.rid] from the provided [SendEncodingParameters].
Pointer<Utf8> _getRid(SendEncodingParameters encoding) {
  return encoding.rid.toNativeUtf8();
}

/// Sets [SendEncodingParameters.active] in the provided [SendEncodingParameters].
void _setActive(SendEncodingParameters encoding, bool active) {
  encoding.active = active;
}

/// Sets [SendEncodingParameters.maxBitrate] in the provided
/// [SendEncodingParameters].
void _setMaxBitrate(SendEncodingParameters encoding, int maxBitrate) {
  encoding.maxBitrate = maxBitrate;
}

/// Sets [SendEncodingParameters.scaleResolutionDownBy] in the provided
/// [SendEncodingParameters].
void _setScaleResolutionDownBy(
    SendEncodingParameters encoding, int scaleResolutionDownBy) {
  encoding.scaleResolutionDownBy = scaleResolutionDownBy.toDouble();
}

/// Sets [SendEncodingParameters.scalabilityMode] in the provided
/// [SendEncodingParameters].
void _setScalabilityMode(
    SendEncodingParameters encoding, Pointer<Utf8> scalabilityMode) {
  encoding.scalabilityMode = scalabilityMode.nativeStringToDartString();
}
