import 'dart:ffi';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

/// Registers functions allowing Rust to create Dart [List]s.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Array__get')(
      Pointer.fromFunction<Pointer Function(Handle, Int32)>(get));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Array__length')(
      Pointer.fromFunction<Int32 Function(Handle)>(len, 0));
}

/// Returns [Pointer] to an object with a provided index.
Pointer get(List arr, int i) {
  final el = arr[i];
  if (el == null) {
    return ForeignValue.none().intoRustOwned();
  } else {
    return ForeignValue.fromHandle(el).intoRustOwned();
  }
}

/// Returns length of the provided [List].
int len(List arr) {
  return arr.length;
}
