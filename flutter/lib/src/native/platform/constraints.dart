import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';

/// Registers functions allowing Rust to create Dart [MediaStreamConstraints]s.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamConstraints__new')(
      Pointer.fromFunction<Handle Function()>(constructor));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamConstraints__set_audio')(
      Pointer.fromFunction<Void Function(Handle, Handle)>(setAudio));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamConstraints__set_video')(
      Pointer.fromFunction<Void Function(Handle, Handle)>(setVideo));
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
