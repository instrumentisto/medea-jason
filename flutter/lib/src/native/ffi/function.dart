import 'dart:ffi';

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
void _callFn(void Function(dynamic) fn, ForeignValue value) {
  var arg = value.toDart();
  if (arg != null) {
    fn(arg);
  } else {
    (fn as void Function())();
  }
}
