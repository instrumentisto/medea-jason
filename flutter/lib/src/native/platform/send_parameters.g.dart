import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function(Object)? _encodings;

_ErrorSetterFnDart? _send_parameters__encodings__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Object Function(Object) encodings,
}) {
  _encodings = encodings;

  _send_parameters__encodings__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'send_parameters__encodings__set_error',
  );

  Pointer<NativeFunction<Handle Function(Handle)>> encodings_native =
      Pointer.fromFunction(_encodingsProxy);

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
    'register_send_parameters',
  )(encodings_native);
}

Object _encodingsProxy(Object arg0) {
  try {
    return _encodings!(arg0);
  } catch (e) {
    _send_parameters__encodings__set_error!(e);
    return 0;
  }
}
