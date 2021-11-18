import 'dart:ffi';

import 'foreign_value.dart';
import 'function.g.dart' as bridge;

void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
      dl,
      caller: _callFn,
  );
}

/// Function used by Rust to call closures with a single [ForeignValue]
/// argument.
void _callFn(Object fn, ForeignValue value) {
  fn as void Function(dynamic);
  var arg = value.toDart();
  if (arg != null) {
    fn(arg);
  } else {
    (fn as void Function())();
  }
}
