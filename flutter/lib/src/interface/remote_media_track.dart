import 'package:flutter_webrtc/flutter_webrtc.dart' as webrtc;

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
  /// Indicate whether this [RemoteMediaTrack] is muted.
  bool muted();

  /// Returns this [RemoteMediaTrack]'s kind (audio/video).
  MediaKind kind();

  /// Returns this [RemoteMediaTrack]'s media source kind (device/display).
  MediaSourceKind mediaSourceKind();

  /// Returns underlying [webrtc.MediaStreamTrack] of this [RemoteMediaTrack].
  webrtc.MediaStreamTrack getTrack();

  /// Returns current general media exchange direction of this [RemoteMediaTrack].
  TrackMediaDirection mediaDirection();

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
