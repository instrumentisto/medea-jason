import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
    DynamicLibrary dl,
    {
        required void Function(Object, ForeignValue) caller,
    }
) {
    dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>('register_function')(
        Pointer.fromFunction<Void Function(Handle, ForeignValue)>(caller),
    );
}
