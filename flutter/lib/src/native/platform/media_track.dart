import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'package:ffi/ffi.dart';

/// Registers [MediaStreamTrack] related functions in Rust.
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

/// Returns ID of the provided [MediaStreamTrack].
Pointer<Utf8> id(MediaStreamTrack track) {
  return track.id!.toNativeUtf8();
}

/// Returns kind of the provided [MediaStreamTrack].
int kind(MediaStreamTrack track) {
  if (track.kind == 'audio') {
    return 0;
  } else {
    return 1;
  }
}

/// Subscribes on the [MediaStreamTrack.onEnded] of the provided [MediaStreamTrack].
void onEnded(MediaStreamTrack track, Function f) {
    track.onEnded = () {
      f();
    };
}

/// Returns device ID of the provided [MediaStreamTrack].
Pointer<Utf8> deviceId(MediaStreamTrack track) {
  // TODO: add correct implementation when flutter_webrtc will be reworked.
  return id(track);
}

/// Returns facingMode of the provided [MediaStreamTrack].
Pointer<Utf8> facingMode(MediaStreamTrack track) {
  // TODO: remove this dummy implementation when flutter_webrtc will be reworked
  return 'user'.toNativeUtf8();
}

/// Returns height of the video of the provided [MediaStreamTrack].
int height(MediaStreamTrack track) {
  // TODO: remove this dummy implementation when flutter_webrtc will be reworked
  return 1600;
}

/// Returns width of the video of the provided [MediaStreamTrack].
int width(MediaStreamTrack track) {
  // TODO: remove this dummy implementation when flutter_webrtc will be reworked
  return 1300;
}

/// Sets [MediaStreamTrack.enabled] state of the provided [MediaStreamTrack].
void setEnabled(MediaStreamTrack track, int enabled) {
  track.enabled = enabled == 1;
}

/// Stops provided [MediaStreamTrack].
void stop(MediaStreamTrack track) {
  track.stop();
}

/// Returns `1` if the provided [MediaStreamTrack] is enabled and `0` otherwise.
int enabled(MediaStreamTrack track) {
  return track.enabled ? 1 : 0;
}
