import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../util/rust_handles_storage.dart';

import 'enums.dart'
    show
        MediaDirection,
        MediaKind,
        MediaSourceKind,
        MediaStreamTrackState,
        NoiseSuppressionLevel;

export 'enums.dart' show MediaKind, MediaSourceKind, NoiseSuppressionLevel;

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

  /// Indicates whether this [LocalMediaTrack] supports audio processing
  /// functions:
  /// - [LocalMediaTrack.isNoiseSuppressionEnabled]
  /// - [LocalMediaTrack.setNoiseSuppressionEnabled]
  /// - [LocalMediaTrack.getNoiseSuppressionLevel]
  /// - [LocalMediaTrack.setNoiseSuppressionLevel]
  /// - [LocalMediaTrack.isEchoCancellationEnabled]
  /// - [LocalMediaTrack.setEchoCancellationEnabled]
  /// - [LocalMediaTrack.isAutoGainControlEnabled]
  /// - [LocalMediaTrack.setAutoGainControlEnabled]
  /// - [LocalMediaTrack.isHighPassFilterEnabled]
  /// - [LocalMediaTrack.setHighPassFilterEnabled]
  ///
  /// This is only available for local audio tracks on web and desktop. Noise
  /// suppression level and high-pass filter are only available on desktop.
  ///
  /// Additionally, updating echo cancellation, noise suppression and auto gain
  /// control in runtime is unsupported by Chromium-based agents (as of v136).
  /// So, [RoomHandle.setLocalMediaSettings] should be used in this case. Safari
  /// and Firefox are fine.
  bool isAudioProcessingAvailable();

  /// Toggles noise suppression for this [LocalMediaTrack].
  ///
  /// Throws an [InternalException] on unexpected platform error. Not supported
  /// by Chromium-based agents (as of v136).
  Future<void> setNoiseSuppressionEnabled(bool enabled);

  /// Configures a noise suppression level for this [LocalMediaTrack].
  ///
  /// Throws an [InternalException] on unexpected platform error. __Always__
  /// throws on web.
  Future<void> setNoiseSuppressionLevel(NoiseSuppressionLevel level);

  /// Toggles acoustic echo cancellation for this [LocalMediaTrack].
  ///
  /// Throws an [InternalException] on unexpected platform error. Not supported
  /// by Chromium-based agents (as of v136).
  Future<void> setEchoCancellationEnabled(bool enabled);

  /// Toggles automatic gain control for this [LocalMediaTrack].
  ///
  /// Throws an [InternalException] on unexpected platform error. Not supported
  /// by Chromium-based agents (as of v136).
  Future<void> setAutoGainControlEnabled(bool enabled);

  /// Toggles high-pass filter for this [LocalMediaTrack].
  ///
  /// Throws an [InternalException] on unexpected platform error. __Always__
  /// throws on web.
  Future<void> setHighPassFilterEnabled(bool enabled);

  /// Indicates whether noise suppression is enabled for this [LocalMediaTrack].
  ///
  /// Throws an [InternalException] on unexpected platform error.
  Future<bool> isNoiseSuppressionEnabled();

  /// Returns the current configured noise suppression level of this
  /// [LocalMediaTrack].
  ///
  /// Throws an [InternalException] on unexpected platform error. __Always__
  /// throws on web.
  Future<NoiseSuppressionLevel> getNoiseSuppressionLevel();

  /// Indicates whether acoustic echo cancellation is enabled for this
  /// [LocalMediaTrack].
  ///
  /// Throws an [InternalException] on unexpected platform error.
  Future<bool> isEchoCancellationEnabled();

  /// Indicates whether auto gain control is enabled for this [LocalMediaTrack].
  ///
  /// Throws an [InternalException] on unexpected platform error.
  Future<bool> isAutoGainControlEnabled();

  /// Indicates whether high-pass filter is enabled for this [LocalMediaTrack].
  ///
  /// Throws an [InternalException] on unexpected platform error. __Always__
  /// throws on web.
  Future<bool> isHighPassFilterEnabled();
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
