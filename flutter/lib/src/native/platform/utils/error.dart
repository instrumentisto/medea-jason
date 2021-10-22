import 'package:ffi/ffi.dart';
import 'dart:ffi';

/// Registers functions needed for `DartFutureResolver` working.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Error__name')(
      Pointer.fromFunction<Pointer<Utf8> Function(Handle)>(name));
  ;
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Error__message')(
      Pointer.fromFunction<Pointer<Utf8> Function(Handle)>(message));
}

/// Registers function needed for obtaining exception name.
Pointer<Utf8> name(Object exception) {
  return exception.runtimeType.toString().toNativeUtf8();
}

/// Registers function needed for obtaining exception message.
Pointer<Utf8> message(Object exception) {
  return exception.toString().toNativeUtf8();
}
