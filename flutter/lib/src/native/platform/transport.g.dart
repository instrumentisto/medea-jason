import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<
          NativeFunction<Handle Function(Pointer<Utf8>, Handle, Handle)>>
      connect,
  required Pointer<NativeFunction<Void Function(Handle, Pointer<Utf8>)>> send,
  required Pointer<NativeFunction<Void Function(Handle, Int32, Pointer<Utf8>)>>
      close,
  required Pointer<NativeFunction<Int32 Function(Handle)>> closeCode,
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> closeReason,
}) {
  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer),
      void Function(
          Pointer, Pointer, Pointer, Pointer, Pointer)>('register_transport')(
    connect,
    send,
    close,
    closeCode,
    closeReason,
  );
}
