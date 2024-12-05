import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Pointer<Utf8> Function(Object)? _deviceId;
int Function(Object)? _kind;
Pointer<Utf8> Function(Object)? _label;
Pointer Function(Object)? _groupId;
bool Function(Object)? _isFailed;

_ErrorSetterFnDart? _media_device_info__device_id__set_error;
_ErrorSetterFnDart? _media_device_info__kind__set_error;
_ErrorSetterFnDart? _media_device_info__label__set_error;
_ErrorSetterFnDart? _media_device_info__group_id__set_error;
_ErrorSetterFnDart? _media_device_info__is_failed__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<Utf8> Function(Object) deviceId,
  required int Function(Object) kind,
  required Pointer<Utf8> Function(Object) label,
  required Pointer Function(Object) groupId,
  required bool Function(Object) isFailed,
}) {
  _deviceId = deviceId;
  _kind = kind;
  _label = label;
  _groupId = groupId;
  _isFailed = isFailed;

  _media_device_info__device_id__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_device_info__device_id__set_error');
  _media_device_info__kind__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_device_info__kind__set_error');
  _media_device_info__label__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_device_info__label__set_error');
  _media_device_info__group_id__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_device_info__group_id__set_error');
  _media_device_info__is_failed__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_device_info__is_failed__set_error');

  Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> deviceId_native =
      Pointer.fromFunction(
    _deviceIdProxy,
  );
  Pointer<NativeFunction<Int64 Function(Handle)>> kind_native =
      Pointer.fromFunction(_kindProxy, 0);
  Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> label_native =
      Pointer.fromFunction(
    _labelProxy,
  );
  Pointer<NativeFunction<Pointer Function(Handle)>> groupId_native =
      Pointer.fromFunction(
    _groupIdProxy,
  );
  Pointer<NativeFunction<Bool Function(Handle)>> isFailed_native =
      Pointer.fromFunction(_isFailedProxy, false);

  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer, Pointer,
          Pointer)>('register_media_device_info')(
    deviceId_native,
    kind_native,
    label_native,
    groupId_native,
    isFailed_native,
  );
}

Pointer<Utf8> _deviceIdProxy(Object arg0) {
  try {
    return _deviceId!(arg0);
  } catch (e) {
    _media_device_info__device_id__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

int _kindProxy(Object arg0) {
  try {
    return _kind!(arg0);
  } catch (e) {
    _media_device_info__kind__set_error!(e);
    return 0;
  }
}

Pointer<Utf8> _labelProxy(Object arg0) {
  try {
    return _label!(arg0);
  } catch (e) {
    _media_device_info__label__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

Pointer _groupIdProxy(Object arg0) {
  try {
    return _groupId!(arg0);
  } catch (e) {
    _media_device_info__group_id__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

bool _isFailedProxy(Object arg0) {
  try {
    return _isFailed!(arg0);
  } catch (e) {
    _media_device_info__is_failed__set_error!(e);
    return false;
  }
}
