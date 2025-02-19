import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function(Pointer)? _callTwoArgProxy;
Object Function(Pointer)? _callProxy;

_ErrorSetterFnDart? _callback__call_two_arg_proxy__set_error;
_ErrorSetterFnDart? _callback__call_proxy__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Object Function(Pointer) callTwoArgProxy,
  required Object Function(Pointer) callProxy,
}) {
  _callTwoArgProxy = callTwoArgProxy;
  _callProxy = callProxy;

  _callback__call_two_arg_proxy__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'callback__call_two_arg_proxy__set_error',
  );
  _callback__call_proxy__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'callback__call_proxy__set_error',
  );

  Pointer<NativeFunction<Handle Function(Pointer)>> callTwoArgProxy_native =
      Pointer.fromFunction(_callTwoArgProxyProxy);
  Pointer<NativeFunction<Handle Function(Pointer)>> callProxy_native =
      Pointer.fromFunction(_callProxyProxy);

  dl.lookupFunction<Void Function(Pointer, Pointer),
          void Function(Pointer, Pointer)>('register_callback')(
      callTwoArgProxy_native, callProxy_native);
}

Object _callTwoArgProxyProxy(Pointer arg0) {
  try {
    return _callTwoArgProxy!(arg0);
  } catch (e) {
    _callback__call_two_arg_proxy__set_error!(e);
    return 0;
  }
}

Object _callProxyProxy(Pointer arg0) {
  try {
    return _callProxy!(arg0);
  } catch (e) {
    _callback__call_proxy__set_error!(e);
    return 0;
  }
}
