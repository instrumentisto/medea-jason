import 'dart:ffi';

import 'package:ffi/ffi.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Pointer<Utf8> Function(Object)? _address;
int Function(Object)? _port;
Pointer<Utf8> Function(Object)? _url;
int Function(Object)? _errorCode;
Pointer<Utf8> Function(Object)? _errorText;

_ErrorSetterFnDart? _ice_candidate_error__address__set_error;
_ErrorSetterFnDart? _ice_candidate_error__port__set_error;
_ErrorSetterFnDart? _ice_candidate_error__url__set_error;
_ErrorSetterFnDart? _ice_candidate_error__error_code__set_error;
_ErrorSetterFnDart? _ice_candidate_error__error_text__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<Utf8> Function(Object) address,
  required int Function(Object) port,
  required Pointer<Utf8> Function(Object) url,
  required int Function(Object) errorCode,
  required Pointer<Utf8> Function(Object) errorText,
}) {
  _address = address;
  _port = port;
  _url = url;
  _errorCode = errorCode;
  _errorText = errorText;

  _ice_candidate_error__address__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'ice_candidate_error__address__set_error');
  _ice_candidate_error__port__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'ice_candidate_error__port__set_error');
  _ice_candidate_error__url__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'ice_candidate_error__url__set_error');
  _ice_candidate_error__error_code__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'ice_candidate_error__error_code__set_error');
  _ice_candidate_error__error_text__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'ice_candidate_error__error_text__set_error');

  Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> address_native =
      Pointer.fromFunction(
    _addressProxy,
  );
  Pointer<NativeFunction<Uint32 Function(Handle)>> port_native =
      Pointer.fromFunction(_portProxy, 0);
  Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> url_native =
      Pointer.fromFunction(
    _urlProxy,
  );
  Pointer<NativeFunction<Int32 Function(Handle)>> errorCode_native =
      Pointer.fromFunction(_errorCodeProxy, 0);
  Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> errorText_native =
      Pointer.fromFunction(
    _errorTextProxy,
  );

  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer, Pointer,
          Pointer)>('register_ice_candidate_error')(
    address_native,
    port_native,
    url_native,
    errorCode_native,
    errorText_native,
  );
}

Pointer<Utf8> _addressProxy(Object arg0) {
  try {
    return _address!(arg0);
  } catch (e) {
    _ice_candidate_error__address__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

int _portProxy(Object arg0) {
  try {
    return _port!(arg0);
  } catch (e) {
    _ice_candidate_error__port__set_error!(e);
    return 0;
  }
}

Pointer<Utf8> _urlProxy(Object arg0) {
  try {
    return _url!(arg0);
  } catch (e) {
    _ice_candidate_error__url__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

int _errorCodeProxy(Object arg0) {
  try {
    return _errorCode!(arg0);
  } catch (e) {
    _ice_candidate_error__error_code__set_error!(e);
    return 0;
  }
}

Pointer<Utf8> _errorTextProxy(Object arg0) {
  try {
    return _errorText!(arg0);
  } catch (e) {
    _ice_candidate_error__error_text__set_error!(e);
    return Pointer.fromAddress(0);
  }
}
