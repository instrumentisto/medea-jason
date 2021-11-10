import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

/// Registers [MediaStreamTrack] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__id')(
      Pointer.fromFunction<Pointer<Utf8> Function(Handle)>(id));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__device_id')(
      Pointer.fromFunction<Pointer Function(Handle)>(deviceId));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__facing_mode')(
      Pointer.fromFunction<Pointer Function(Handle)>(facingMode));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__kind')(
      Pointer.fromFunction<Int64 Function(Handle)>(kind, 0));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__height')(
      Pointer.fromFunction<Pointer Function(Handle)>(height));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaStreamTrack__width')(
      Pointer.fromFunction<Pointer Function(Handle)>(width));
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
Pointer deviceId(MediaStreamTrack track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  final id = track.id;
  if (id == null) {
    return ForeignValue.none().intoRustOwned();
  } else {
    return ForeignValue.fromString(id).intoRustOwned();
  }
}

/// Returns facingMode of the provided [MediaStreamTrack].
Pointer facingMode(MediaStreamTrack track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  return ForeignValue.fromInt(0).intoRustOwned();
}

/// Returns height of the video of the provided [MediaStreamTrack].
Pointer height(MediaStreamTrack track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  return ForeignValue.fromInt(1600).intoRustOwned();
}

/// Returns width of the video of the provided [MediaStreamTrack].
Pointer width(MediaStreamTrack track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  return ForeignValue.fromInt(1300).intoRustOwned();
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
