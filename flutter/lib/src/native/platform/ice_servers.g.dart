import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function()? _init;
void Function(Object, Pointer<Utf8>, ForeignValue, ForeignValue)? _add;

_ErrorSetterFnDart? _ice_servers__init__set_error;
_ErrorSetterFnDart? _ice_servers__add__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Object Function() init,
  required void Function(Object, Pointer<Utf8>, ForeignValue, ForeignValue) add,
}) {
  _init = init;
  _add = add;

  _ice_servers__init__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'ice_servers__init__set_error');
  _ice_servers__add__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'ice_servers__add__set_error');

  Pointer<NativeFunction<Handle Function()>> init_native = Pointer.fromFunction(
    _initProxy,
  );
  Pointer<
          NativeFunction<
              Void Function(Handle, Pointer<Utf8>, ForeignValue, ForeignValue)>>
      add_native = Pointer.fromFunction(
    _addProxy,
  );

  dl.lookupFunction<Void Function(Pointer, Pointer),
      void Function(Pointer, Pointer)>('register_ice_servers')(
    init_native,
    add_native,
  );
}

Object _initProxy() {
  try {
    return _init!();
  } catch (e) {
    _ice_servers__init__set_error!(e);
    return 0;
  }
}

void _addProxy(
    Object arg0, Pointer<Utf8> arg1, ForeignValue arg2, ForeignValue arg3) {
  try {
    return _add!(arg0, arg1, arg2, arg3);
  } catch (e) {
    _ice_servers__add__set_error!(e);
    return;
  }
}
