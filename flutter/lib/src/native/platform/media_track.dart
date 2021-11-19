import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

import 'media_track.g.dart' as bridge;

/// Registers [MediaStreamTrack] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    id: Pointer.fromFunction(_id),
    deviceId: Pointer.fromFunction(_deviceId),
    facingMode: Pointer.fromFunction(_facingMode),
    kind: Pointer.fromFunction(_kind, 0),
    height: Pointer.fromFunction(_height),
    width: Pointer.fromFunction(_width),
    setEnabled: Pointer.fromFunction(_setEnabled),
    enabled: Pointer.fromFunction(_enabled, 0),
    stop: Pointer.fromFunction(_stop),
    onEnded: Pointer.fromFunction(_onEnded),
    readyState: Pointer.fromFunction(_readyState, 0),
  );
}

/// Returns ID of the provided [MediaStreamTrack].
Pointer<Utf8> _id(MediaStreamTrack track) {
  return track.id!.toNativeUtf8();
}

/// Returns kind of the provided [MediaStreamTrack].
int _kind(MediaStreamTrack track) {
  if (track.kind == 'audio') {
    return 0;
  } else {
    return 1;
  }
}

/// Subscribes on the [MediaStreamTrack.onEnded] of the provided
/// [MediaStreamTrack].
void _onEnded(MediaStreamTrack track, Function f) {
  track.onEnded = () {
    f();
  };
}

/// Returns device ID of the provided [MediaStreamTrack].
Pointer<Utf8> _deviceId(MediaStreamTrack track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  return _id(track);
}

int _readyState(MediaStreamTrack track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  throw UnimplementedError();
}

/// Returns facingMode of the provided [MediaStreamTrack].
Pointer _facingMode(MediaStreamTrack track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  return ForeignValue.fromInt(0).intoRustOwned();
}

/// Returns height of the video of the provided [MediaStreamTrack].
Pointer _height(MediaStreamTrack track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  return ForeignValue.fromInt(1600).intoRustOwned();
}

/// Returns width of the video of the provided [MediaStreamTrack].
Pointer _width(MediaStreamTrack track) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  return ForeignValue.fromInt(1300).intoRustOwned();
}

/// Sets [MediaStreamTrack.enabled] state of the provided [MediaStreamTrack].
void _setEnabled(MediaStreamTrack track, int enabled) {
  track.enabled = enabled == 1;
}

/// Stops provided [MediaStreamTrack].
void _stop(MediaStreamTrack track) {
  track.stop();
}

/// Returns `1` if the provided [MediaStreamTrack] is enabled and `0` otherwise.
int _enabled(MediaStreamTrack track) {
  return track.enabled ? 1 : 0;
}
