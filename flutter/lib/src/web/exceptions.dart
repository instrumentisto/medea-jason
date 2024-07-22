import '../interface/exceptions.dart';
import 'jason_wasm.dart' as wasm;

/// Returns name of the provided [wasm] exception.
///
/// Returns `null` in case if the provided exception is not from Jason.
String? _getName(dynamic e) {
  try {
    return e.name;
  } catch (e) {
    print('_getName failed: $e');
    return null;
  }
}

/// Converts the provided [wasm] exception into the Dart exception.
dynamic convertException(dynamic e) {
  var name = _getName(e);
  if (name == null) {
    return e;
  } else if (name == 'FormatException') {
    e as wasm.FormatException;
    var message = e.message();
    e.free();
    return FormatException(message);
  } else if (name == 'StateError') {
    e as wasm.StateError;
    var message = e.message();
    e.free();
    return StateError(message);
  } else if (name == 'EnumerateDevicesException') {
    return WebEnumerateDevicesException(e as wasm.EnumerateDevicesException);
  } else if (name == 'InternalException') {
    return WebInternalException(e as wasm.InternalException);
  } else if (name == 'LocalMediaInitException') {
    return WebLocalMediaInitException(e as wasm.LocalMediaInitException);
  } else if (name == 'MediaSettingsUpdateException') {
    return WebMediaSettingsUpdateException(
        e as wasm.MediaSettingsUpdateException);
  } else if (name == 'MediaStateTransitionException') {
    return WebMediaStateTransitionException(
        e as wasm.MediaStateTransitionException);
  } else if (name == 'RpcClientException') {
    return WebRpcClientException(e as wasm.RpcClientException);
  } else {
    return e;
  }
}

/// Wraps provided [Function] to the try/catch block and wraps [wasm] exception
/// to the Dart wrapper.
T fallibleFunction<T>(T Function() f) {
  try {
    return f();
  } catch (e) {
    throw convertException(e);
  }
}

/// Wraps provided [Future] to the try/catch block and wraps [wasm] exception to
/// the Dart wrapper.
Future<T> fallibleFuture<T>(Future<T> f) async {
  try {
    return await f;
  } catch (e) {
    throw convertException(e);
  }
}

/// Exception thrown when cannot get info of available media devices.
class WebEnumerateDevicesException extends EnumerateDevicesException {
  late dynamic _cause;
  late String _trace;

  WebEnumerateDevicesException(wasm.EnumerateDevicesException e) {
    _cause = e.cause();
    _trace = e.trace();
    e.free();
  }

  /// Returns error that caused this [EnumerateDevicesException].
  @override
  dynamic cause() {
    return _cause;
  }

  /// Returns stacktrace of this [EnumerateDevicesException].
  @override
  String trace() {
    return _trace;
  }
}

/// Jason's internal exception.
///
/// This is either a programmatic error or some unexpected platform component
/// failure that cannot be handled in any way.
class WebInternalException extends InternalException {
  late String _message;
  late dynamic _cause;
  late String _trace;

  WebInternalException(wasm.InternalException e) {
    _message = e.message();
    _cause = e.cause();
    _trace = e.trace();
    e.free();
  }

  /// Returns error message describing the problem.
  @override
  String message() {
    return _message;
  }

  /// Returns error that caused this [RpcClientException].
  @override
  dynamic cause() {
    return _cause;
  }

  /// Returns stacktrace of this [InternalException].
  @override
  String trace() {
    return _trace;
  }
}

/// Exception thrown when accessing media devices.
class WebLocalMediaInitException extends LocalMediaInitException {
  late LocalMediaInitExceptionKind _kind;
  late String _message;
  late dynamic _cause;
  late String _trace;

  WebLocalMediaInitException(wasm.LocalMediaInitException e) {
    _kind = LocalMediaInitExceptionKind.values[e.kind().toInt()];
    _message = e.message();
    _cause = e.cause();
    _trace = e.trace();
    e.free();
  }

  /// Returns concrete error kind of this [LocalMediaInitException].
  @override
  LocalMediaInitExceptionKind kind() {
    return _kind;
  }

  /// Returns error message describing the problem.
  @override
  String message() {
    return _message;
  }

  /// Returns error that caused this [LocalMediaInitException].
  @override
  dynamic cause() {
    return _cause;
  }

  /// Returns stacktrace of this [LocalMediaInitException].
  @override
  String trace() {
    return _trace;
  }
}

/// Errors occurring in `RoomHandle::set_local_media_settings()` method.
class WebMediaSettingsUpdateException extends MediaSettingsUpdateException {
  late String _message;
  late dynamic _cause;
  late bool _rolledBack;

  WebMediaSettingsUpdateException(wasm.MediaSettingsUpdateException e) {
    _message = e.message();
    _cause = e.cause();
    _rolledBack = e.rolled_back();
    e.free();
  }

  /// Returns error message describing the problem.
  @override
  String message() {
    return _message;
  }

  /// Returns original error that was encountered while updating local media settings.
  @override
  dynamic cause() {
    return _cause;
  }

  /// Indicates whether media settings were successfully rolled back after new
  /// settings application failed.
  @override
  bool rolledBack() {
    return _rolledBack;
  }
}

/// Exception thrown when the requested media state transition could not be
/// performed.
class WebMediaStateTransitionException extends MediaStateTransitionException {
  late String _message;
  late String _trace;
  late MediaStateTransitionExceptionKind _kind;

  WebMediaStateTransitionException(wasm.MediaStateTransitionException e) {
    _message = e.message();
    _trace = e.trace();
    _kind = MediaStateTransitionExceptionKind.values[e.kind().toInt()];
    e.free();
  }

  /// Returns error message describing the problem.
  @override
  String message() {
    return _message;
  }

  /// Returns stacktrace of this [MediaStateTransitionException].
  @override
  String trace() {
    return _trace;
  }

  /// Returns concrete error kind of this [MediaStateTransitionException].
  @override
  MediaStateTransitionExceptionKind kind() {
    return _kind;
  }
}

/// Exceptions thrown from an RPC client that implements messaging with a media
/// server.
class WebRpcClientException extends RpcClientException {
  late RpcClientExceptionKind _kind;
  late String _message;
  late dynamic _cause;
  late String _trace;

  WebRpcClientException(wasm.RpcClientException e) {
    _kind = RpcClientExceptionKind.values[e.kind().toInt()];
    _message = e.message();
    _cause = e.cause();
    _trace = e.trace();
    e.free();
  }

  /// Returns concrete error kind of this [RpcClientException].
  @override
  RpcClientExceptionKind kind() {
    return _kind;
  }

  /// Returns error message describing the problem.
  @override
  String message() {
    return _message;
  }

  /// Returns error that caused this [RpcClientException].
  @override
  dynamic cause() {
    return _cause;
  }

  /// Returns stacktrace of this [RpcClientException].
  @override
  String trace() {
    return _trace;
  }
}
