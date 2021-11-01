import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';

/// Registers functions allowing Rust to manage Dart [MediaDevices]s.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaDevices__get_user_media')(
      Pointer.fromFunction<Handle Function(Handle)>(getUserMedia));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaDevices__enumerate_devices')(
      Pointer.fromFunction<Handle Function()>(enumerateDevices));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaDevices__enumerate_devices')(
      Pointer.fromFunction<Handle Function(Handle)>(getDisplayMedia));
}

/// Calls `getUserMedia` and returns created [MediaStreamTrack]s.
Object getUserMedia(MediaStreamConstraints constraints) {
  return () async {
    var res = await navigator.mediaDevices.getUserMedia(
      {
        'audio': constraints.audio,
        'video': constraints.video,
      },
    );
    // ignore: deprecated_member_use
    await res.getMediaTracks();
    return res.getTracks();
  };
}

/// Returns all available devices.
Object enumerateDevices() {
  return () => navigator.mediaDevices.enumerateDevices();
}

/// Calls `getDisplayMedia` and returns created [MediaStreamTrack]s.
Object getDisplayMedia(Map<String, dynamic> constraints) {
  return () => navigator.mediaDevices.getDisplayMedia(constraints);
}
