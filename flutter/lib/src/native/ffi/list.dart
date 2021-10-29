import 'dart:ffi';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register_Array__get')(
      Pointer.fromFunction<Pointer Function(Handle, Int32)>(get));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register_Array__length')(
      Pointer.fromFunction<Int32 Function(Handle)>(len, 0));
}

Pointer get(List arr, int i) {
  final el = arr[i];
  if (el == null) {
    return ForeignValue.none().intoBoxed();
  } else {
    return ForeignValue.fromHandle(el).intoBoxed();
  }
}

int len(List arr) {
  return arr.length;
}
