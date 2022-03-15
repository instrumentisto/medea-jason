import 'dart:ffi';

import 'package:ffi/ffi.dart';

import '/src/interface/exceptions.dart';
import 'box_handle.dart';
import 'exception.g.dart' as bridge;
import 'foreign_value.dart';
import 'native_string.dart';

/// Registers functions allowing Rust to create Dart [Exception]s and [Error]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(dl,
      newArgumentError: Pointer.fromFunction(_newArgumentError),
      newStateError: Pointer.fromFunction(_newStateError),
      newFormatException: Pointer.fromFunction(_newFormatException),
      newLocalMediaInitException:
          Pointer.fromFunction(_newLocalMediaInitException),
      newEnumerateDevicesException:
          Pointer.fromFunction(_newEnumerateDevicesException),
      newRpcClientException: Pointer.fromFunction(_newRpcClientException),
      newMediaStateTransitionException:
          Pointer.fromFunction(_newMediaStateTransitionException),
      newInternalException: Pointer.fromFunction(_newInternalException),
      newMediaSettingsUpdateException:
          Pointer.fromFunction(_newMediaSettingsUpdateException),
      throwPanicException: Pointer.fromFunction(_throwPanicException));
}

/// Creates a new [ArgumentError] from the provided invalid [value], its [name]
/// and the [message] describing the problem.
Object _newArgumentError(
    ForeignValue value, Pointer<Utf8> name, Pointer<Utf8> message) {
  return ArgumentError.value(value.toDart(), name.nativeStringToDartString(),
      message.nativeStringToDartString());
}

/// Creates a new [StateError] with the provided [message].
Object _newStateError(Pointer<Utf8> message) {
  return StateError(message.nativeStringToDartString());
}

/// Creates a new [FormatException] with the provided [message].
Object _newFormatException(Pointer<Utf8> message) {
  return FormatException(message.nativeStringToDartString());
}

/// Creates a new [NativeLocalMediaInitException] with the provided error [kind],
/// [message], [cause] and [stacktrace].
Object _newLocalMediaInitException(int kind, Pointer<Utf8> message,
    ForeignValue cause, Pointer<Utf8> stacktrace) {
  return NativeLocalMediaInitException(
      LocalMediaInitExceptionKind.values[kind],
      message.nativeStringToDartString(),
      cause.toDart(),
      stacktrace.nativeStringToDartString());
}

/// Creates a new [NativeEnumerateDevicesException] with the provided error [cause]
/// and [stacktrace].
Object _newEnumerateDevicesException(
    Pointer<Handle> cause, Pointer<Utf8> stacktrace) {
  return NativeEnumerateDevicesException(
      unboxDartHandle(cause), stacktrace.nativeStringToDartString());
}

/// Creates a new [NativeRpcClientException] with the provided error [kind],
/// [message], [cause] and [stacktrace].
Object _newRpcClientException(int kind, Pointer<Utf8> message,
    ForeignValue cause, Pointer<Utf8> stacktrace) {
  return NativeRpcClientException(
      RpcClientExceptionKind.values[kind],
      message.nativeStringToDartString(),
      cause.toDart(),
      stacktrace.nativeStringToDartString());
}

/// Creates a new [NativeMediaStateTransitionException] with the provided error
/// [message] and [stacktrace].
Object _newMediaStateTransitionException(
    Pointer<Utf8> message, Pointer<Utf8> stacktrace) {
  return NativeMediaStateTransitionException(message.nativeStringToDartString(),
      stacktrace.nativeStringToDartString());
}

/// Creates a new [InternalException] with the provided error [message], error
/// [cause] and [stacktrace].
Object _newInternalException(
    Pointer<Utf8> message, ForeignValue cause, Pointer<Utf8> stacktrace) {
  return NativeInternalException(message.nativeStringToDartString(),
      cause.toDart(), stacktrace.nativeStringToDartString());
}

/// Creates a new [NativeMediaSettingsUpdateException] with the provided error
/// [message], error [cause] and [rolledBack] property.
Object _newMediaSettingsUpdateException(
    Pointer<Utf8> message, Pointer<Handle> cause, bool rolledBack) {
  return NativeMediaSettingsUpdateException(
      message.nativeStringToDartString(), unboxDartHandle(cause), rolledBack);
}

/// Throws a new [NativePanicException] with the provided error [message].
Object _throwPanicException() {
  throw NativePanicException();
}

/// Exception thrown when Rust side panics.
class NativePanicException implements Exception {
  /// Instantiates a new [NativePanicException].
  NativePanicException();

  @override
  String toString() {
    return 'Rust code unexpectedly panicked';
  }
}

