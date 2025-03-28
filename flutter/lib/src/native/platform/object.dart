import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'object.g.dart' as bridge;

/// Registers [Object]-related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(dl, runtimeType: _runtimeType, toString: _toString);
}

/// Returns string representation of the [Type] of the provided [Object].
Pointer<Utf8> _runtimeType(Object obj) {
  return obj.runtimeType.toString().toNativeUtf8();
}

/// Returns a string representation of the provided [Object].
Pointer<Utf8> _toString(Object obj) {
  return obj.toString().toNativeUtf8();
}
