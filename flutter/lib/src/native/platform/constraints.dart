import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';

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

Object constructor() {
  return MediaStreamConstraints();
}

void setAudio(MediaStreamConstraints cons, Object val) {
  cons.audio = val;
}

void setVideo(MediaStreamConstraints cons, Object val) {
  cons.video = val;
}
