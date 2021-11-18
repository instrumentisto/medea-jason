import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
    DynamicLibrary dl,
    {
        required Object Function() constructNew,
        required void Function(Object, ForeignValue) complete,
        required void Function(Object, Pointer<Handle>) completeError,
        required Object Function(Object) future,
        required Object Function(int) delayed,
    }
) {
    dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer, Pointer), void Function(Pointer, Pointer, Pointer, Pointer, Pointer)>('register_completer')(
        Pointer.fromFunction<Handle Function()>(constructNew),
        Pointer.fromFunction<Void Function(Handle, ForeignValue)>(complete),
        Pointer.fromFunction<Void Function(Handle, Pointer<Handle>)>(completeError),
        Pointer.fromFunction<Handle Function(Handle)>(future),
        Pointer.fromFunction<Handle Function(Int32)>(delayed),
    );
}
