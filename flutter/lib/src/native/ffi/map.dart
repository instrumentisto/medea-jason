import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'package:medea_jason/src/native/ffi/native_string.dart';
import 'map.g.dart' as bridge;

/// Registers functions allowing Rust to create Dart [Map]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(dl, init: _init, set: _set);
}

/// Returns an empty [Map].
Map _init() {
  return {};
}

/// Sets the given [value] under the given [key] in the provided [Map].
void _set(Object map, Pointer<Utf8> key, ForeignValue value) {
  map as Map<String, dynamic>;
  map[key.nativeStringToDartString()] = value.toDart();
}
