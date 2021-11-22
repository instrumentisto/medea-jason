import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Handle Function(Handle)>> getCurrentDirection,
  required Pointer<NativeFunction<Pointer Function(Handle)>> getSendTrack,
  required Pointer<NativeFunction<Handle Function(Handle, Handle)>>
      replaceTrack,
  required Pointer<NativeFunction<Handle Function(Handle)>> dropSender,
  required Pointer<NativeFunction<Pointer Function(Handle)>> isStopped,
  required Pointer<NativeFunction<Void Function(Handle, Int32)>>
      setSendTrackEnabled,
  required Pointer<NativeFunction<Pointer Function(Handle)>> mid,
  required Pointer<NativeFunction<Int8 Function(Handle)>> hasSendTrack,
  required Pointer<NativeFunction<Handle Function(Handle, Int64)>> setDirection,
}) {
  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer, Pointer)>('register_transceiver')(
    getCurrentDirection,
    getSendTrack,
    replaceTrack,
    dropSender,
    isStopped,
    setSendTrackEnabled,
    mid,
    hasSendTrack,
    setDirection,
  );
}
