import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
void registerFunction(DynamicLibrary dl, {
required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> runtimeType,
required Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> toString,
} ) {
dl.lookupFunction<Void Function(Pointer, Pointer), void Function(Pointer, Pointer)>('register_handle')(
runtimeType,
toString,
);}
