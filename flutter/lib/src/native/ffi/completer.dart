import 'dart:async';
import 'dart:ffi';

import 'box_handle.dart';
import 'completer.g.dart' as bridge;
import 'foreign_value.dart';

/// Registers functions that allow Rust to manage [Completer]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    init: Pointer.fromFunction(_new),
    future: Pointer.fromFunction(_future),
    complete: Pointer.fromFunction(_complete),
    completeError: Pointer.fromFunction(_completeError),
    delayed: Pointer.fromFunction(_delayed),
  );
}

/// Returns closure returning a [Future.delayed] with the provided amount of
/// milliseconds.
Object _delayed(int delayMs) {
  return () => Future.delayed(Duration(milliseconds: delayMs));
}

/// Returns a new [Completer].
Object _new() {
  return Completer();
}

/// Returns a [Future] that is completed by the provided [Completer].
Object _future(Object completer) {
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
