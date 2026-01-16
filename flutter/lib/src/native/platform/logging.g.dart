import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function(int)? _setWebrtcLogLevel;

_ErrorSetterFnDart? _logging__set_webrtc_log_level__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Object Function(int) setWebrtcLogLevel,
}) {
  _setWebrtcLogLevel = setWebrtcLogLevel;

  _logging__set_webrtc_log_level__set_error = dl
      .lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
        'logging__set_webrtc_log_level__set_error',
      );

  Pointer<NativeFunction<Handle Function(Int64)>> setWebrtcLogLevel_native =
      Pointer.fromFunction(_setWebrtcLogLevelProxy);

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
    'register_logging',
  )(setWebrtcLogLevel_native);
}

Object _setWebrtcLogLevelProxy(int arg0) {
  try {
    return _setWebrtcLogLevel!(arg0);
  } catch (e) {
    _logging__set_webrtc_log_level__set_error!(e);
    return 0;
  }
}
