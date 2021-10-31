import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_DartMap__new')(
      Pointer.fromFunction<Handle Function()>(construct));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_DartMap__set')(
      Pointer.fromFunction<Void Function(Handle, Pointer<Utf8>, ForeignValue)>(
          set));
}

Object construct() {
  return {};
}

void set(Map map, Pointer<Utf8> key, ForeignValue value) {
  map[key.toDartString()] = value.toDart();
}

void remove(Map map, Pointer<Utf8> key) {
  map.remove(key.toDartString());
}
