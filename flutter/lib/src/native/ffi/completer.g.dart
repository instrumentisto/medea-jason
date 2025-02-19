import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function()? _init;
void Function(Object, ForeignValue)? _complete;
void Function(Object, Pointer<Handle>)? _completeError;
Object Function(Object)? _future;
Object Function(int)? _delayed;

_ErrorSetterFnDart? _completer__init__set_error;
_ErrorSetterFnDart? _completer__complete__set_error;
_ErrorSetterFnDart? _completer__complete_error__set_error;
_ErrorSetterFnDart? _completer__future__set_error;
_ErrorSetterFnDart? _completer__delayed__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Object Function() init,
  required void Function(Object, ForeignValue) complete,
  required void Function(Object, Pointer<Handle>) completeError,
  required Object Function(Object) future,
  required Object Function(int) delayed,
}) {
  _init = init;
  _complete = complete;
  _completeError = completeError;
  _future = future;
  _delayed = delayed;

  _completer__init__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'completer__init__set_error',
  );
  _completer__complete__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'completer__complete__set_error',
  );
  _completer__complete_error__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'completer__complete_error__set_error',
  );
  _completer__future__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'completer__future__set_error',
  );
  _completer__delayed__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'completer__delayed__set_error',
  );

  Pointer<NativeFunction<Handle Function()>> init_native = Pointer.fromFunction(
    _initProxy,
  );
  Pointer<NativeFunction<Void Function(Handle, ForeignValue)>> complete_native =
      Pointer.fromFunction(_completeProxy);
  Pointer<NativeFunction<Void Function(Handle, Pointer<Handle>)>>
      completeError_native = Pointer.fromFunction(_completeErrorProxy);
  Pointer<NativeFunction<Handle Function(Handle)>> future_native =
      Pointer.fromFunction(_futureProxy);
  Pointer<NativeFunction<Handle Function(Int32)>> delayed_native =
      Pointer.fromFunction(_delayedProxy);

  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer),
      void Function(
          Pointer, Pointer, Pointer, Pointer, Pointer)>('register_completer')(
    init_native,
    complete_native,
    completeError_native,
    future_native,
    delayed_native,
  );
}

Object _initProxy() {
  try {
    return _init!();
  } catch (e) {
    _completer__init__set_error!(e);
    return 0;
  }
}

void _completeProxy(Object arg0, ForeignValue arg1) {
  try {
    return _complete!(arg0, arg1);
  } catch (e) {
    _completer__complete__set_error!(e);
    return;
  }
}

void _completeErrorProxy(Object arg0, Pointer<Handle> arg1) {
  try {
    return _completeError!(arg0, arg1);
  } catch (e) {
    _completer__complete_error__set_error!(e);
    return;
  }
}

Object _futureProxy(Object arg0) {
  try {
    return _future!(arg0);
  } catch (e) {
    _completer__future__set_error!(e);
    return 0;
  }
}

Object _delayedProxy(int arg0) {
  try {
    return _delayed!(arg0);
  } catch (e) {
    _completer__delayed__set_error!(e);
    return 0;
  }
}
