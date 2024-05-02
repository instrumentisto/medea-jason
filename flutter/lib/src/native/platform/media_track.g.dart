import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> id,
  required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> deviceId,
  required Pointer<NativeFunction<Int64 Function(Handle)>> kind,
  required Pointer<NativeFunction<Pointer Function(Handle)>> facingMode,
  required Pointer<NativeFunction<Pointer Function(Handle)>> height,
  required Pointer<NativeFunction<Pointer Function(Handle)>> width,
  required Pointer<NativeFunction<Bool Function(Handle)>> enabled,
  required Pointer<NativeFunction<Void Function(Handle, Bool)>> setEnabled,
  required Pointer<NativeFunction<Handle Function(Handle)>> readyState,
  required Pointer<NativeFunction<Handle Function(Handle)>> stop,
  required Pointer<NativeFunction<Void Function(Handle, Handle)>> onEnded,
  required Pointer<NativeFunction<Handle Function(Handle)>> clone,
  required Pointer<NativeFunction<Handle Function(Handle)>> dispose,
  required Pointer<NativeFunction<Bool Function(Handle)>>
      isOnAudioLevelAvailable,
  required Pointer<NativeFunction<Void Function(Handle, Handle)>>
      onAudioLevelChanged,
}) {
  dl.lookupFunction<
      Void Function(
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
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer),
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
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer)>('register_media_stream_track')(
    id,
    deviceId,
    kind,
    facingMode,
    height,
    width,
    enabled,
    setEnabled,
    readyState,
    stop,
    onEnded,
    clone,
    dispose,
    isOnAudioLevelAvailable,
    onAudioLevelChanged,
  );
}
