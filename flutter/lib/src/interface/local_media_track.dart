import 'package:flutter_webrtc/flutter_webrtc.dart' as webrtc;

import '/src/util/rust_handles_storage.dart';
import 'track_kinds.dart';

/// Strongly referenced media track received from a
/// [`getUserMedia()`][1]/[`getDisplayMedia()`][2] request.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
abstract class LocalMediaTrack implements PlatformHandle {
  /// Returns the [MediaKind.Audio] if this [LocalMediaTrack] represents an
  /// audio track, or the [MediaKind.Video] if it represents a video track.
  MediaKind kind();

  /// Returns the [MediaSourceKind.Device] if this [LocalMediaTrack] is sourced
  /// from some device (webcam/microphone), or the [MediaSourceKind.Display]
  /// if it's captured via [`MediaDevices.getDisplayMedia()`][1].
  ///
  /// [1]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
  MediaSourceKind mediaSourceKind();

  /// Returns underlying [MediaStreamTrack] of this [LocalMediaTrack].
  webrtc.MediaStreamTrack getTrack();
}
