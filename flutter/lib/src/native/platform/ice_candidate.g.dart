import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
    DynamicLibrary dl,
    {
        required Object Function(ForeignValue, ForeignValue, ForeignValue) constructNew,
        required Pointer Function(Object) candidate,
        required Pointer Function(Object) sdpMLineIndex,
        required Pointer Function(Object) sdpMid,
    }
) {
    dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer), void Function(Pointer, Pointer, Pointer, Pointer)>('register_ice_candidate')(
        Pointer.fromFunction<Handle Function(ForeignValue, ForeignValue, ForeignValue)>(constructNew),
        Pointer.fromFunction<Pointer Function(Handle)>(candidate),
        Pointer.fromFunction<Pointer Function(Handle)>(sdpMLineIndex),
        Pointer.fromFunction<Pointer Function(Handle)>(sdpMid),
    );
}
