import 'package:flutter_webrtc/flutter_webrtc.dart';
import '../ffi/foreign_value.dart';
import 'dart:ffi';
import 'package:ffi/ffi.dart';

void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__id')(
      Pointer.fromFunction<Pointer<Utf8> Function(Handle)>(id));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__device_id')(
      Pointer.fromFunction<Pointer<Utf8> Function(Handle)>(deviceId));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__facing_mode')(
      Pointer.fromFunction<Pointer<Utf8> Function(Handle)>(facingMode));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__kind')(
      Pointer.fromFunction<Int32 Function(Handle)>(kind, 0));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__height')(
      Pointer.fromFunction<Int32 Function(Handle)>(height, 0));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__width')(
      Pointer.fromFunction<Int32 Function(Handle)>(width, 0));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__set_enabled')(
      Pointer.fromFunction<Void Function(Handle, Int8)>(setEnabled));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__enabled')(
      Pointer.fromFunction<Int8 Function(Handle)>(enabled, 0));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__stop')(
      Pointer.fromFunction<Void Function(Handle)>(stop));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__on_ended')(
      Pointer.fromFunction<Void Function(Handle, Handle)>(onEnded));
}

Pointer<Utf8> id(Object track) {
    track = track as MediaStreamTrack;
    return track.id!.toNativeUtf8();
}

int kind(MediaStreamTrack track) {
  if (track.kind == 'audio') {
    return 0;
  } else {
    return 1;
  }
}

void onEnded(Object track, Object f) {
  if (track is MediaStreamTrack) {
    if (f is Function) {
      track.onEnded = () {
        f();
      };
    }
  }
}

Pointer<Utf8> deviceId(MediaStreamTrack track) {
  return id(track);
}

Pointer<Utf8> facingMode(MediaStreamTrack track) {
  // TODO: remove this dummy implementation when flutter_webrtc will be reworked
  return 'user'.toNativeUtf8();
}

int height(MediaStreamTrack track) {
  // TODO: remove this dummy implementation when flutter_webrtc will be reworked
  return 1600;
}

int width(MediaStreamTrack track) {
  // TODO: remove this dummy implementation when flutter_webrtc will be reworked
  return 1300;
}

void setEnabled(MediaStreamTrack track, int enabled) {
  track.enabled = enabled == 1;
}

void stop(MediaStreamTrack track) {
  track.stop();
}

int enabled(MediaStreamTrack track) {
  return track.enabled ? 1 : 0;
}
