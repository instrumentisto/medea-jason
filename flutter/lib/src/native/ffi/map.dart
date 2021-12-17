import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

/// Registers functions allowing Rust to create Dart [Map]s.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_DartMap__new')(
      Pointer.fromFunction<Handle Function()>(_init));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_DartMap__set')(
      Pointer.fromFunction<Void Function(Handle, Pointer<Utf8>, ForeignValue)>(
          _set));
}

/// Returns empty [Map].
Object _init() {
  return {};
}

/// Sets [ForeignValue] in the provided [Map] at the provided `key`.
void _set(Map map, Pointer<Utf8> key, ForeignValue value) {
  map[key.toDartString()] = value.toDart();
}
