import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../util/rust_handles_storage.dart';

import 'enums.dart'
    show MediaDirection, MediaKind, MediaSourceKind, MediaStreamTrackState;

export 'enums.dart' show MediaKind, MediaSourceKind;

typedef TrackMediaDirection = MediaDirection;

/// Representation of the `onEnded` callback.
typedef OnEndedCallback = void Function();

/// Representation of an `onAudioLevelChanged` callback.
///
/// The provided values will be in [0; 100] range.
typedef OnAudioLevelChangedCallback = void Function(int);

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

  /// Returns the underlying [MediaStreamTrack] of this [LocalMediaTrack].
  webrtc.MediaStreamTrack getTrack();
}

/// Strongly referenced media track received from a
/// [`getUserMedia()`][1]/[`getDisplayMedia()`][2] request.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
abstract class LocalMediaTrack implements MediaTrack {
  /// Sets a callback to invoke when this [LocalMediaTrack] is ended.
  ///
  /// This only works on Web.
  void onEnded(OnEndedCallback f);

  /// Indicates whether an [OnAudioLevelChangedCallback] is supported for this
  /// [MediaTrack].
  bool isOnAudioLevelAvailable();

  /// Sets the provided [OnAudioLevelChangedCallback] for this [MediaTrack].
  ///
  /// It's called for live [MediaTrack]s when their audio level changes.
  ///
  /// Throws an [InternalException] on unexpected platform error.
  void onAudioLevelChanged(OnAudioLevelChangedCallback f);

  /// Returns a [MediaStreamTrackState.live] if this [LocalMediaTrack] is
  /// active, or a [MediaStreamTrackState.ended] if it has ended.
  Future<MediaStreamTrackState> state();
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
}
