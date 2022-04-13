import 'package:flutter_webrtc/flutter_webrtc.dart';

import '/src/util/rust_handles_storage.dart';
import 'track_kinds.dart';

/// Media exchange direction of the `Track`.
enum TrackMediaDirection {
  /// `Track` is enabled on recv and send sides.
  SendRecv,

  /// `Track` is enabled on send side.
  SendOnly,

  /// `Track` is enabled on recv side.
  RecvOnly,

  /// `Track` is disabled on both sides.
  Inactive,
}

/// Representation of a received remote [`MediaStreamTrack`][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
abstract class RemoteMediaTrack implements PlatformHandle {
  /// Indicates whether this [RemoteMediaTrack] is enabled.
  bool enabled();

  /// Indicate whether this [RemoteMediaTrack] is muted.
  bool muted();

  /// Returns this [RemoteMediaTrack]'s kind (audio/video).
  MediaKind kind();

  /// Returns this [RemoteMediaTrack]'s media source kind (device/display).
  MediaSourceKind mediaSourceKind();

  /// Returns underlying [MediaStreamTrack] of this [LocalMediaTrack].
  MediaStreamTrack getTrack();

  /// Sets callback, invoked when this [RemoteMediaTrack] is enabled.
  void onEnabled(void Function() f);

  /// Sets callback, invoked when this [RemoteMediaTrack] is disabled.
  void onDisabled(void Function() f);

  /// Sets callback to invoke when this [RemoteMediaTrack] is muted.
  void onMuted(void Function() f);

  /// Sets callback to invoke when this [RemoteMediaTrack] is unmuted.
  void onUnmuted(void Function() f);

  /// Sets callback to invoke when this [RemoteMediaTrack] is stopped.
  void onStopped(void Function() f);

  /// Sets callback to invoke when this [RemoteMediaTrack]'s general media
  /// exchange direction changes.
  void onMediaDirectionChanged(void Function(TrackMediaDirection) f);
}
