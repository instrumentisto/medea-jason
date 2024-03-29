import 'dart:async';
import 'dart:ffi';

import '../jason.dart';
import 'foreign_value.dart';
import 'function.g.dart' as bridge;

void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    caller: Pointer.fromFunction(_callFn),
  );
}

/// Function used by Rust to call closures with a single [ForeignValue]
/// argument.
void _callFn(FutureOr<void> Function(dynamic) fn, ForeignValue value) {
  try {
    var arg = value.toDart();
    if (arg != null) {
      var res = fn(arg);
      if (res is Future<void>) {
        res.catchError((e, stack) => api.logDartException(
            message: e.toString(), stackTrace: stack.toString()));
      }
    } else {
      var res = (fn as dynamic Function())();
      if (res is Future<void>) {
        res.catchError((e, stack) => api.logDartException(
            message: e.toString(), stackTrace: stack.toString()));
      }
    }
  } catch (e, stack) {
    api.logDartException(message: e.toString(), stackTrace: stack.toString());
  }
}
