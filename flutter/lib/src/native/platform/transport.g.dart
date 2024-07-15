import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function(Pointer<Utf8>, Object, Object)? _connect;
void Function(Object, Pointer<Utf8>)? _send;
void Function(Object, int, Pointer<Utf8>)? _close;
int Function(Object)? _closeCode;
Pointer<Utf8> Function(Object)? _closeReason;

_ErrorSetterFnDart? _transport__connect__set_error;
_ErrorSetterFnDart? _transport__send__set_error;
_ErrorSetterFnDart? _transport__close__set_error;
_ErrorSetterFnDart? _transport__close_code__set_error;
_ErrorSetterFnDart? _transport__close_reason__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Object Function(Pointer<Utf8>, Object, Object) connect,
  required void Function(Object, Pointer<Utf8>) send,
  required void Function(Object, int, Pointer<Utf8>) close,
  required int Function(Object) closeCode,
  required Pointer<Utf8> Function(Object) closeReason,
}) {
  _connect = connect;
  _send = send;
  _close = close;
  _closeCode = closeCode;
  _closeReason = closeReason;

  _transport__connect__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transport__connect__set_error');
  _transport__send__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transport__send__set_error');
  _transport__close__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transport__close__set_error');
  _transport__close_code__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transport__close_code__set_error');
  _transport__close_reason__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transport__close_reason__set_error');

  Pointer<NativeFunction<Handle Function(Pointer<Utf8>, Handle, Handle)>>
      connect_native = Pointer.fromFunction(
    _connectProxy,
  );
  Pointer<NativeFunction<Void Function(Handle, Pointer<Utf8>)>> send_native =
      Pointer.fromFunction(
    _sendProxy,
  );
  Pointer<NativeFunction<Void Function(Handle, Int32, Pointer<Utf8>)>>
      close_native = Pointer.fromFunction(
    _closeProxy,
  );
  Pointer<NativeFunction<Int32 Function(Handle)>> closeCode_native =
      Pointer.fromFunction(_closeCodeProxy, 0);
  Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> closeReason_native =
      Pointer.fromFunction(
    _closeReasonProxy,
  );

  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer),
      void Function(
          Pointer, Pointer, Pointer, Pointer, Pointer)>('register_transport')(
    connect_native,
    send_native,
    close_native,
    closeCode_native,
    closeReason_native,
  );
}

Object _connectProxy(Pointer<Utf8> arg0, Object arg1, Object arg2) {
  try {
    return _connect!(arg0, arg1, arg2);
  } catch (e) {
    _transport__connect__set_error!(e);
    return 0;
  }
}

void _sendProxy(Object arg0, Pointer<Utf8> arg1) {
  try {
    return _send!(arg0, arg1);
  } catch (e) {
    _transport__send__set_error!(e);
    return;
  }
}

void _closeProxy(Object arg0, int arg1, Pointer<Utf8> arg2) {
  try {
    return _close!(arg0, arg1, arg2);
  } catch (e) {
    _transport__close__set_error!(e);
    return;
  }
}

int _closeCodeProxy(Object arg0) {
  try {
    return _closeCode!(arg0);
  } catch (e) {
    _transport__close_code__set_error!(e);
    return 0;
  }
}

Pointer<Utf8> _closeReasonProxy(Object arg0) {
  try {
    return _closeReason!(arg0);
  } catch (e) {
    _transport__close_reason__set_error!(e);
    return Pointer.fromAddress(0);
  }
}
