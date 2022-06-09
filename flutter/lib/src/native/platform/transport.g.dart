import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
void registerFunction(DynamicLibrary dl, {
required Pointer<NativeFunction<Handle Function(Pointer<Utf8>, Handle, Handle)>> connect,
required Pointer<NativeFunction<Void Function(Handle, Pointer<Utf8>)>> send,
required Pointer<NativeFunction<Void Function(Handle, Int32, Pointer<Utf8>)>> close,
} ) {
dl.lookupFunction<Void Function(Pointer, Pointer, Pointer), void Function(Pointer, Pointer, Pointer)>('register_transport')(
connect,
send,
close,
);}
