import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function()? _init;
void Function(Object, Pointer<Utf8>, ForeignValue)? _set;

_ErrorSetterFnDart? _map__init__set_error;
_ErrorSetterFnDart? _map__set__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Object Function() init,
  required void Function(Object, Pointer<Utf8>, ForeignValue) set,
}) {
  _init = init;
  _set = set;

  _map__init__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'map__init__set_error');
  _map__set__set_error = dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
      'map__set__set_error');

  Pointer<NativeFunction<Handle Function()>> init_native = Pointer.fromFunction(
    _initProxy,
  );
  Pointer<NativeFunction<Void Function(Handle, Pointer<Utf8>, ForeignValue)>>
      set_native = Pointer.fromFunction(
    _setProxy,
  );

  dl.lookupFunction<Void Function(Pointer, Pointer),
      void Function(Pointer, Pointer)>('register_map')(
    init_native,
    set_native,
  );
}

Object _initProxy() {
  try {
    return _init!();
  } catch (e) {
    _map__init__set_error!(e);
    return 0;
  }
}

void _setProxy(Object a, Pointer<Utf8> b, ForeignValue c) {
  try {
    return _set!(a, b, c);
  } catch (e) {
    _map__set__set_error!(e);
    return;
  }
}
