import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register_InputDeviceInfo__device_id')(
      Pointer.fromFunction<Pointer Function(Handle)>(deviceId));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register_InputDeviceInfo__label')(
      Pointer.fromFunction<Pointer Function(Handle)>(label));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register_InputDeviceInfo__group_id')(
      Pointer.fromFunction<Pointer Function(Handle)>(groupId));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register_InputDeviceInfo__kind')(
      Pointer.fromFunction<Pointer Function(Handle)>(kind));
}

// TODO: can be just String, because device_id is always Some.
Pointer deviceId(MediaDeviceInfo deviceInfo) {
  if (deviceInfo.deviceId != null) {
    return ForeignValue.fromString(deviceInfo.deviceId).intoBoxed();
  } else {
    return ForeignValue.none().intoBoxed();
  }
}

// TODO: can be just String, because label is always Some.
Pointer label(MediaDeviceInfo deviceInfo) {
  if (deviceInfo.label != null) {
    return ForeignValue.fromString(deviceInfo.label).intoBoxed();
  } else {
    return ForeignValue.none().intoBoxed();
  }
}

Pointer groupId(MediaDeviceInfo deviceInfo) {
  if (deviceInfo.groupId != null) {
    return ForeignValue.fromString(deviceInfo.groupId!).intoBoxed();
  } else {
    return ForeignValue.none().intoBoxed();
  }
}

Pointer kind(MediaDeviceInfo deviceInfo) {
  if (deviceInfo.kind != null) {
    return ForeignValue.fromString(deviceInfo.kind!).intoBoxed();
  } else {
    return ForeignValue.none().intoBoxed();
  }
}
