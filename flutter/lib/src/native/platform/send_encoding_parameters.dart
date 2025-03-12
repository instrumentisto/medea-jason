import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'package:medea_jason/src/native/ffi/native_string.dart';
import 'send_encoding_parameters.g.dart' as bridge;

/// Registers [SendEncodingParameters] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    newSendEncodingParameters: _newSendEncodingParameters,
    getRid: _getRid,
    setActive: _setActive,
    getActive: _getActive,
    setMaxBitrate: _setMaxBitrate,
    getMaxBitrate: _getMaxBitrate,
    setScaleResolutionDownBy: _setScaleResolutionDownBy,
    getScaleResolutionDownBy: _getScaleResolutionDownBy,
    setScalabilityMode: _setScalabilityMode,
    getScalabilityMode: _getScalabilityMode,
  );
}

/// Creates new [SendEncodingParameters].
SendEncodingParameters _newSendEncodingParameters(
  Pointer<Utf8> rid,
  bool active,
) {
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

/// Returns [SendEncodingParameters.active] of the provided
/// [SendEncodingParameters].
bool _getActive(Object encoding) {
  encoding as SendEncodingParameters;
  return encoding.active;
}

/// Sets [SendEncodingParameters.maxBitrate] of the provided
/// [SendEncodingParameters].
void _setMaxBitrate(Object encoding, int maxBitrate) {
  encoding as SendEncodingParameters;
  encoding.maxBitrate = maxBitrate;
}

/// Returns [SendEncodingParameters.maxBitrate] of the provided
/// [SendEncodingParameters].
Pointer _getMaxBitrate(Object encoding) {
  encoding as SendEncodingParameters;

  if (encoding.maxBitrate != null) {
    return ForeignValue.fromInt(encoding.maxBitrate!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Sets [SendEncodingParameters.scaleResolutionDownBy] of the provided
/// [SendEncodingParameters].
void _setScaleResolutionDownBy(Object encoding, double scaleResolutionDownBy) {
  encoding as SendEncodingParameters;
  encoding.scaleResolutionDownBy = scaleResolutionDownBy;
}

/// Returns [SendEncodingParameters.scaleResolutionDownBy] of the provided
/// [SendEncodingParameters].
double _getScaleResolutionDownBy(Object encoding) {
  encoding as SendEncodingParameters;

  if (encoding.scaleResolutionDownBy != null) {
    return encoding.scaleResolutionDownBy!;
  } else {
    return 1.0;
  }
}

/// Sets [SendEncodingParameters.scalabilityMode] of the provided
/// [SendEncodingParameters].
void _setScalabilityMode(Object encoding, Pointer<Utf8> scalabilityMode) {
  encoding as SendEncodingParameters;
  encoding.scalabilityMode = scalabilityMode.nativeStringToDartString();
}

/// Returns [SendEncodingParameters.scalabilityMode] of the provided
/// [SendEncodingParameters].
Pointer<Utf8> _getScalabilityMode(Object encoding) {
  encoding as SendEncodingParameters;

  if (encoding.scalabilityMode != null) {
    return encoding.scalabilityMode!.toNativeUtf8();
  } else {
    return ''.toNativeUtf8();
  }
}
