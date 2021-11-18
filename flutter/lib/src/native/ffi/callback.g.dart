import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Object Function(Pointer) callTwoArgProxy,
  required Object Function(Pointer) callProxy,
}) {
  dl.lookupFunction<Void Function(Pointer, Pointer),
      void Function(Pointer, Pointer)>('register_callback')(
    Pointer.fromFunction<Handle Function(Pointer)>(callTwoArgProxy),
    Pointer.fromFunction<Handle Function(Pointer)>(callProxy),
  );
}
