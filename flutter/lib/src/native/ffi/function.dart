import 'dart:async';
import 'dart:ffi';

import 'foreign_value.dart';
import 'frb/api/dart/api.dart' as frb;
import 'function.g.dart' as bridge;

void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(dl, caller: _callFn);
}

/// Function used by Rust to call closures with a single [ForeignValue]
/// argument.
void _callFn(Object fn, ForeignValue value) {
  try {
    var arg = value.toDart();
    if (arg != null) {
      if (fn is dynamic Function(dynamic)) {
        var res = fn(arg);
        if (res is Future<void>) {
          res.catchError(
            (e, stack) => frb.logDartException(
              message: e.toString(),
              stackTrace: stack.toString(),
            ),
          );
        }
      } else if (fn is void Function(int)) {
        fn(arg);
      } else {
        throw 'Unknown Function signature, this typecast needs to be extended';
      }
    } else {
      var res = (fn as dynamic Function())();
      if (res is Future<void>) {
        res.catchError(
          (e, stack) => frb.logDartException(
            message: e.toString(),
            stackTrace: stack.toString(),
          ),
        );
      }
    }
  } catch (e, stack) {
    frb.logDartException(message: e.toString(), stackTrace: stack.toString());
  }
}
