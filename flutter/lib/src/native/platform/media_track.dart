import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

import 'media_track.g.dart' as bridge;

/// Registers [MediaStreamTrack] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    id: _id,
    deviceId: _deviceId,
    facingMode: _facingMode,
    kind: _kind,
    height: _height,
    width: _width,
    setEnabled: _setEnabled,
    enabled: _enabled,
    stop: _stop,
    onEnded: _onEnded,
    readyState: _readyState,
  );
}

/// Returns ID of the provided [MediaStreamTrack].
Pointer<Utf8> _id(Object track) {
  track as MediaStreamTrack;
  return track.id!.toNativeUtf8();
}

/// Returns kind of the provided [MediaStreamTrack].
int _kind(Object track) {
  track as MediaStreamTrack;
  if (track.kind == 'audio') {
    return 0;
  } else {
    return 1;
  }
}

/// Subscribes on the [MediaStreamTrack.onEnded] of the provided
/// [MediaStreamTrack].
void _onEnded(Object track, Object f) {
  track as MediaStreamTrack;
  f as Function;
  track.onEnded = () {
    f();
  };
}

/// Returns device ID of the provided [MediaStreamTrack].
Pointer<Utf8> _deviceId(Object track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  track as MediaStreamTrack;
  return _id(track);
}

int _readyState(Object track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  track as MediaStreamTrack;
  throw UnimplementedError();
}

/// Returns facingMode of the provided [MediaStreamTrack].
Pointer _facingMode(Object track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  track as MediaStreamTrack;
  return ForeignValue.fromInt(0).intoRustOwned();
}

/// Returns height of the video of the provided [MediaStreamTrack].
Pointer _height(Object track) {
  track as MediaStreamTrack;
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  return ForeignValue.fromInt(1600).intoRustOwned();
}

/// Returns width of the video of the provided [MediaStreamTrack].
Pointer _width(Object track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  track as MediaStreamTrack;
  return ForeignValue.fromInt(1300).intoRustOwned();
}

/// Sets [MediaStreamTrack.enabled] state of the provided [MediaStreamTrack].
void _setEnabled(Object track, int enabled) {
  track as MediaStreamTrack;
  track.enabled = enabled == 1;
}

/// Stops provided [MediaStreamTrack].
void _stop(Object track) {
  track as MediaStreamTrack;
  track.stop();
}

/// Returns `1` if the provided [MediaStreamTrack] is enabled and `0` otherwise.
int _enabled(Object track) {
  track as MediaStreamTrack;
  return track.enabled ? 1 : 0;
}
