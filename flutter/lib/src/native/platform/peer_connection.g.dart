import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
    DynamicLibrary dl,
    {
        required int Function(Object) iceConnectionState,
        required void Function(Object, Object) onConnectionStateChange,
        required Pointer Function(Object) connectionState,
        required void Function(Object) restartIce,
        required Object Function(Object) rollback,
        required void Function(Object, Object) onTrack,
        required void Function(Object, Object) onIceCandidate,
        required Object Function(Object, Pointer<Utf8>) getTransceiverByMid,
        required Object Function(Object, Object) addIceCandidate,
        required void Function(Object, Object) onIceConnectionStateChange,
        required Object Function(Object) newPeer,
        required Object Function(Object, int, int) addTransceiver,
        required Object Function(Object) createOffer,
        required Object Function(Object) createAnswer,
        required Object Function(Object, Pointer<Utf8>, Pointer<Utf8>) setLocalDescription,
        required Object Function(Object, Pointer<Utf8>, Pointer<Utf8>) setRemoteDescription,
    }
) {
    dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer), void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer)>('register_peer_connection')(
        Pointer.fromFunction<Int32 Function(Handle)>(iceConnectionState, 0),
        Pointer.fromFunction<Void Function(Handle, Handle)>(onConnectionStateChange),
        Pointer.fromFunction<Pointer Function(Handle)>(connectionState),
        Pointer.fromFunction<Void Function(Handle)>(restartIce),
        Pointer.fromFunction<Handle Function(Handle)>(rollback),
        Pointer.fromFunction<Void Function(Handle, Handle)>(onTrack),
        Pointer.fromFunction<Void Function(Handle, Handle)>(onIceCandidate),
        Pointer.fromFunction<Handle Function(Handle, Pointer<Utf8>)>(getTransceiverByMid),
        Pointer.fromFunction<Handle Function(Handle, Handle)>(addIceCandidate),
        Pointer.fromFunction<Void Function(Handle, Handle)>(onIceConnectionStateChange),
        Pointer.fromFunction<Handle Function(Handle)>(newPeer),
        Pointer.fromFunction<Handle Function(Handle, Int64, Int64)>(addTransceiver),
        Pointer.fromFunction<Handle Function(Handle)>(createOffer),
        Pointer.fromFunction<Handle Function(Handle)>(createAnswer),
        Pointer.fromFunction<Handle Function(Handle, Pointer<Utf8>, Pointer<Utf8>)>(setLocalDescription),
        Pointer.fromFunction<Handle Function(Handle, Pointer<Utf8>, Pointer<Utf8>)>(setRemoteDescription),
    );
}
