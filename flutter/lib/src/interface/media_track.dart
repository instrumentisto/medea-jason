import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../util/rust_handles_storage.dart';
import 'enums.dart' show MediaKind, MediaSourceKind, MediaDirection;

export 'enums.dart' show MediaKind, MediaSourceKind;

typedef TrackMediaDirection = MediaDirection;

/// Representation of the `onEnded` callback.
typedef OnEndedCallback = void Function();

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
  /// Sets callback to invoke when this [LocalMediaTrack] is ended.
  void onEnded(OnEndedCallback f);

  /// Returns a [webrtc.MediaStreamTrackState.live] if this [LocalMediaTrack] is
  /// active, or a [webrtc.MediaStreamTrackState.ended] if the track has ended.
  Future<webrtc.MediaStreamTrackState> state();
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

// Target dart2js failed: Exception: Warning: The 'dart2js' entrypoint script is deprecated, please use 'dart compile js' instead.
// ../../CLionProjects3/medea-jason/flutter/lib/src/interface/media_track.dart:41:10:
// Error: Type 'webrtc.MediaStreamTrackState' not found.
// Future<webrtc.MediaStreamTrackState> state();
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// ../../CLionProjects3/medea-jason/flutter/lib/src/web/local_media_track.dart:44:10:
// Error: Type 'webrtc.MediaStreamTrackState' not found.
// Future<webrtc.MediaStreamTrackState> state() async {
//   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//   lib/ui/page/home/page/chat/view.dart:1595:47:
//   Warning: Operand of null-aware operation '!' has type 'String' which excludes null.
//   child: Text(text!, style: style.boldBody),
//   ^
//   ../../CLionProjects3/medea-jason/flutter/lib/src/web/local_media_track.dart:46:19:
//   Error: Undefined name 'MediaStreamTrackState'.
//   return webrtc.MediaStreamTrackState.values[index];
//   ^^^^^^^^^^^^^^^^^^^^^
//   Error: Compilation failed.
