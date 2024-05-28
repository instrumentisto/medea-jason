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
void _callFn(Object fn, ForeignValue value) {
  try {
    var arg = value.toDart();
    if (arg != null) {
      if (fn is void Function(dynamic)) {
        (fn as void Function(dynamic))(arg);
      } else if (fn is dynamic Function(dynamic)) {
        var res = (fn as dynamic Function(dynamic))(arg);
        if (res is Future<void>) {
          res.catchError((e, stack) => api.logDartException(
              message: e.toString(), stackTrace: stack.toString()));
        }
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
