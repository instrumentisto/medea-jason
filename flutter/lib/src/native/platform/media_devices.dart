import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:flutter_webrtc/src/model/constraints.dart';

import 'media_devices.g.dart' as bridge;

/// Registers functions allowing Rust to operate Dart media devices.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    enumerateDevices: Pointer.fromFunction(_enumerateDevices),
    getUserMedia: Pointer.fromFunction(_getUserMedia),
    getDisplayMedia: Pointer.fromFunction(_getDisplayMedia),
    setOutputAudioId: Pointer.fromFunction(_setOutputAudioId),
    onDeviceChange: Pointer.fromFunction(_onDeviceChange),
  );
}

/// Requests media input access and returns the created [MediaStreamTrack]s.
Object _getUserMedia(DeviceConstraints constraints) {
  return () => getUserMedia(constraints);
}

/// Returns all the available media devices.
Object _enumerateDevices() {
  return () => enumerateDevices();
}

/// Starts capturing the contents of a display and returns the created
/// [MediaStreamTrack]s.
Object _getDisplayMedia(DisplayConstraints constraints) {
  return () => getDisplayMedia(constraints);
}

/// Switches output audio device to the device with the provided [deviceId].
Object _setOutputAudioId(Pointer<Utf8> deviceId) {
  return () => setOutputAudioId(deviceId.toDartString());
}

/// Subscribes onto the `MediaDevices`'s `devicechange` event.
void _onDeviceChange(Function cb) {
  onDeviceChange(() => cb(null));
}
