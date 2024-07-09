import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function(Pointer<Utf8>)? _newStateError;
Object Function(Pointer<Utf8>)? _newFormatException;
Object Function(int, Pointer<Utf8>, ForeignValue, Pointer<Utf8>)?
    _newLocalMediaInitException;
Object Function(Pointer<Handle>, Pointer<Utf8>)? _newEnumerateDevicesException;
Object Function(int, Pointer<Utf8>, ForeignValue, Pointer<Utf8>)?
    _newRpcClientException;
Object Function(Pointer<Utf8>, Pointer<Utf8>, int)?
    _newMediaStateTransitionException;
Object Function(Pointer<Utf8>, ForeignValue, Pointer<Utf8>)?
    _newInternalException;
Object Function(Pointer<Utf8>, Pointer<Handle>, bool)?
    _newMediaSettingsUpdateException;
Object Function(Pointer<Utf8>)? _newInvalidOutputAudioDeviceIdException;
Object Function(Pointer<Handle>, Pointer<Utf8>)? _newMicVolumeException;
Object Function()? _throwPanicException;

_ErrorSetterFnDart? _exception__new_state_error__set_error;
_ErrorSetterFnDart? _exception__new_format_exception__set_error;
_ErrorSetterFnDart? _exception__new_local_media_init_exception__set_error;
_ErrorSetterFnDart? _exception__new_enumerate_devices_exception__set_error;
_ErrorSetterFnDart? _exception__new_rpc_client_exception__set_error;
_ErrorSetterFnDart? _exception__new_media_state_transition_exception__set_error;
_ErrorSetterFnDart? _exception__new_internal_exception__set_error;
_ErrorSetterFnDart? _exception__new_media_settings_update_exception__set_error;
_ErrorSetterFnDart?
    _exception__new_invalid_output_audio_device_id_exception__set_error;
_ErrorSetterFnDart? _exception__new_mic_volume_exception__set_error;
_ErrorSetterFnDart? _exception__throw_panic_exception__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Object Function(Pointer<Utf8>) newStateError,
  required Object Function(Pointer<Utf8>) newFormatException,
  required Object Function(int, Pointer<Utf8>, ForeignValue, Pointer<Utf8>)
      newLocalMediaInitException,
  required Object Function(Pointer<Handle>, Pointer<Utf8>)
      newEnumerateDevicesException,
  required Object Function(int, Pointer<Utf8>, ForeignValue, Pointer<Utf8>)
      newRpcClientException,
  required Object Function(Pointer<Utf8>, Pointer<Utf8>, int)
      newMediaStateTransitionException,
  required Object Function(Pointer<Utf8>, ForeignValue, Pointer<Utf8>)
      newInternalException,
  required Object Function(Pointer<Utf8>, Pointer<Handle>, bool)
      newMediaSettingsUpdateException,
  required Object Function(Pointer<Utf8>)
      newInvalidOutputAudioDeviceIdException,
  required Object Function(Pointer<Handle>, Pointer<Utf8>)
      newMicVolumeException,
  required Object Function() throwPanicException,
}) {
  _newStateError = newStateError;
  _newFormatException = newFormatException;
  _newLocalMediaInitException = newLocalMediaInitException;
  _newEnumerateDevicesException = newEnumerateDevicesException;
  _newRpcClientException = newRpcClientException;
  _newMediaStateTransitionException = newMediaStateTransitionException;
  _newInternalException = newInternalException;
  _newMediaSettingsUpdateException = newMediaSettingsUpdateException;
  _newInvalidOutputAudioDeviceIdException =
      newInvalidOutputAudioDeviceIdException;
  _newMicVolumeException = newMicVolumeException;
  _throwPanicException = throwPanicException;

  _exception__new_state_error__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'exception__new_state_error__set_error');
  _exception__new_format_exception__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'exception__new_format_exception__set_error');
  _exception__new_local_media_init_exception__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'exception__new_local_media_init_exception__set_error');
  _exception__new_enumerate_devices_exception__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'exception__new_enumerate_devices_exception__set_error');
  _exception__new_rpc_client_exception__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'exception__new_rpc_client_exception__set_error');
  _exception__new_media_state_transition_exception__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'exception__new_media_state_transition_exception__set_error');
  _exception__new_internal_exception__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'exception__new_internal_exception__set_error');
  _exception__new_media_settings_update_exception__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'exception__new_media_settings_update_exception__set_error');
  _exception__new_invalid_output_audio_device_id_exception__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'exception__new_invalid_output_audio_device_id_exception__set_error');
  _exception__new_mic_volume_exception__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'exception__new_mic_volume_exception__set_error');
  _exception__throw_panic_exception__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'exception__throw_panic_exception__set_error');

  Pointer<NativeFunction<Handle Function(Pointer<Utf8>)>> newStateError_native =
      Pointer.fromFunction(
    _newStateErrorProxy,
  );
  Pointer<NativeFunction<Handle Function(Pointer<Utf8>)>>
      newFormatException_native = Pointer.fromFunction(
    _newFormatExceptionProxy,
  );
  Pointer<
          NativeFunction<
              Handle Function(
                  Int64, Pointer<Utf8>, ForeignValue, Pointer<Utf8>)>>
      newLocalMediaInitException_native = Pointer.fromFunction(
    _newLocalMediaInitExceptionProxy,
  );
  Pointer<NativeFunction<Handle Function(Pointer<Handle>, Pointer<Utf8>)>>
      newEnumerateDevicesException_native = Pointer.fromFunction(
    _newEnumerateDevicesExceptionProxy,
  );
  Pointer<
          NativeFunction<
              Handle Function(
                  Int64, Pointer<Utf8>, ForeignValue, Pointer<Utf8>)>>
      newRpcClientException_native = Pointer.fromFunction(
    _newRpcClientExceptionProxy,
  );
  Pointer<NativeFunction<Handle Function(Pointer<Utf8>, Pointer<Utf8>, Int64)>>
      newMediaStateTransitionException_native = Pointer.fromFunction(
    _newMediaStateTransitionExceptionProxy,
  );
  Pointer<
          NativeFunction<
              Handle Function(Pointer<Utf8>, ForeignValue, Pointer<Utf8>)>>
      newInternalException_native = Pointer.fromFunction(
    _newInternalExceptionProxy,
  );
  Pointer<NativeFunction<Handle Function(Pointer<Utf8>, Pointer<Handle>, Bool)>>
      newMediaSettingsUpdateException_native = Pointer.fromFunction(
    _newMediaSettingsUpdateExceptionProxy,
  );
  Pointer<NativeFunction<Handle Function(Pointer<Utf8>)>>
      newInvalidOutputAudioDeviceIdException_native = Pointer.fromFunction(
    _newInvalidOutputAudioDeviceIdExceptionProxy,
  );
  Pointer<NativeFunction<Handle Function(Pointer<Handle>, Pointer<Utf8>)>>
      newMicVolumeException_native = Pointer.fromFunction(
    _newMicVolumeExceptionProxy,
  );
  Pointer<NativeFunction<Handle Function()>> throwPanicException_native =
      Pointer.fromFunction(
    _throwPanicExceptionProxy,
  );

  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer, Pointer, Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer, Pointer, Pointer, Pointer)>('register_exception')(
    newStateError_native,
    newFormatException_native,
    newLocalMediaInitException_native,
    newEnumerateDevicesException_native,
    newRpcClientException_native,
    newMediaStateTransitionException_native,
    newInternalException_native,
    newMediaSettingsUpdateException_native,
    newInvalidOutputAudioDeviceIdException_native,
    newMicVolumeException_native,
    throwPanicException_native,
  );
}

