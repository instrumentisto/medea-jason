import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> deviceId,
  required Pointer<NativeFunction<Int64 Function(Handle)>> kind,
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> label,
  required Pointer<NativeFunction<Pointer Function(Handle)>> groupId,
}) {
  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer),
      void Function(
          Pointer, Pointer, Pointer, Pointer)>('register_media_device_info')(
    deviceId,
    kind,
    label,
    groupId,
  );
}
