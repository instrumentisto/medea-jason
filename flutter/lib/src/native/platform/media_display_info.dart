import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/src/model/device.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'media_display_info.g.dart' as bridge;

/// Registers functions allowing Rust to operate Dart [MediaDisplayInfo].
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    deviceId: Pointer.fromFunction(_deviceId),
    title: Pointer.fromFunction(_title),
  );
}

/// Returns [MediaDeviceInfo.deviceId] value.
Pointer<Utf8> _deviceId(MediaDeviceInfo deviceInfo) {
  return deviceInfo.deviceId.toNativeUtf8();
}

/// Returns [MediaDeviceInfo.title] value.
Pointer _title(MediaDisplayInfo displayInfo) {
  if (displayInfo.title != null) {
    return ForeignValue.fromString(displayInfo.title!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}
