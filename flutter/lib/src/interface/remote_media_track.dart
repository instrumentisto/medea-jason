import 'package:flutter_webrtc/flutter_webrtc.dart' as webrtc;

import '/src/util/rust_handles_storage.dart';
import 'track_kinds.dart';

/// Media exchange direction of a [RemoteMediaTrack].
enum TrackMediaDirection {
  /// [RemoteMediaTrack] is enabled on both receiver and sender sides.
  SendRecv,

  /// [RemoteMediaTrack] is enabled on sender side only.
  SendOnly,

  /// [RemoteMediaTrack] is enabled on receiver side only.
  RecvOnly,

  /// [RemoteMediaTrack] is disabled on both sides.
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

  /// Returns the underlying [webrtc.MediaStreamTrack] of this
  /// [RemoteMediaTrack].
  webrtc.MediaStreamTrack getTrack();

  /// Returns the current general [TrackMediaDirection] of this
  /// [RemoteMediaTrack].
  TrackMediaDirection mediaDirection();

  /// Sets callback to invoke when this [RemoteMediaTrack] is muted.
  void onMuted(void Function() f);

  /// Sets callback to invoke when this [RemoteMediaTrack] is unmuted.
  void onUnmuted(void Function() f);

  /// Sets callback to invoke when this [RemoteMediaTrack] is stopped.
  void onStopped(void Function() f);

  /// Sets callback to be invoked whenever this [RemoteMediaTrack]'s general
  /// [TrackMediaDirection] is changed.
  void onMediaDirectionChanged(void Function(TrackMediaDirection) f);
}
