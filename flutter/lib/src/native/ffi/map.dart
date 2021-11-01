import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

/// Registers functions allowing Rust to create Dart [Map]s.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_DartMap__new')(
      Pointer.fromFunction<Handle Function()>(construct));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_DartMap__set')(
      Pointer.fromFunction<Void Function(Handle, Pointer<Utf8>, ForeignValue)>(
          set));
}

/// Returns empty [Map].
Object construct() {
  return {};
}

/// Sets [ForeignValue] in the provided [Map] at the provided `key`.
void set(Map map, Pointer<Utf8> key, ForeignValue value) {
  map[key.toDartString()] = value.toDart();
}

/// Removes value with a provided `key` from the [Map].
void remove(Map map, Pointer<Utf8> key) {
  map.remove(key.toDartString());
}
