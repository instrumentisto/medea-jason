import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Handle Function()>> enumerateDevices,
  required Pointer<NativeFunction<Handle Function(Handle)>> getUserMedia,
  required Pointer<NativeFunction<Handle Function(Handle)>> getDisplayMedia,
  required Pointer<NativeFunction<Handle Function(Pointer<Utf8>)>>
      setOutputAudioId,
  required Pointer<NativeFunction<Handle Function(Int64)>> setMicrophoneVolume,
  required Pointer<NativeFunction<Handle Function()>>
      microphoneVolumeIsAvailable,
  required Pointer<NativeFunction<Handle Function()>> microphoneVolume,
  required Pointer<NativeFunction<Void Function(Handle)>> onDeviceChange,
}) {
  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer)>('register_media_devices')(
    enumerateDevices,
    getUserMedia,
    getDisplayMedia,
    setOutputAudioId,
    setMicrophoneVolume,
    microphoneVolumeIsAvailable,
    microphoneVolume,
    onDeviceChange,
  );
}
