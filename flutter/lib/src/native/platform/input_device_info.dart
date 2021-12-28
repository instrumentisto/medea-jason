import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

import 'input_device_info.g.dart' as bridge;

/// Registers functions allowing Rust to operate Dart [MediaDeviceInfo].
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    deviceId: Pointer.fromFunction(_deviceId),
    label: Pointer.fromFunction(_label),
    groupId: Pointer.fromFunction(_groupId),
    kind: Pointer.fromFunction(_kind, 2),
  );
}

/// Returns [MediaDeviceInfo.deviceId] value.
Pointer<Utf8> _deviceId(MediaDeviceInfo deviceInfo) {
  return deviceInfo.deviceId.toNativeUtf8();
}

/// Returns [MediaDeviceInfo.label] value.
Pointer<Utf8> _label(MediaDeviceInfo deviceInfo) {
  return deviceInfo.label.toNativeUtf8();
}

/// Returns [MediaDeviceInfo.groupId] value.
Pointer _groupId(MediaDeviceInfo deviceInfo) {
  if (deviceInfo.groupId != null) {
    return ForeignValue.fromString(deviceInfo.groupId!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns [MediaDeviceInfo.kind] value.
int _kind(MediaDeviceInfo deviceInfo) {
  // TODO: Refactor flutter-webrtc to use enum instead of String.
  if (deviceInfo.kind == 'audioinput') {
    return 0;
  } else if (deviceInfo.kind == 'videoinput') {
    return 1;
  } else if (deviceInfo.kind == 'audiooutput') {
    return 2;
  } else {
    // Not supposed to ever happen.
    throw StateError('Unknown MediaKind: ${deviceInfo.kind}');
  }
}
