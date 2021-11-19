import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<NativeFunction<Handle Function()>> init,
  required Pointer<NativeFunction<Void Function(Handle, ForeignValue)>>
      complete,
  required Pointer<NativeFunction<Void Function(Handle, Pointer<Handle>)>>
      completeError,
  required Pointer<NativeFunction<Handle Function(Handle)>> future,
  required Pointer<NativeFunction<Handle Function(Int32)>> delayed,
}) {
  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer),
      void Function(
          Pointer, Pointer, Pointer, Pointer, Pointer)>('register_completer')(
    init,
    complete,
    completeError,
    future,
    delayed,
  );
}
