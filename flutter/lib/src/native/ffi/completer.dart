import 'dart:async';
import 'dart:ffi';

import 'box_handle.dart';
import 'completer.g.dart' as bridge;
import 'foreign_value.dart';

/// Registers functions that allow Rust to manage [Completer]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    init: _new,
    future: _future,
    complete: _complete,
    completeError: _completeError,
    delayed: _delayed,
  );
}

/// Returns closure returning a [Future.delayed] with the provided amount of
/// milliseconds.
Future Function() _delayed(int delayMs) {
  return () => Future.delayed(Duration(milliseconds: delayMs));
}

/// Returns a new [Completer].
Completer _new() {
  return Completer();
}

/// Returns a [Future] that is completed by the provided [Completer].
Future _future(Object completer) {
  completer as Completer;
  return completer.future;
}

/// Completes the provided [Completer] with the provided [ForeignValue].
void _complete(Object completer, ForeignValue arg) {
  completer as Completer;
  completer.complete(arg.toDart());
}

/// Complete the provided [Completer] with an error.
void _completeError(Object completer, Pointer<Handle> err) {
  completer as Completer;
  var e = unboxDartHandle(err);
  freeBoxedDartHandle(err);
  completer.completeError(e);
}
