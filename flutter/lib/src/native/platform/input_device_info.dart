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
    kind: Pointer.fromFunction(_kind, 2),
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
int _kind(MediaDeviceInfo deviceInfo) {
  if (deviceInfo.kind == 'audioinput') {
    return 0;
  } else if (deviceInfo.kind == 'videoinput') {
    return 1;
  } else if (deviceInfo.kind == 'audiooutput') {
    return 2;
  } else {
    throw Exception('Unknown MediaKind: ${deviceInfo.kind}');
  }
}
