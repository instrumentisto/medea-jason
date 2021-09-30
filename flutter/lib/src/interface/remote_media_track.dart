import 'package:flutter_webrtc/flutter_webrtc.dart';

import '../util/move_semantic.dart';
import 'track_kinds.dart';

/// Representation of a received remote [`MediaStreamTrack`][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
abstract class RemoteMediaTrack {
  /// Indicates whether this [RemoteMediaTrack] is enabled.
  bool enabled() {
    throw UnimplementedError();
  }

  /// Indicate whether this [RemoteMediaTrack] is muted.
  bool muted() {
    throw UnimplementedError();
  }

  /// Returns this [RemoteMediaTrack]'s kind (audio/video).
  MediaKind kind() {
    throw UnimplementedError();
  }

  /// Returns this [RemoteMediaTrack]'s media source kind (device/display).
  MediaSourceKind mediaSourceKind() {
    throw UnimplementedError();
  }

  /// Returns underlying [MediaStreamTrack] of this [LocalMediaTrack].
  MediaStreamTrack getTrack() {
    throw UnimplementedError();
  }

  /// Sets callback, invoked when this [RemoteMediaTrack] is enabled.
  void onEnabled(void Function() f) {
    throw UnimplementedError();
  }

  /// Sets callback, invoked when this [RemoteMediaTrack] is disabled.
  void onDisabled(void Function() f) {
    throw UnimplementedError();
  }

  /// Sets callback to invoke when this [RemoteMediaTrack] is muted.
  void onMuted(void Function() f) {
    throw UnimplementedError();
  }

  /// Sets callback to invoke when this [RemoteMediaTrack] is unmuted.
  void onUnmuted(void Function() f) {
    throw UnimplementedError();
  }

  /// Sets callback to invoke when this [RemoteMediaTrack] is stopped.
  void onStopped(void Function() f) {
    throw UnimplementedError();
  }

  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free() {
    throw UnimplementedError();
  }
}
