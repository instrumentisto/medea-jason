import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Pointer Function(Object, int)? _get;
int Function(Object)? _length;
Object Function()? _init;
void Function(Object, ForeignValue)? _add;

_ErrorSetterFnDart? _list__get__set_error;
_ErrorSetterFnDart? _list__length__set_error;
_ErrorSetterFnDart? _list__init__set_error;
_ErrorSetterFnDart? _list__add__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Pointer Function(Object, int) get,
  required int Function(Object) length,
  required Object Function() init,
  required void Function(Object, ForeignValue) add,
}) {
  _get = get;
  _length = length;
  _init = init;
  _add = add;

  _list__get__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'list__get__set_error',
  );
  _list__length__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'list__length__set_error',
  );
  _list__init__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'list__init__set_error',
  );
  _list__add__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'list__add__set_error',
  );

  Pointer<NativeFunction<Pointer Function(Handle, Uint32)>> get_native =
      Pointer.fromFunction(_getProxy);
  Pointer<NativeFunction<Uint32 Function(Handle)>> length_native =
      Pointer.fromFunction(_lengthProxy, 0);
  Pointer<NativeFunction<Handle Function()>> init_native = Pointer.fromFunction(
    _initProxy,
  );
  Pointer<NativeFunction<Void Function(Handle, ForeignValue)>> add_native =
      Pointer.fromFunction(_addProxy);

  dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer),
          void Function(Pointer, Pointer, Pointer, Pointer)>('register_list')(
      get_native, length_native, init_native, add_native);
}

Pointer _getProxy(Object arg0, int arg1) {
  try {
    return _get!(arg0, arg1);
  } catch (e) {
    _list__get__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

int _lengthProxy(Object arg0) {
  try {
    return _length!(arg0);
  } catch (e) {
    _list__length__set_error!(e);
    return 0;
  }
}

Object _initProxy() {
  try {
    return _init!();
  } catch (e) {
    _list__init__set_error!(e);
    return 0;
  }
}

void _addProxy(Object arg0, ForeignValue arg1) {
  try {
    return _add!(arg0, arg1);
  } catch (e) {
    _list__add__set_error!(e);
    return;
  }
}
