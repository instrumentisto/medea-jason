import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Handle Function()>> init,
  required Pointer<NativeFunction<Void Function(Handle, Handle)>> audio,
  required Pointer<NativeFunction<Void Function(Handle, Handle)>> video,
}) {
  dl.lookupFunction<Void Function(Pointer, Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer)>('register_constraints')(
    init,
    audio,
    video,
  );
}
