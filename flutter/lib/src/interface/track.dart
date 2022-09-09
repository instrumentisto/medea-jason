import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../util/rust_handles_storage.dart';

/// Abstraction of a handle to an object allocated in the Rust side.
abstract class MediaTrack implements PlatformHandle {
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
