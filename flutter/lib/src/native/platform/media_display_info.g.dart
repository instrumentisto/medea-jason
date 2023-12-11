import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> deviceId,
  required Pointer<NativeFunction<Pointer Function(Handle)>> title,
}) {
  dl.lookupFunction<Void Function(Pointer, Pointer),
      void Function(Pointer, Pointer)>('register_media_display_info')(
    deviceId,
    title,
  );
}
