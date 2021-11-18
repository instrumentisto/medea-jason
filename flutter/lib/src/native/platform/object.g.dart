import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
    DynamicLibrary dl,
    {
        required Pointer<Utf8> Function(Object) runtimeType,
        required Pointer<Utf8> Function(Object) toString,
    }
) {
    dl.lookupFunction<Void Function(Pointer, Pointer), void Function(Pointer, Pointer)>('register_handle')(
        Pointer.fromFunction<Pointer<Utf8> Function(Handle)>(runtimeType),
        Pointer.fromFunction<Pointer<Utf8> Function(Handle)>(toString),
    );
}
