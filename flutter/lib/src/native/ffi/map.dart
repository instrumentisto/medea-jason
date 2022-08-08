import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'package:medea_jason/src/native/ffi/native_string.dart';
import 'map.g.dart' as bridge;

/// Registers functions allowing Rust to create Dart [Map]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(dl,
      init: Pointer.fromFunction(_init), set: Pointer.fromFunction(_set));
}

/// Returns an empty [Map].
Object _init() {
  return {};
}

/// Sets the given [value] under the given [key] in the provided [Map].
void _set(Map<String, dynamic> map, Pointer<Utf8> key, ForeignValue value) {
  map[key.nativeStringToDartString()] = value.toDart();
}
