import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Pointer Function(Handle)>> deviceId,
  required Pointer<NativeFunction<Pointer Function(Handle)>> label,
  required Pointer<NativeFunction<Pointer Function(Handle)>> groupId,
  required Pointer<NativeFunction<Pointer Function(Handle)>> kind,
}) {
  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer),
      void Function(
          Pointer, Pointer, Pointer, Pointer)>('register_input_device_info')(
    deviceId,
    label,
    groupId,
    kind,
  );
}
