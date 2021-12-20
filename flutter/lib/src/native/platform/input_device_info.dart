import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

import 'input_device_info.g.dart' as bridge;

/// Registers functions allowing Rust to create Dart [MediaDeviceInfo]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    deviceId: Pointer.fromFunction(_deviceId),
    label: Pointer.fromFunction(_label),
    groupId: Pointer.fromFunction(_groupId),
    kind: Pointer.fromFunction(_kind),
  );
}

/// Returns `deviceId` field of the provided [MediaDeviceInfo].
Pointer<Utf8> _deviceId(MediaDeviceInfo deviceInfo) {
  return deviceInfo.deviceId.toNativeUtf8();
}

/// Returns `label` field of the provided [MediaDeviceInfo].
Pointer<Utf8> _label(MediaDeviceInfo deviceInfo) {
  return deviceInfo.label.toNativeUtf8();
}

/// Returns `groupId` field of the provided [MediaDeviceInfo].
Pointer _groupId(MediaDeviceInfo deviceInfo) {
  if (deviceInfo.groupId != null) {
    return ForeignValue.fromString(deviceInfo.groupId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns `kind` field of the provided [MediaDeviceInfo].
Pointer _kind(MediaDeviceInfo deviceInfo) {
  // TODO: Why kind is nullable?
  if (deviceInfo.kind != null) {
    if (deviceInfo.kind == 'audioinput') {
      return ForeignValue.fromInt(0).intoRustOwned();
    } else if (deviceInfo.kind == 'videoinput') {
      return ForeignValue.fromInt(1).intoRustOwned();
    } else {
      // TODO: Handle audiooutput?
      throw Exception('Unknown MediaKind: ${deviceInfo.kind}');
    }
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}
