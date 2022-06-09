import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Void Function(Handle, Pointer)>>
      completeProxy,
}) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register_future_from_dart')(
    completeProxy,
  );
}
