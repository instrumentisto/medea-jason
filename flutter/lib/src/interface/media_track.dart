import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../util/rust_handles_storage.dart';

/// Representation of a [`MediaStreamTrack.kind`][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-kind
enum MediaKind {
  /// Audio track.
  Audio,

  /// Video track.
  Video,
}

/// Media source type.
enum MediaSourceKind {
  /// Media is sourced from some media device (webcam or microphone).
  Device,

  /// Media is obtained via screen capturing.
  Display,
}

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

/// Abstraction of a handle to an object allocated on the Rust side.
abstract class MediaTrack implements AsyncPlatformHandle {
  /// Returns the [MediaKind.Audio] if this [LocalMediaTrack] represents an
  /// audio track, or the [MediaKind.Video] if it represents a video track.
  MediaKind kind();

  /// Returns the [MediaSourceKind.Device] if this [LocalMediaTrack] is sourced
  /// from some device (webcam/microphone), or the [MediaSourceKind.Display]
  /// if it's captured via [`MediaDevices.getDisplayMedia()`][1].
  ///
  /// [1]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
  MediaSourceKind mediaSourceKind();
}

/// Strongly referenced media track received from a
/// [`getUserMedia()`][1]/[`getDisplayMedia()`][2] request.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
abstract class LocalMediaTrack implements MediaTrack {
  /// Returns the underlying [MediaStreamTrack] of this [LocalMediaTrack].
  webrtc.MediaStreamTrack getTrack();
}

/// Representation of a received remote [`MediaStreamTrack`][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
abstract class RemoteMediaTrack implements MediaTrack {
  /// Indicate whether this [RemoteMediaTrack] is muted.
  bool muted();

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

  /// Returns the underlying [webrtc.MediaStreamTrack] of this
  /// [RemoteMediaTrack].
  webrtc.MediaStreamTrack? getTrack();

  /// Waits and returns the underlying [webrtc.MediaStreamTrack] of this
  /// [RemoteMediaTrack].
  Future<webrtc.MediaStreamTrack> waitTrack();
}
