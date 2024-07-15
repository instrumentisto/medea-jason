import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'media_device_info.g.dart' as bridge;

/// Registers functions allowing Rust to operate Dart [MediaDeviceInfo].
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    deviceId: _deviceId,
    label: _label,
    groupId: _groupId,
    kind: _kind,
    isFailed: _isFailed,
  );
}

/// Returns [MediaDeviceInfo.deviceId] value.
Pointer<Utf8> _deviceId(Object deviceInfo) {
  deviceInfo as webrtc.MediaDeviceInfo;
  return deviceInfo.deviceId.toNativeUtf8();
}

/// Returns [MediaDeviceInfo.label] value.
Pointer<Utf8> _label(Object deviceInfo) {
  deviceInfo as webrtc.MediaDeviceInfo;
  return deviceInfo.label.toNativeUtf8();
}

/// Returns [MediaDeviceInfo.groupId] value.
Pointer _groupId(Object deviceInfo) {
  deviceInfo as webrtc.MediaDeviceInfo;
  return ForeignValue.none().intoRustOwned();
}

/// Indicates whether the last attempt to use the provided device failed.
bool _isFailed(Object deviceInfo) {
  deviceInfo as webrtc.MediaDeviceInfo;
  return deviceInfo.isFailed;
}

/// Returns [MediaDeviceInfo.kind] value.
int _kind(Object deviceInfo) {
  deviceInfo as webrtc.MediaDeviceInfo;
  switch (deviceInfo.kind) {
    case webrtc.MediaDeviceKind.audioinput:
      return 0;
    case webrtc.MediaDeviceKind.videoinput:
      return 1;
    case webrtc.MediaDeviceKind.audiooutput:
      return 2;
    default:
      // Not supposed to ever happen.
      throw StateError('Unknown MediaKind: ${deviceInfo.kind}');
  }
}
