/// Possible error kinds of a [ILocalMediaInitException].
enum LocalMediaInitExceptionKind {
  /// Occurs if the [getUserMedia()][1] request failed.
  ///
  /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
  GetUserMediaFailed,

  /// Occurs if the [getDisplayMedia()][1] request failed.
  ///
  /// [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
  GetDisplayMediaFailed,

  /// Occurs when local track is [`ended`][1] right after [getUserMedia()][2]
  /// or [getDisplayMedia()][3] request.
  ///
  /// [1]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
  /// [2]: https://tinyurl.com/rnxcavf
  /// [3]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
  LocalTrackIsEnded,
}

/// Possible error kinds of a [IRpcClientException].
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

/// Exception thrown when cannot get info of available media devices.
abstract class IEnumerateDevicesException {
  /// Returns error that caused this [IEnumerateDevicesException].
  dynamic cause();

  /// Returns stacktrace of this [IEnumerateDevicesException].
  String trace();
}

/// Exception thrown when a string or some other data doesn't have an expected
/// format and cannot be parsed or processed.
abstract class IFormatException {
  /// Returns describing of the problem.
  String message();
}

/// Jason's internal exception.
///
/// This is either a programmatic error or some unexpected platform component
/// failure that cannot be handled in any way.
abstract class IInternalException {
  /// Returns error message describing the problem.
  String message();

  /// Returns error that caused this [IRpcClientException].
  dynamic cause();

  /// Returns stacktrace of this [IInternalException].
  String trace();

  @override
  String toString() {
    return message();
  }
}

/// Exception thrown when accessing media devices.
abstract class ILocalMediaInitException {
  /// Returns concrete error kind of this [ILocalMediaInitException].
  LocalMediaInitExceptionKind kind();

  /// Returns error message describing the problem.
  String message();

  /// Returns error that caused this [ILocalMediaInitException].
  dynamic cause();

  /// Returns stacktrace of this [ILocalMediaInitException].
  String trace();

  @override
  String toString() {
    return message();
  }
}

/// Errors occurring in `RoomHandle.set_local_media_settings` method.
abstract class IMediaSettingsUpdateException {
  /// Returns error message describing the problem.
  String message();

  /// Returns original error that was encountered while updating local media settings.
  dynamic cause();

  /// Returns whether media settings were successfully rolled back after new
  /// settings application failed.
  bool rolled_back();

  @override
  String toString() {
    return message();
  }
}

/// Exception thrown when the requested media state transition could not be
/// performed.
abstract class IMediaStateTransitionException {
  /// Returns error message describing the problem.
  String message();

  /// Returns stacktrace of this [IMediaStateTransitionException].
  String trace();

  @override
  String toString() {
    return message();
  }
}

/// Exceptions thrown from an RPC client that implements messaging with media
/// server.
abstract class IRpcClientException {
  /// Returns concrete error kind of this [IRpcClientException].
  RpcClientExceptionKind kind();

  /// Returns error message describing the problem.
  String message();

  /// Returns error that caused this [IRpcClientException].
  dynamic cause();

  /// Returns stacktrace of this [IRpcClientException].
  String trace();

  @override
  String toString() {
    return message();
  }
}

/// Error thrown when the operation wasn't allowed by the current state of the
/// object.
abstract class IStateError {
  /// Returns message describing the problem.
  String message();

  /// Returns native stacktrace of this [IStateError].
  String trace();

  @override
  String toString() {
    return message();
  }
}