Object _newStateErrorProxy(Pointer<Utf8> arg0) {
  try {
    return _newStateError!(arg0);
  } catch (e) {
    _exception__new_state_error__set_error!(e);
    return 0;
  }
}

Object _newFormatExceptionProxy(Pointer<Utf8> arg0) {
  try {
    return _newFormatException!(arg0);
  } catch (e) {
    _exception__new_format_exception__set_error!(e);
    return 0;
  }
}

Object _newLocalMediaInitExceptionProxy(
    int arg0, Pointer<Utf8> arg1, ForeignValue arg2, Pointer<Utf8> arg3) {
  try {
    return _newLocalMediaInitException!(arg0, arg1, arg2, arg3);
  } catch (e) {
    _exception__new_local_media_init_exception__set_error!(e);
    return 0;
  }
}

Object _newEnumerateDevicesExceptionProxy(
    Pointer<Handle> arg0, Pointer<Utf8> arg1) {
  try {
    return _newEnumerateDevicesException!(arg0, arg1);
  } catch (e) {
    _exception__new_enumerate_devices_exception__set_error!(e);
    return 0;
  }
}

Object _newRpcClientExceptionProxy(
    int arg0, Pointer<Utf8> arg1, ForeignValue arg2, Pointer<Utf8> arg3) {
  try {
    return _newRpcClientException!(arg0, arg1, arg2, arg3);
  } catch (e) {
    _exception__new_rpc_client_exception__set_error!(e);
    return 0;
  }
}

Object _newMediaStateTransitionExceptionProxy(
    Pointer<Utf8> arg0, Pointer<Utf8> arg1, int arg2) {
  try {
    return _newMediaStateTransitionException!(arg0, arg1, arg2);
  } catch (e) {
    _exception__new_media_state_transition_exception__set_error!(e);
    return 0;
  }
}

Object _newInternalExceptionProxy(
    Pointer<Utf8> arg0, ForeignValue arg1, Pointer<Utf8> arg2) {
  try {
    return _newInternalException!(arg0, arg1, arg2);
  } catch (e) {
    _exception__new_internal_exception__set_error!(e);
    return 0;
  }
}

Object _newMediaSettingsUpdateExceptionProxy(
    Pointer<Utf8> arg0, Pointer<Handle> arg1, bool arg2) {
  try {
    return _newMediaSettingsUpdateException!(arg0, arg1, arg2);
  } catch (e) {
    _exception__new_media_settings_update_exception__set_error!(e);
    return 0;
  }
}

Object _newInvalidOutputAudioDeviceIdExceptionProxy(Pointer<Utf8> arg0) {
  try {
    return _newInvalidOutputAudioDeviceIdException!(arg0);
  } catch (e) {
    _exception__new_invalid_output_audio_device_id_exception__set_error!(e);
    return 0;
  }
}

Object _newMicVolumeExceptionProxy(Pointer<Handle> arg0, Pointer<Utf8> arg1) {
  try {
    return _newMicVolumeException!(arg0, arg1);
  } catch (e) {
    _exception__new_mic_volume_exception__set_error!(e);
    return 0;
  }
}

Object _throwPanicExceptionProxy() {
  try {
    return _throwPanicException!();
  } catch (e) {
    _exception__throw_panic_exception__set_error!(e);
    return 0;
  }
}
