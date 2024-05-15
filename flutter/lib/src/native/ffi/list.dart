import 'dart:ffi';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'list.g.dart' as bridge;

/// Registers functions allowing Rust to create Dart [List]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    get: Pointer.fromFunction(_get),
    length: Pointer.fromFunction(_len, 0),
  );
}

/// Returns a [Pointer] to a [List] element with the provided [index].
Pointer _get(Object arr, int index) {
  arr as List;

  final el = arr[index];
  if (el == null) {
    return ForeignValue.none().intoRustOwned();
  } else {
    return ForeignValue.fromHandle(el).intoRustOwned();
  }
}

/// Returns length of the provided [List].
int _len(Object arr) {
  arr as List;

  return arr.length;
}
