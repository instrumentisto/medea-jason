import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> id,
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> deviceId,
  required Pointer<NativeFunction<Pointer Function(Handle)>> facingMode,
  required Pointer<NativeFunction<Pointer Function(Handle)>> height,
  required Pointer<NativeFunction<Pointer Function(Handle)>> width,
  required Pointer<NativeFunction<Void Function(Handle, Int32)>> setEnabled,
  required Pointer<NativeFunction<Void Function(Handle)>> stop,
  required Pointer<NativeFunction<Int64 Function(Handle)>> enabled,
  required Pointer<NativeFunction<Int64 Function(Handle)>> kind,
  required Pointer<NativeFunction<Int64 Function(Handle)>> readyState,
  required Pointer<NativeFunction<Void Function(Handle, Handle)>> onEnded,
}) {
  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer, Pointer, Pointer, Pointer),
      void Function(
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer)>('register_media_stream_track')(
    id,
    deviceId,
    facingMode,
    height,
    width,
    setEnabled,
    stop,
    enabled,
    kind,
    readyState,
    onEnded,
  );
}
