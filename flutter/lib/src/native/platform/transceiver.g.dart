import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
    DynamicLibrary dl,
    {
        required Object Function(Object) getCurrentDirection,
        required Pointer Function(Object) getSendTrack,
        required Object Function(Object, Object) replaceTrack,
        required Object Function(Object) dropSender,
        required Pointer Function(Object) isStopped,
        required void Function(Object, int) setSendTrackEnabled,
        required Pointer Function(Object) mid,
        required int Function(Object) hasSendTrack,
        required Object Function(Object, int) setDirection,
    }
) {
    dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer), void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer)>('register_transceiver')(
        Pointer.fromFunction<Handle Function(Handle)>(getCurrentDirection),
        Pointer.fromFunction<Pointer Function(Handle)>(getSendTrack),
        Pointer.fromFunction<Handle Function(Handle, Handle)>(replaceTrack),
        Pointer.fromFunction<Handle Function(Handle)>(dropSender),
        Pointer.fromFunction<Pointer Function(Handle)>(isStopped),
        Pointer.fromFunction<Void Function(Handle, Int32)>(setSendTrackEnabled),
        Pointer.fromFunction<Pointer Function(Handle)>(mid),
        Pointer.fromFunction<Int8 Function(Handle)>(hasSendTrack, 0),
        Pointer.fromFunction<Handle Function(Handle, Int64)>(setDirection),
    );
}
