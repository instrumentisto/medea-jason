import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Pointer Function(Handle, Int32)>> get,
  required Pointer<NativeFunction<Int32 Function(Handle)>> length,
}) {
  dl.lookupFunction<Void Function(Pointer, Pointer),
      void Function(Pointer, Pointer)>('register_list')(
    get,
    length,
  );
}
