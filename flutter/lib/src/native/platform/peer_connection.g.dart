import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Int32 Function(Handle)>> iceConnectionState,
  required Pointer<NativeFunction<Void Function(Handle, Handle)>>
      onConnectionStateChange,
  required Pointer<NativeFunction<Pointer Function(Handle)>> connectionState,
  required Pointer<NativeFunction<Void Function(Handle)>> restartIce,
  required Pointer<NativeFunction<Handle Function(Handle)>> rollback,
  required Pointer<NativeFunction<Void Function(Handle, Handle)>> onTrack,
  required Pointer<NativeFunction<Void Function(Handle, Handle)>>
      onIceCandidate,
  required Pointer<NativeFunction<Handle Function(Handle, Pointer<Utf8>)>>
      getTransceiverByMid,
  required Pointer<NativeFunction<Handle Function(Handle, Handle)>>
      addIceCandidate,
  required Pointer<NativeFunction<Void Function(Handle, Handle)>>
      onIceConnectionStateChange,
  required Pointer<NativeFunction<Handle Function(Handle)>> newPeer,
  required Pointer<NativeFunction<Handle Function(Handle, Int64, Int64)>>
      addTransceiver,
  required Pointer<NativeFunction<Handle Function(Handle)>> createOffer,
  required Pointer<NativeFunction<Handle Function(Handle)>> createAnswer,
  required Pointer<
          NativeFunction<Handle Function(Handle, Pointer<Utf8>, Pointer<Utf8>)>>
      setLocalDescription,
  required Pointer<
          NativeFunction<Handle Function(Handle, Pointer<Utf8>, Pointer<Utf8>)>>
      setRemoteDescription,
  required Pointer<NativeFunction<Void Function(Handle)>> close,
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
          Pointer,
          Pointer,
          Pointer)>('register_peer_connection')(
    iceConnectionState,
    onConnectionStateChange,
    connectionState,
    restartIce,
    rollback,
    onTrack,
    onIceCandidate,
    getTransceiverByMid,
    addIceCandidate,
    onIceConnectionStateChange,
    newPeer,
    addTransceiver,
    createOffer,
    createAnswer,
    setLocalDescription,
    setRemoteDescription,
    close,
  );
}
