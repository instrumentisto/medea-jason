import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Pointer<Utf8> Function(Object)? _deviceId;
Pointer Function(Object)? _title;

_ErrorSetterFnDart? _media_display_info__device_id__set_error;
_ErrorSetterFnDart? _media_display_info__title__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<Utf8> Function(Object) deviceId,
  required Pointer Function(Object) title,
}) {
  _deviceId = deviceId;
  _title = title;

  _media_display_info__device_id__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_display_info__device_id__set_error');
  _media_display_info__title__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_display_info__title__set_error');

  Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> deviceId_native =
      Pointer.fromFunction(
    _deviceIdProxy,
  );
  Pointer<NativeFunction<Pointer Function(Handle)>> title_native =
      Pointer.fromFunction(
    _titleProxy,
  );

  dl.lookupFunction<Void Function(Pointer, Pointer),
      void Function(Pointer, Pointer)>('register_media_display_info')(
    deviceId_native,
    title_native,
  );
}

Pointer<Utf8> _deviceIdProxy(Object arg0) {
  try {
    return _deviceId!(arg0);
  } catch (e) {
    _media_display_info__device_id__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

Pointer _titleProxy(Object arg0) {
  try {
    return _title!(arg0);
  } catch (e) {
    _media_display_info__title__set_error!(e);
    return Pointer.fromAddress(0);
  }
}
