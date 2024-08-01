import 'dart:ffi';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function(Object)? _encodings;
Object Function(Object, Object)? _setEncoding;

_ErrorSetterFnDart? _parameters__encodings__set_error;
_ErrorSetterFnDart? _parameters__set_encoding__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Object Function(Object) encodings,
  required Object Function(Object, Object) setEncoding,
}) {
  _encodings = encodings;
  _setEncoding = setEncoding;

  _parameters__encodings__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'parameters__encodings__set_error');
  _parameters__set_encoding__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'parameters__set_encoding__set_error');

  Pointer<NativeFunction<Handle Function(Handle)>> encodings_native =
      Pointer.fromFunction(
    _encodingsProxy,
  );
  Pointer<NativeFunction<Handle Function(Handle, Handle)>> setEncoding_native =
      Pointer.fromFunction(
    _setEncodingProxy,
  );

  dl.lookupFunction<Void Function(Pointer, Pointer),
      void Function(Pointer, Pointer)>('register_parameters')(
    encodings_native,
    setEncoding_native,
  );
}

Object _encodingsProxy(Object arg0) {
  try {
    return _encodings!(arg0);
  } catch (e) {
    _parameters__encodings__set_error!(e);
    return 0;
  }
}

Object _setEncodingProxy(Object arg0, Object arg1) {
  try {
    return _setEncoding!(arg0, arg1);
  } catch (e) {
    _parameters__set_encoding__set_error!(e);
    return 0;
  }
}
