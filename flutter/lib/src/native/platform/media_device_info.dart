import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/src/model/device.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'media_device_info.g.dart' as bridge;

/// Registers functions allowing Rust to operate Dart [MediaDeviceInfo].
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    deviceId: Pointer.fromFunction(_deviceId),
    label: Pointer.fromFunction(_label),
    groupId: Pointer.fromFunction(_groupId),
    kind: Pointer.fromFunction(_kind, 2),
    isFailed: Pointer.fromFunction(_isFailed, true),
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
  return ForeignValue.none().intoRustOwned();
}

bool _isFailed(MediaDeviceInfo deviceInfo) {
  return deviceInfo.isFailed;
}

/// Returns [MediaDeviceInfo.kind] value.
int _kind(MediaDeviceInfo deviceInfo) {
  switch (deviceInfo.kind) {
    case MediaDeviceKind.audioinput:
      return 0;
    case MediaDeviceKind.videoinput:
      return 1;
    case MediaDeviceKind.audiooutput:
      return 2;
    default:
      // Not supposed to ever happen.
      throw StateError('Unknown MediaKind: ${deviceInfo.kind}');
  }
}
