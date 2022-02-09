import 'dart:ffi';

import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:flutter_webrtc/src/model/constraints.dart';

import 'media_devices.g.dart' as bridge;

/// Registers functions allowing Rust to operate Dart [MediaDevices].
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    enumerateDevices: Pointer.fromFunction(_enumerateDevices),
    getUserMedia: Pointer.fromFunction(_getUserMedia),
    getDisplayMedia: Pointer.fromFunction(_getDisplayMedia),
  );
}

/// Requests media input access and returns the created [MediaStreamTrack]s.
Object _getUserMedia(Constraints constraints) {
  return () => getUserMedia(constraints);
}

/// Returns all the available media devices.
Object _enumerateDevices() {
  return () => enumerateDevices();
}

/// Starts capturing the contents of a display and returns the created
/// [MediaStreamTrack]s.
Object _getDisplayMedia(Map<String, dynamic> constraints) {
  throw UnimplementedError(
      "getDisplayMedia currently isn't supported by flutter_webrtc");
}
