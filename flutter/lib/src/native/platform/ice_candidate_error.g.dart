import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> address,
  required Pointer<NativeFunction<Uint32 Function(Handle)>> port,
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> url,
  required Pointer<NativeFunction<Int32 Function(Handle)>> errorCode,
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> errorText,
}) {
  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer, Pointer,
          Pointer)>('register_ice_candidate_error')(
    address,
    port,
    url,
    errorCode,
    errorText,
  );
}
