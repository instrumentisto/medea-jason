import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Handle Function()>> init,
  required Pointer<
          NativeFunction<Void Function(Handle, Pointer<Utf8>, ForeignValue)>>
      set,
}) {
  dl.lookupFunction<Void Function(Pointer, Pointer),
      void Function(Pointer, Pointer)>('register_map')(
    init,
    set,
  );
}
