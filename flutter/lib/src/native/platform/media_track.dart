import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

import 'media_track.g.dart' as bridge;

/// Registers [MediaStreamTrack] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    id: id,
    deviceId: deviceId,
    facingMode: facingMode,
    kind: kind,
    height: height,
    width: width,
    setEnabled: setEnabled,
    enabled: enabled,
    stop: stop,
    onEnded: onEnded,
    readyState: readyState,
  );
}

/// Returns ID of the provided [MediaStreamTrack].
Pointer<Utf8> id(Object track) {
  track as MediaStreamTrack;
  return track.id!.toNativeUtf8();
}

/// Returns kind of the provided [MediaStreamTrack].
int kind(Object track) {
  track as MediaStreamTrack;
  if (track.kind == 'audio') {
    return 0;
  } else {
    return 1;
  }
}

/// Subscribes on the [MediaStreamTrack.onEnded] of the provided
/// [MediaStreamTrack].
void onEnded(Object track, Object f) {
  track as MediaStreamTrack;
  f as Function;
  track.onEnded = () {
    f();
  };
}

/// Returns device ID of the provided [MediaStreamTrack].
Pointer<Utf8> deviceId(Object track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  track as MediaStreamTrack;
  return id(track);
}

int readyState(Object track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  track as MediaStreamTrack;
  throw UnimplementedError();
}

/// Returns facingMode of the provided [MediaStreamTrack].
Pointer facingMode(Object track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  track as MediaStreamTrack;
  return ForeignValue.fromInt(0).intoRustOwned();
}

/// Returns height of the video of the provided [MediaStreamTrack].
Pointer height(Object track) {
  track as MediaStreamTrack;
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  return ForeignValue.fromInt(1600).intoRustOwned();
}

/// Returns width of the video of the provided [MediaStreamTrack].
Pointer width(Object track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  track as MediaStreamTrack;
  return ForeignValue.fromInt(1300).intoRustOwned();
}

/// Sets [MediaStreamTrack.enabled] state of the provided [MediaStreamTrack].
void setEnabled(Object track, int enabled) {
  track as MediaStreamTrack;
  track.enabled = enabled == 1;
}

/// Stops provided [MediaStreamTrack].
void stop(Object track) {
  track as MediaStreamTrack;
  track.stop();
}

/// Returns `1` if the provided [MediaStreamTrack] is enabled and `0` otherwise.
int enabled(Object track) {
  track as MediaStreamTrack;
  return track.enabled ? 1 : 0;
}
