import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

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
    clone: _clone,
    readyState: _readyState,
    dispose: _dispose,
    onAudioLevelChanged: _onAudioLevelChanged,
    isOnAudioLevelAvailable: _isOnAudioLevelAvailable,
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

Future<int> Function() _readyState(Object track) {
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
Future<void> Function() _stop(Object track) {
  track as MediaStreamTrack;
  return () => track.stop();
}

/// Sets the provided [OnAudioLevelChangedCallback] for this [MediaStreamTrack].
///
/// It's called for live [MediaStreamTrack]s when their audio level changes.
void _onAudioLevelChanged(Object track, Object f) {
  track as MediaStreamTrack;
  f as Function;
  track.onAudioLevelChanged((lvl) {
    f(lvl);
  });
}

/// Indicates whether a [MediaStreamTrack.onAudioLevelChanged] callback is
/// supported for this [MediaStreamTrack].
bool _isOnAudioLevelAvailable(Object track) {
  track as MediaStreamTrack;
  return track.isOnAudioLevelAvailable();
}

/// Indicates whether the provided [MediaStreamTrack] is enabled.
bool _enabled(Object track) {
  track as MediaStreamTrack;
  return track.isEnabled();
}

/// Clones the provided [MediaStreamTrack] preserving the same media source.
Future<MediaStreamTrack> Function() _clone(Object track) {
  track as MediaStreamTrack;
  return () => track.clone();
}

/// Disposes of this [MediaStreamTrack].
Future<void> Function() _dispose(Object track) {
  track as MediaStreamTrack;
  return () => track.dispose();
}
