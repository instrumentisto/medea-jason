import 'package:ffi/ffi.dart';

import 'dart:ffi';

/// Registers [Object]-related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Object__runtimeType__toString')(
      Pointer.fromFunction<Pointer<Utf8> Function(Handle)>(
          runtimeTypeToString));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Object__toString')(
      Pointer.fromFunction<Pointer<Utf8> Function(Handle)>(toString));
}

/// Returns string representation of the [Type] of the provided [Object].
Pointer<Utf8> runtimeTypeToString(Object obj) {
  return obj.runtimeType.toString().toNativeUtf8();
}

/// Returns a string representation of the provided [Object].
Pointer<Utf8> toString(Object obj) {
  return obj.toString().toNativeUtf8();
}
