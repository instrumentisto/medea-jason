import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
void registerFunction(DynamicLibrary dl, {
required Pointer<NativeFunction<Handle Function(Pointer)>> callTwoArgProxy,
required Pointer<NativeFunction<Handle Function(Pointer)>> callProxy,
} ) {
dl.lookupFunction<Void Function(Pointer, Pointer), void Function(Pointer, Pointer)>('register_callback')(
callTwoArgProxy,
callProxy,
);}
