import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

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
Pointer<Utf8> _deviceId(webrtc.MediaDeviceInfo deviceInfo) {
  return deviceInfo.deviceId.toNativeUtf8();
}

/// Returns [MediaDeviceInfo.label] value.
Pointer<Utf8> _label(webrtc.MediaDeviceInfo deviceInfo) {
  return deviceInfo.label.toNativeUtf8();
}

/// Returns [MediaDeviceInfo.groupId] value.
Pointer _groupId(webrtc.MediaDeviceInfo deviceInfo) {
  return ForeignValue.none().intoRustOwned();
}

/// Indicates whether the last attempt to use the provided device failed.
bool _isFailed(webrtc.MediaDeviceInfo deviceInfo) {
  return deviceInfo.isFailed;
}

/// Returns [MediaDeviceInfo.kind] value.
int _kind(webrtc.MediaDeviceInfo deviceInfo) {
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
