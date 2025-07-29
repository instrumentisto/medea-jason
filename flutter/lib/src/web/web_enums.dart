/// Describes directions that a camera can face, as seen from a user's
/// perspective.
///
/// Representation of a [VideoFacingModeEnum][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-videofacingmodeenum
enum FacingMode {
  /// Facing towards a user (a self-view camera).
  user,

  /// Facing away from a user (viewing an environment).
  environment,

  /// Facing to the left of a user.
  left,

  /// Facing to the right of a user.
  right,
}

/// [MediaDeviceInfo.kind][1] representation.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-kind
enum MediaDeviceKind {
  /// Audio input device (for example, a microphone).
  audioInput,

  /// Video input device (for example, a webcam).
  videoInput,

  /// Audio output device (for example, a pair of headphones).
  audioOutput,
}

/// Media exchange direction of a [`Track`].
enum MediaDirection {
  /// [`Track`] is enabled on both receiver and sender sides.
  sendRecv,

  /// [`Track`] is enabled on sender side only.
  sendOnly,

  /// [`Track`] is enabled on receiver side only.
  recvOnly,

  /// [`Track`] is disabled on both sides.
  inactive,
}

/// [MediaStreamTrack.kind][1] representation.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-kind
enum MediaKind {
  /// Audio track.
  audio,

  /// Video track.
  video,
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
  device,

  /// Media is obtained with screen-capture.
  display,
}

/// Audio processing noise suppression aggressiveness.
enum NoiseSuppressionLevel {
  /// Minimal noise suppression.
  low,

  /// Moderate level of suppression.
  moderate,

  /// Aggressive noise suppression.
  high,

  /// Maximum suppression.
  veryHigh,
}

/// `Connection`'s state kind.
enum MemberConnectionStateKind {
  /// `Connection`'s state is in P2P mode.
  p2p,
}
