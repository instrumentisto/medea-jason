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
  print('DART');
  var arg = value.toDart();
  print('DART2 ${arg}');
  if (arg != null) {
    print('DART3 ${arg}');
    fn(arg);
    print('DART4 ${arg}');
  } else {
    print('DART5 ${arg}');
    (fn as void Function())();
    print('DART6 ${arg}');
  }
}
