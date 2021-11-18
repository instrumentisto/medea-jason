import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Object Function() constructNew,
  required void Function(Object, Pointer<Utf8>, ForeignValue, ForeignValue) add,
}) {
  dl.lookupFunction<Void Function(Pointer, Pointer),
      void Function(Pointer, Pointer)>('register_ice_servers')(
    Pointer.fromFunction<Handle Function()>(constructNew),
    Pointer.fromFunction<
        Void Function(Handle, Pointer<Utf8>, ForeignValue, ForeignValue)>(add),
  );
}
