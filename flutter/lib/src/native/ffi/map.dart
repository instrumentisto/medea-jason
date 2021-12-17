import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'map.g.dart' as bridge;

/// Registers functions allowing Rust to create Dart [Map]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(dl, init: Pointer.fromFunction(_init), set: Pointer.fromFunction(_set));
}

/// Returns empty [Map].
Object _init() {
  return {};
}

/// Sets [ForeignValue] in the provided [Map] at the provided `key`.
void _set(Map map, Pointer<Utf8> key, ForeignValue value) {
  map[key.toDartString()] = value.toDart();
}
