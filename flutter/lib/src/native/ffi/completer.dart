import 'dart:async';
import 'dart:ffi';

import 'foreign_value.dart';
import 'box_handle.dart';
import 'completer.g.dart' as bridge;

/// Registers functions that allow Rust to manage [Completer]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    constructNew: _Completer_new,
    future: _Completer_future,
    complete: _Completer_complete,
    completeError: _Completer_completeError_Pointer,
    delayed: delayedFuture,
  );
}

/// Returns closure returning a [Future.delayed] with the provided amount of
/// milliseconds.
Object delayedFuture(int delayMs) {
  return () => Future.delayed(Duration(milliseconds: delayMs));
}

/// Returns a new [Completer].
Object _Completer_new() {
  return Completer();
}

/// Returns a [Future] that is completed by the provided [Completer].
Object _Completer_future(Object completer) {
  return (completer as Completer).future;
}

/// Completes the provided [Completer] with the provided [ForeignValue].
void _Completer_complete(Object completer, ForeignValue arg) {
  (completer as Completer).complete(arg.toDart());
}

/// Complete the provided [Completer] with an error.
void _Completer_completeError_Pointer(Object completer, Pointer<Handle> err) {
  (completer as Completer).completeError(unboxDartHandle(err));
}
