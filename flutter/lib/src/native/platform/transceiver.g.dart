import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Handle Function(Handle)>> getDirection,
  required Pointer<NativeFunction<Pointer Function(Handle)>> getSendTrack,
  required Pointer<NativeFunction<Handle Function(Handle, Handle)>>
      replaceTrack,
  required Pointer<NativeFunction<Handle Function(Handle)>> dropSender,
  required Pointer<NativeFunction<Bool Function(Handle)>> isStopped,
  required Pointer<NativeFunction<Pointer Function(Handle)>> mid,
  required Pointer<NativeFunction<Handle Function(Handle, Bool)>> setRecv,
  required Pointer<NativeFunction<Handle Function(Handle, Bool)>> setSend,
  required Pointer<NativeFunction<Handle Function(Handle)>> dispose,
  required Pointer<NativeFunction<Handle Function(Int64)>>
      createTransceiverInit,
  required Pointer<NativeFunction<Void Function(Handle, Handle)>>
      addSendingEncodings,
}) {
  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer, Pointer, Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer, Pointer, Pointer, Pointer)>('register_transceiver')(
    getDirection,
    getSendTrack,
    replaceTrack,
    dropSender,
    isStopped,
    mid,
    setRecv,
    setSend,
    dispose,
    createTransceiverInit,
    addSendingEncodings,
  );
}
