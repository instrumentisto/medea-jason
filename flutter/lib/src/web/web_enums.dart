/// Describes directions that a camera can face, as seen from a user's
/// perspective.
///
/// Representation of a [VideoFacingModeEnum][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-videofacingmodeenum
enum FacingMode {
  /// Facing towards a user (a self-view camera).
  User,

  /// Facing away from a user (viewing an environment).
  Environment,

  /// Facing to the left of a user.
  Left,

  /// Facing to the right of a user.
  Right,
}

/// [MediaDeviceInfo.kind][1] representation.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-kind
enum MediaDeviceKind {
  /// Audio input device (for example, a microphone).
  AudioInput,

  /// Video input device (for example, a webcam).
  VideoInput,

  /// Audio output device (for example, a pair of headphones).
  AudioOutput,
}

/// Media exchange direction of a [`Track`].
enum MediaDirection {
  /// [`Track`] is enabled on both receiver and sender sides.
  SendRecv,

  /// [`Track`] is enabled on sender side only.
  SendOnly,

  /// [`Track`] is enabled on receiver side only.
  RecvOnly,

  /// [`Track`] is disabled on both sides.
  Inactive,
}

/// [MediaStreamTrack.kind][1] representation.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-kind
enum MediaKind {
  /// Audio track.
  Audio,

  /// Video track.
  Video,
}

/// Representation of a `MediaStreamTrack` readiness.
enum MediaStreamTrackState {
  /// Indicates that an input is connected and does its best-effort in the
  /// providing real-time data.
  live,

  /// Indicates that the input is not giving any more data and will never
  /// provide a new data.
  ended,
}

/// Media source type.
enum MediaSourceKind {
  /// Media is sourced from some media device (webcam or microphone).
  Device,

  /// Media is obtained with screen-capture.
  Display,
}
