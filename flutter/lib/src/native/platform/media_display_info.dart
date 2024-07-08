import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'media_display_info.g.dart' as bridge;

/// Registers functions allowing Rust to operate Dart [MediaDisplayInfo].
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    deviceId: _deviceId,
    title: _title,
  );
}

/// Returns [MediaDisplayInfo.deviceId] value.
Pointer<Utf8> _deviceId(Object displayInfo) {
  displayInfo as MediaDisplayInfo;
  return displayInfo.deviceId.toNativeUtf8();
}

/// Returns [MediaDisplayInfo.title] value.
Pointer _title(Object displayInfo) {
  displayInfo as MediaDisplayInfo;
  if (displayInfo.title != null) {
    return ForeignValue.fromString(displayInfo.title!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}
