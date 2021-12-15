import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

import 'input_device_info.g.dart' as bridge;

/// Registers functions allowing Rust to create Dart [MediaDeviceInfo]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    deviceId: Pointer.fromFunction(deviceId),
    label: Pointer.fromFunction(label),
    groupId: Pointer.fromFunction(groupId),
    kind: Pointer.fromFunction(kind),
  );
}

/// Returns `deviceId` field of the provided [MediaDeviceInfo].
Pointer deviceId(MediaDeviceInfo deviceInfo) {
  return ForeignValue.fromString(deviceInfo.deviceId).intoRustOwned();
}

/// Returns `label` field of the provided [MediaDeviceInfo].
Pointer label(MediaDeviceInfo deviceInfo) {
  return ForeignValue.fromString(deviceInfo.label).intoRustOwned();
}

/// Returns `groupId` field of the provided [MediaDeviceInfo].
Pointer groupId(MediaDeviceInfo deviceInfo) {
  if (deviceInfo.groupId != null) {
    return ForeignValue.fromString(deviceInfo.groupId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns `kind` field of the provided [MediaDeviceInfo].
Pointer kind(MediaDeviceInfo deviceInfo) {
  if (deviceInfo.kind != null) {
    if (deviceInfo.kind!.contains('audio')) {
      return ForeignValue.fromInt(0).intoRustOwned();
    } else if (deviceInfo.kind!.contains('video')) {
      return ForeignValue.fromInt(1).intoRustOwned();
    } else {
      throw Exception('Unknown MediaKind: ${deviceInfo.kind}');
    }
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}
