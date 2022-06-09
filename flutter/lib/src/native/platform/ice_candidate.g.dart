import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
void registerFunction(DynamicLibrary dl, {
required Pointer<NativeFunction<Handle Function(ForeignValue, ForeignValue, ForeignValue)>> init,
required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> candidate,
required Pointer<NativeFunction<Uint64 Function(Handle)>> sdpMLineIndex,
required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> sdpMid,
} ) {
dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer), void Function(Pointer, Pointer, Pointer, Pointer)>('register_ice_candidate')(
init,
candidate,
sdpMLineIndex,
sdpMid,
);}
