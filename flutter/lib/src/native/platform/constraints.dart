import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'constraints.g.dart' as bridge;

/// Registers functions allowing Rust to create Dart [MediaStreamConstraints]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    init: Pointer.fromFunction(_new),
    audio: Pointer.fromFunction(_setAudio),
    video: Pointer.fromFunction(_setVideo),
  );
}

/// Returns empty [MediaStreamConstraints].
Object _new() {
  return MediaStreamConstraints();
}

/// Sets `audio` field of the provided [MediaStreamConstraints].
void _setAudio(MediaStreamConstraints cons, Object val) {
  cons.audio = val;
}

/// Sets `video` field of the provided [MediaStreamConstraints].
void _setVideo(MediaStreamConstraints cons, Object val) {
  cons.video = val;
}
