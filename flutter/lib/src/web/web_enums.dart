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

/// Audio device kind.
enum AudioDeviceKind {
  /// Built-in earpiece speaker.
  earSpeaker,

  /// Built-in loudspeaker.
  speakerphone,

  /// Wired headphones without microphone.
  wiredHeadphones,

  /// Wired headset with a microphone.
  wiredHeadset,

  /// USB headphones without microphone.
  usbHeadphones,

  /// USB headset with a microphone.
  usbHeadset,

  /// Bluetooth headphones profile (A2DP/BLE speaker).
  bluetoothHeadphones,

  /// Bluetooth headset profile suitable for calls (SCO/BLE headset).
  bluetoothHeadset,
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

/// Possible connection states of a `PeerConnection`.
enum PeerConnectionState {
  /// At least one of the connection's [ICE] transports are in the `new` state,
  /// and none of them are in one of the following states: `checking`, `failed`,
  /// or `disconnected`, or all of the connection's transports are in the
  /// `closed` state.
  ///
  /// [ICE]: https://webrtcglossary.com/ice
  new_,

  /// One or more of the [ICE] transports are currently in the process of
  /// establishing a connection; that is, their state is either `checking` or
  /// `connected`, and no transports are in the `failed` state.
  ///
  /// [ICE]: https://webrtcglossary.com/ice
  connecting,

  /// Every [ICE] transport used by the connection is either in use (state
  /// `connected` or `completed`) or is `closed`.
  ///
  /// In addition, at least one transport is either `connected` or `completed`.
  ///
  /// [ICE]: https://webrtcglossary.com/ice
  connected,

  /// At least one of the [ICE] transports for the connection is in the
  /// `disconnected` state and none of the other transports are in the state
  /// `failed` or `checking`.
  ///
  /// It's not a terminal state, and it can go back to [connecting] and then
  /// [connected] on its own.
  ///
  /// [ICE]: https://webrtcglossary.com/ice
  disconnected,

  /// One or more of the [ICE] transports on the connection is in the `failed`
  /// state.
  ///
  /// It's not a terminal state, and it can be fixed with [ICE] restart if
  /// signalling connection is alive.
  ///
  /// [ICE]: https://webrtcglossary.com/ice
  failed,

  /// `PeerConnection` is closed.
  ///
  /// It's a terminal state.
  closed,
}

/// The reason of why a `Room` was closed.
enum RoomCloseKind {
  /// Unexpected client error.
  internalClientError,

  /// Unexpected server error.
  internalServerError,

  /// Room was normally closed by client via `Jason::close_room()`.
  finished,

  /// Connection has been inactive for a while and thus considered idle
  /// by a server.
  idle,

  /// Establishing of connection with a server was rejected on server side.
  ///
  /// Most likely because of incorrect `Member` credentials.
  rejected,

  /// Client was evicted on the server side.
  ///
  /// Usually this means that either `Member` or `Room` was deleted from the
  /// server.
  evicted,
}
