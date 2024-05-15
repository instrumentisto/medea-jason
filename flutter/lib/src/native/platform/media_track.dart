import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

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
    enabled: Pointer.fromFunction(_enabled, false),
    stop: Pointer.fromFunction(_stop),
    onEnded: Pointer.fromFunction(_onEnded),
    clone: Pointer.fromFunction(_clone),
    readyState: Pointer.fromFunction(_readyState),
    dispose: Pointer.fromFunction(_dispose),
  );
}

/// Returns ID of the provided [MediaStreamTrack].
Pointer<Utf8> _id(Object track) {
  track as MediaStreamTrack;
  return track.id().toNativeUtf8();
}

/// Returns kind of the provided [MediaStreamTrack].
int _kind(Object track) {
  track as MediaStreamTrack;
  if (track.kind() == MediaKind.audio) {
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

  track.onEnded(() {
    f(null);
  });
}

/// Returns device ID of the provided [MediaStreamTrack].
Pointer<Utf8> _deviceId(Object track) {
  track as MediaStreamTrack;
  return track.deviceId().toNativeUtf8();
}

Object _readyState(Object track) {
  track as MediaStreamTrack;
  return () => track.state().then((s) => s.index);
}

/// Returns facingMode of the provided [MediaStreamTrack].
Pointer _facingMode(Object track) {
  track as MediaStreamTrack;
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
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
  track as MediaStreamTrack;
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  return ForeignValue.fromInt(1300).intoRustOwned();
}

/// Sets [MediaStreamTrack.enabled] state of the provided [MediaStreamTrack].
void _setEnabled(Object track, bool enabled) {
  track = track as MediaStreamTrack;
  track.setEnabled(enabled);
}

/// Stops the provided [MediaStreamTrack].
Object _stop(Object track) {
  track as MediaStreamTrack;
  return () => track.stop();
}

/// Indicates whether the provided [MediaStreamTrack] is enabled.
bool _enabled(Object track) {
  track as MediaStreamTrack;
  return track.isEnabled();
}

/// Clones the provided [MediaStreamTrack] preserving the same media source.
Object _clone(Object track) {
  track as MediaStreamTrack;
  return () => track.clone();
}

/// Disposes of this [MediaStreamTrack].
Object _dispose(Object track) {
  track as MediaStreamTrack;
  return () => track.dispose();
}
