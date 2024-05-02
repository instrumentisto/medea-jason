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
    onAudioLevelChanged: Pointer.fromFunction(_onAudioLevelChanged),
    isOnAudioLevelAvailable:
        Pointer.fromFunction(_isOnAudioLevelAvailable, false),
  );
}

/// Returns ID of the provided [MediaStreamTrack].
Pointer<Utf8> _id(MediaStreamTrack track) {
  return track.id().toNativeUtf8();
}

/// Returns kind of the provided [MediaStreamTrack].
int _kind(MediaStreamTrack track) {
  if (track.kind() == MediaKind.audio) {
    return 0;
  } else {
    return 1;
  }
}

/// Subscribes on the [MediaStreamTrack.onEnded] of the provided
/// [MediaStreamTrack].
void _onEnded(MediaStreamTrack track, Function f) {
  track.onEnded(() {
    f(null);
  });
}

/// Returns device ID of the provided [MediaStreamTrack].
Pointer<Utf8> _deviceId(MediaStreamTrack track) {
  return track.deviceId().toNativeUtf8();
}

Object _readyState(MediaStreamTrack track) {
  return () => track.state().then((s) => s.index);
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
void _setEnabled(MediaStreamTrack track, bool enabled) {
  track.setEnabled(enabled);
}

/// Stops the provided [MediaStreamTrack].
Object _stop(MediaStreamTrack track) {
  return () => track.stop();
}

/// Sets the provided [OnAudioLevelChangedCallback] for this [MediaStreamTrack].
///
/// It's called for live tracks when audio level of this track changes.
void _onAudioLevelChanged(
    MediaStreamTrack track, OnAudioLevelChangedCallback cb) {
  track.onAudioLevelChanged(cb);
}

/// Indicates whether [MediaStreamTrack.onAudioLevelChanged] callback is
/// supported for this [MediaStreamTrack].
bool _isOnAudioLevelAvailable(MediaStreamTrack track) {
  return track.isOnAudioLevelAvailable();
}

/// Indicates whether the provided [MediaStreamTrack] is enabled.
bool _enabled(MediaStreamTrack track) {
  return track.isEnabled();
}

/// Clones the provided [MediaStreamTrack] preserving the same media source.
Object _clone(MediaStreamTrack track) {
  return () => track.clone();
}

/// Disposes of this [MediaStreamTrack].
Object _dispose(MediaStreamTrack track) {
  return () => track.dispose();
}
