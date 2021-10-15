import 'dart:js_util';

import '../interface/exceptions.dart';
import 'jason_wasm.dart' as wasm;

/// Returns name of the provided [wasm] exception.
String getExceptionName(dynamic e) {
  var exceptionConstructor = getProperty(e, 'constructor');
  return getProperty(exceptionConstructor, 'name');
}

/// Converts provided [wasm] exception to the Dart exception
dynamic convertException(dynamic e) {
  var name = getExceptionName(e);
  if (name == 'FormatException') {
    return FormatException((e as wasm.FormatException).message());
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
  } else if (name == 'StateError') {
    return WebJasonStateError(e as wasm.StateError);
  } else {
    return e;
  }
}

/// Wraps provided [Function] to the try/catch block and wraps [wasm] exception to the Dart wrapper.
T failableFunction<T>(T Function() f) {
  try {
    return f();
  } catch (e) {
    throw convertException(e);
  }
}

/// Wraps provided [Future] to the try/catch block and wraps [wasm] exception to the Dart wrapper.
Future<T> failableFuture<T>(Future<T> f) async {
  try {
    return await f;
  } catch (e) {
    throw convertException(e);
  }
}

/// Exception thrown when cannot get info of available media devices.
class WebEnumerateDevicesException extends EnumerateDevicesException {
  final wasm.EnumerateDevicesException _exception;

  WebEnumerateDevicesException(this._exception);

  /// Returns error that caused this [EnumerateDevicesException].
  @override
  dynamic cause() {
    return _exception.cause();
  }

  /// Returns stacktrace of this [EnumerateDevicesException].
  @override
  String trace() {
    return _exception.trace();
  }
}

/// Jason's internal exception.
/// This is either a programmatic error or some unexpected platform component
/// failure that cannot be handled in any way.
class WebInternalException extends InternalException {
  final wasm.InternalException _exception;

  WebInternalException(this._exception);

  /// Returns error message describing the problem.
  @override
  String message() {
    return _exception.message();
  }

  /// Returns error that caused this [RpcClientException].
  @override
  dynamic cause() {
    return _exception.cause();
  }

  /// Returns stacktrace of this [InternalException].
  @override
  String trace() {
    return _exception.trace();
  }
}

/// Exception thrown when accessing media devices.
class WebLocalMediaInitException extends LocalMediaInitException {
  final wasm.LocalMediaInitException _exception;

  WebLocalMediaInitException(this._exception);

  /// Returns concrete error kind of this [LocalMediaInitException].
  @override
  LocalMediaInitExceptionKind kind() {
    return LocalMediaInitExceptionKind.values[_exception.kind().toInt()];
  }

  /// Returns error message describing the problem.
  @override
  String message() {
    return _exception.message();
  }

  /// Returns error that caused this [LocalMediaInitException].
  @override
  dynamic cause() {
    return _exception.cause();
  }

  /// Returns stacktrace of this [LocalMediaInitException].
  @override
  String trace() {
    return _exception.trace();
  }
}

/// Errors occurring in `RoomHandle::set_local_media_settings` method.
class WebMediaSettingsUpdateException extends MediaSettingsUpdateException {
  final wasm.MediaSettingsUpdateException _exception;

  WebMediaSettingsUpdateException(this._exception);

  /// Returns error message describing the problem.
  @override
  String message() {
    return _exception.message();
  }

  /// Returns original error that was encountered while updating local media settings.
  @override
  dynamic cause() {
    return _exception.cause();
  }

  /// Returns whether media settings were successfully rolled back after new
  /// settings application failed.
  @override
  bool rolled_back() {
    return _exception.rolled_back();
  }
}

/// Exception thrown when the requested media state transition could not be
/// performed.
class WebMediaStateTransitionException extends MediaStateTransitionException {
  final wasm.MediaStateTransitionException _exception;

  WebMediaStateTransitionException(this._exception);

  /// Returns error message describing the problem.
  @override
  String message() {
    return _exception.message();
  }

  /// Returns stacktrace of this [MediaStateTransitionException].
  @override
  String trace() {
    return _exception.trace();
  }
}

/// Exceptions thrown from an RPC client that implements messaging with media
/// server.
class WebRpcClientException extends RpcClientException {
  final wasm.RpcClientException _exception;

  WebRpcClientException(this._exception);

  /// Returns concrete error kind of this [RpcClientException].
  @override
  RpcClientExceptionKind kind() {
    return RpcClientExceptionKind.values[_exception.kind().toInt()];
  }

  /// Returns error message describing the problem.
  @override
  String message() {
    return _exception.message();
  }

  /// Returns error that caused this [RpcClientException].
  @override
  dynamic cause() {
    return _exception.cause();
  }

  /// Returns stacktrace of this [RpcClientException].
  @override
  String trace() {
    return _exception.trace();
  }
}

/// Error thrown when the operation wasn't allowed by the current state of the
/// object.
class WebJasonStateError extends JasonStateError {
  final wasm.StateError _exception;

  WebJasonStateError(this._exception);

  /// Returns message describing the problem.
  @override
  String message() {
    return _exception.message();
  }

  /// Returns native stacktrace of this [StateError].
  @override
  String trace() {
    return _exception.trace();
  }
}
