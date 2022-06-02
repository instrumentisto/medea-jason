/// Possible error kinds of a [LocalMediaInitException].
enum LocalMediaInitExceptionKind {
  /// Occurs if the [getUserMedia()][1] request failed.
  ///
  /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
  GetUserMediaFailed,

  /// Occurs if the [getUserMedia()][1] request failed on getting audio.
  ///
  /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
  GetUserMediaAudioFailed,

  /// Occurs if the [getUserMedia()][1] request failed on getting video.
  ///
  /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
  GetUserMediaVideoFailed,

  /// Occurs if the [getDisplayMedia()][1] request failed.
  ///
  /// [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
  GetDisplayMediaFailed,

  /// Occurs when local track is [ended][1] right after [getUserMedia()][2]
  /// or [getDisplayMedia()][3] request.
  ///
  /// [1]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
  /// [2]: https://tinyurl.com/rnxcavf
  /// [3]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
  LocalTrackIsEnded,
}

/// Possible error kinds of a [RpcClientException].
enum RpcClientExceptionKind {
  /// Connection with a server was lost.
  ///
  /// This usually means that some transport error occurred, so a client can
  /// continue performing reconnecting attempts.
  ConnectionLost,

  /// Could not authorize an RPC session.
  ///
  /// This usually means that authentication data a client provides is
  /// obsolete.
  AuthorizationFailed,

  /// RPC session has been finished. This is a terminal state.
  SessionFinished,
}

/// Kind of a [MediaStateTransitionException].
enum MediaStateTransitionExceptionKind {
  /// Media state of a `Sender` transits to an opposite of the requested one.
  OppositeState,

  /// Requested state transition is not allowed by `Sender`'s settings.
  ProhibitedState,
}

/// Exception thrown when cannot get info of available media devices.
abstract class EnumerateDevicesException {
  /// Returns error that caused this [EnumerateDevicesException].
  dynamic cause();

  /// Returns stacktrace of this [EnumerateDevicesException].
  String trace();
}

/// Exception thrown when cannot switch output audio device ID.
abstract class InvalidOutputAudioDeviceIdException {
  /// Returns stacktrace of this [InvalidOutputAudioDeviceIdException].
  String trace();
}

/// Exception thrown when cannot interact with microphone volume.
abstract class MicVolumeException {
  /// Returns error that caused this [MicVolumeException].
  dynamic cause();

  /// Returns stacktrace of this [MicVolumeException].
  String trace();
}

/// Jason's internal exception.
///
/// This is either a programmatic error or some unexpected platform component
/// failure that cannot be handled in any way.
abstract class InternalException {
  /// Returns error message describing the problem.
  String message();

  /// Returns error that caused this [RpcClientException].
  dynamic cause();

  /// Returns stacktrace of this [InternalException].
  String trace();

  @override
  String toString() {
    return message();
  }
}

/// Exception thrown when accessing media devices.
abstract class LocalMediaInitException {
  /// Returns concrete error kind of this [LocalMediaInitException].
  LocalMediaInitExceptionKind kind();

  /// Returns error message describing the problem.
  String message();

  /// Returns error that caused this [LocalMediaInitException].
  dynamic cause();

  /// Returns stacktrace of this [LocalMediaInitException].
  String trace();

  @override
  String toString() {
    return message();
  }
}

/// Errors occurring in `RoomHandle.set_local_media_settings` method.
abstract class MediaSettingsUpdateException {
  /// Returns error message describing the problem.
  String message();

  /// Returns original error that was encountered while updating local media
  /// settings.
  dynamic cause();

  /// Indicates whether media settings were successfully rolled back after new
  /// settings application failed.
  bool rolledBack();

  @override
  String toString() {
    return message();
  }
}

/// Exception thrown when the requested media state transition could not be
/// performed.
abstract class MediaStateTransitionException {
  /// Returns error message describing the problem.
  String message();

  /// Returns stacktrace of this [MediaStateTransitionException].
  String trace();

  /// Returns concrete error kind of this [MediaStateTransitionException].
  MediaStateTransitionExceptionKind kind();

  @override
  String toString() {
    return message();
  }
}

/// Exceptions thrown from an RPC client that implements messaging with media
/// server.
abstract class RpcClientException {
  /// Returns concrete error kind of this [RpcClientException].
  RpcClientExceptionKind kind();

  /// Returns error message describing the problem.
  String message();

  /// Returns error that caused this [RpcClientException].
  dynamic cause();

  /// Returns stacktrace of this [RpcClientException].
  String trace();

  @override
  String toString() {
    return message();
  }
}
