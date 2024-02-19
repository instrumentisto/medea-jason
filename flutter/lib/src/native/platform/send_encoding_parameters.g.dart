import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Handle Function(Pointer<Utf8>, Bool)>>
      newSendEncodingParameters,
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> getRid,
  required Pointer<NativeFunction<Void Function(Handle, Bool)>> setActive,
  required Pointer<NativeFunction<Void Function(Handle, Int64)>> setMaxBitrate,
  required Pointer<NativeFunction<Void Function(Handle, Int64)>>
      setScaleResolutionDownBy,
}) {
  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer, Pointer,
          Pointer)>('register_send_encoding_parameters')(
    newSendEncodingParameters,
    getRid,
    setActive,
    setMaxBitrate,
    setScaleResolutionDownBy,
  );
}
