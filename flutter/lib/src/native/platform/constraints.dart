import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'constraints.g.dart' as bridge;

/// Registers functions allowing Rust to create Dart [MediaStreamConstraints]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    init: Pointer.fromFunction(constructor),
    audio: Pointer.fromFunction(setAudio),
    video: Pointer.fromFunction(setVideo),
  );
}

/// Returns empty [MediaStreamConstraints].
Object constructor() {
  return MediaStreamConstraints();
}

/// Sets `audio` field of the provided [MediaStreamConstraints].
void setAudio(MediaStreamConstraints cons, Object val) {
  cons.audio = val;
}

/// Sets `video` field of the provided [MediaStreamConstraints].
void setVideo(MediaStreamConstraints cons, Object val) {
  cons.video = val;
}
