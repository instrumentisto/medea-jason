import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
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

/// Returns all the available media devices.
Object _enumerateDevices() {
  return () => navigator.mediaDevices.enumerateDevices();
}

/// Starts capturing the contents of a display and returns the created
/// [MediaStreamTrack]s.
Object _getDisplayMedia(Map<String, dynamic> constraints) {
  return () => navigator.mediaDevices.getDisplayMedia(constraints);
}
