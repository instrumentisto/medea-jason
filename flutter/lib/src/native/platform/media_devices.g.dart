import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Handle Function()>> enumerateDevices,
  required Pointer<NativeFunction<Handle Function(Handle)>> getUserMedia,
  required Pointer<NativeFunction<Handle Function(Handle)>> getDisplayMedia,
}) {
  dl.lookupFunction<Void Function(Pointer, Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer)>('register_media_devices')(
    enumerateDevices,
    getUserMedia,
    getDisplayMedia,
  );
}
