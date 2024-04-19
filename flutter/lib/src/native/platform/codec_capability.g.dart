import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Handle Function(Int64)>>
      getSenderCodecCapabilities,
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> mimeType,
}) {
  dl.lookupFunction<Void Function(Pointer, Pointer),
      void Function(Pointer, Pointer)>('register_codec_capability')(
    getSenderCodecCapabilities,
    mimeType,
  );
}
