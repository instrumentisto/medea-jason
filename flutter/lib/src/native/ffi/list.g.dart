import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Pointer Function(Handle, Uint32)>> get,
  required Pointer<NativeFunction<Uint32 Function(Handle)>> length,
  required Pointer<NativeFunction<Handle Function()>> init,
  required Pointer<NativeFunction<Void Function(Handle, ForeignValue)>> add,
}) {
  dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer, Pointer)>('register_list')(
    get,
    length,
    init,
    add,
  );
}