/// Exception thrown when local media acquisition fails.
class NativeLocalMediaInitException extends LocalMediaInitException
    implements Exception {
  /// Concrete error kind of this [NativeLocalMediaInitException].
  late final LocalMediaInitExceptionKind _kind;

  /// Error message describing the problem.
  late final String _message;

  /// Dart [Exception] or [Error] that caused this [NativeLocalMediaInitException].
  late final Object? _cause;

  /// Native stacktrace.
  late final String _nativeStackTrace;

  /// Instantiates a new [NativeLocalMediaInitException].
  NativeLocalMediaInitException(
      this._kind, this._message, this._cause, this._nativeStackTrace);

  @override
  dynamic cause() {
    return _cause;
  }

  @override
  LocalMediaInitExceptionKind kind() {
    return _kind;
  }

  @override
  String message() {
    return _message;
  }

  @override
  String trace() {
    return _nativeStackTrace;
  }
}

/// Exception thrown when cannot get info about connected [MediaDevices][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#mediadevices
class NativeEnumerateDevicesException extends EnumerateDevicesException
    implements Exception {
  /// Dart [Exception] or [Error] that caused this [NativeEnumerateDevicesException].
  late final Object _cause;

  /// Native stacktrace.
  late final String _nativeStackTrace;

  /// Instantiates a new [NativeEnumerateDevicesException].
  NativeEnumerateDevicesException(this._cause, this._nativeStackTrace);

  @override
  dynamic cause() {
    return _cause;
  }

  @override
  String trace() {
    return _nativeStackTrace;
  }
}

/// Exceptions thrown from `Jason`'s `RpcClient` which implements messaging with
/// media server.
class NativeRpcClientException extends RpcClientException implements Exception {
  /// Concrete error kind of this [NativeRpcClientException].
  late final RpcClientExceptionKind _kind;

  /// Error message describing the problem.
  late final String _message;

  /// Dart [Exception] or [Error] that caused this [NativeRpcClientException].
  late final Object? _cause;

  /// Native stacktrace.
  late final String _nativeStackTrace;

  /// Instantiates a new [NativeRpcClientException].
  NativeRpcClientException(
      this._kind, this._message, this._cause, this._nativeStackTrace);

  @override
  dynamic cause() {
    return _cause;
  }

  @override
  RpcClientExceptionKind kind() {
    return _kind;
  }

  @override
  String message() {
    return _message;
  }

  @override
  String trace() {
    return _nativeStackTrace;
  }
}

/// Exception thrown when the requested media state transition could not be
/// performed.
class NativeMediaStateTransitionException extends MediaStateTransitionException
    implements Exception {
  /// Error message describing the problem.
  late final String _message;

  /// Native stacktrace.
  late final String _nativeStackTrace;

  /// Instantiates a new [NativeMediaStateTransitionException].
  NativeMediaStateTransitionException(this._message, this._nativeStackTrace);

  @override
  String message() {
    return _message;
  }

  @override
  String trace() {
    return _nativeStackTrace;
  }
}

/// Jason's internal exception.
///
/// This is either a programmatic error or some unexpected platform component
/// failure that cannot be handled in any way.
class NativeInternalException extends InternalException implements Exception {
  /// Error message describing the problem.
  late final String _message;

  /// Dart [Exception] or [Error] that caused this [InternalException].
  late final Object? _cause;

  /// Native stacktrace.
  late final String _nativeStackTrace;

  /// Instantiates a new [InternalException].
  NativeInternalException(this._message, this._cause, this._nativeStackTrace);

  @override
  dynamic cause() {
    return _cause;
  }

  @override
  String message() {
    return _message;
  }

  @override
  String trace() {
    return _nativeStackTrace;
  }
}

/// Exception that might happen when updating local media settings via
/// `RoomHandle.setLocalMediaSettings`.
class NativeMediaSettingsUpdateException extends MediaSettingsUpdateException
    implements Exception {
  /// Error message describing the problem.
  late final String _message;

  /// The reason why media settings update failed.
  ///
  /// Possible exception kinds are:
  /// - [StateError] if an underlying `RoomHandle` object has been disposed.
  /// - [NativeLocalMediaInitException] if a request of platform media devices access
  ///   failed.
  /// - [NativeMediaStateTransitionException] if transition is prohibited by tracks
  ///   configuration or explicitly denied by server.
  /// - [InternalException] in case of a programmatic error or some unexpected
  ///   platform component failure.
  late final Object _updateException;

  /// Whether media settings were successfully rolled back after new settings
  /// application failed.
  late final bool _rolledBack;

  /// Instantiates a new [NativeMediaSettingsUpdateException].
  NativeMediaSettingsUpdateException(
      this._message, this._updateException, this._rolledBack);

  @override
  dynamic cause() {
    return _updateException;
  }

  @override
  String message() {
    return _message;
  }

  @override
  bool rolledBack() {
    return _rolledBack;
  }
}
