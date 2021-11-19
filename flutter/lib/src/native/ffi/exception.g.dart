import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunction(
  DynamicLibrary dl, {
  required Object Function(ForeignValue, Pointer<Utf8>, Pointer<Utf8>)
      newArgumentError,
  required Object Function(Pointer<Utf8>) newStateError,
  required Object Function(Pointer<Utf8>) newFormatException,
  required Object Function(int, Pointer<Utf8>, ForeignValue, Pointer<Utf8>)
      newLocalMediaInitException,
  required Object Function(Pointer<Handle>, Pointer<Utf8>)
      newEnumerateDevicesException,
  required Object Function(int, Pointer<Utf8>, ForeignValue, Pointer<Utf8>)
      newRpcClientException,
  required Object Function(Pointer<Utf8>, Pointer<Utf8>)
      newMediaStateTransitionException,
  required Object Function(Pointer<Utf8>, ForeignValue, Pointer<Utf8>)
      newInternalException,
  required Object Function(Pointer<Utf8>, Pointer<Handle>, int)
      newMediaSettingsUpdateException,
}) {
  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer, Pointer)>('register_exception')(
    Pointer.fromFunction<
        Handle Function(
            ForeignValue, Pointer<Utf8>, Pointer<Utf8>)>(newArgumentError),
    Pointer.fromFunction<Handle Function(Pointer<Utf8>)>(newStateError),
    Pointer.fromFunction<Handle Function(Pointer<Utf8>)>(newFormatException),
    Pointer.fromFunction<
        Handle Function(Int64, Pointer<Utf8>, ForeignValue,
            Pointer<Utf8>)>(newLocalMediaInitException),
    Pointer.fromFunction<Handle Function(Pointer<Handle>, Pointer<Utf8>)>(
        newEnumerateDevicesException),
    Pointer.fromFunction<
        Handle Function(Int64, Pointer<Utf8>, ForeignValue,
            Pointer<Utf8>)>(newRpcClientException),
    Pointer.fromFunction<Handle Function(Pointer<Utf8>, Pointer<Utf8>)>(
        newMediaStateTransitionException),
    Pointer.fromFunction<
        Handle Function(
            Pointer<Utf8>, ForeignValue, Pointer<Utf8>)>(newInternalException),
    Pointer.fromFunction<Handle Function(Pointer<Utf8>, Pointer<Handle>, Int8)>(
        newMediaSettingsUpdateException),
  );
}
