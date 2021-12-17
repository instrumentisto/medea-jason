import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'media_devices.g.dart' as bridge;

/// Registers functions allowing Rust to manage Dart [MediaDevices]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    enumerateDevices: Pointer.fromFunction(_enumerateDevices),
    getUserMedia: Pointer.fromFunction(_getUserMedia),
    getDisplayMedia: Pointer.fromFunction(_getDisplayMedia),
  );
}

/// Calls `getUserMedia` and returns created [MediaStreamTrack]s.
Object _getUserMedia(MediaStreamConstraints constraints) {
  return () async {
    var videoConstraints = {};
    if (constraints.video != null && constraints.video['video'] != null) {
      videoConstraints = constraints.video['video'];
    }
    var res = await navigator.mediaDevices.getUserMedia(
      {
        'audio': constraints.audio,
        'video': videoConstraints,
      },
    );
    // ignore: deprecated_member_use
    await res.getMediaTracks();
    return res.getTracks();
  };
}

/// Returns all available devices.
Object _enumerateDevices() {
  return () => navigator.mediaDevices.enumerateDevices();
}

/// Calls `getDisplayMedia` and returns created [MediaStreamTrack]s.
Object _getDisplayMedia(Map<String, dynamic> constraints) {
  return () => navigator.mediaDevices.getDisplayMedia(constraints);
}
