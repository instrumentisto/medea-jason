import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Pointer<Utf8> Function(Object)? _id;
Pointer<Utf8> Function(Object)? _deviceId;
int Function(Object)? _kind;
Pointer Function(Object)? _facingMode;
Pointer Function(Object)? _height;
Pointer Function(Object)? _width;
bool Function(Object)? _enabled;
void Function(Object, bool)? _setEnabled;
Object Function(Object)? _readyState;
Object Function(Object)? _stop;
void Function(Object, Object)? _onEnded;
Object Function(Object)? _clone;
Object Function(Object)? _dispose;
bool Function(Object)? _isOnAudioLevelAvailable;
void Function(Object, Object)? _onAudioLevelChanged;

_ErrorSetterFnDart? _media_stream_track__id__set_error;
_ErrorSetterFnDart? _media_stream_track__device_id__set_error;
_ErrorSetterFnDart? _media_stream_track__kind__set_error;
_ErrorSetterFnDart? _media_stream_track__facing_mode__set_error;
_ErrorSetterFnDart? _media_stream_track__height__set_error;
_ErrorSetterFnDart? _media_stream_track__width__set_error;
_ErrorSetterFnDart? _media_stream_track__enabled__set_error;
_ErrorSetterFnDart? _media_stream_track__set_enabled__set_error;
_ErrorSetterFnDart? _media_stream_track__ready_state__set_error;
_ErrorSetterFnDart? _media_stream_track__stop__set_error;
_ErrorSetterFnDart? _media_stream_track__on_ended__set_error;
_ErrorSetterFnDart? _media_stream_track__clone__set_error;
_ErrorSetterFnDart? _media_stream_track__dispose__set_error;
_ErrorSetterFnDart? _media_stream_track__is_on_audio_level_available__set_error;
_ErrorSetterFnDart? _media_stream_track__on_audio_level_changed__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Pointer<Utf8> Function(Object) id,
  required Pointer<Utf8> Function(Object) deviceId,
  required int Function(Object) kind,
  required Pointer Function(Object) facingMode,
  required Pointer Function(Object) height,
  required Pointer Function(Object) width,
  required bool Function(Object) enabled,
  required void Function(Object, bool) setEnabled,
  required Object Function(Object) readyState,
  required Object Function(Object) stop,
  required void Function(Object, Object) onEnded,
  required Object Function(Object) clone,
  required Object Function(Object) dispose,
  required bool Function(Object) isOnAudioLevelAvailable,
  required void Function(Object, Object) onAudioLevelChanged,
}) {
  _id = id;
  _deviceId = deviceId;
  _kind = kind;
  _facingMode = facingMode;
  _height = height;
  _width = width;
  _enabled = enabled;
  _setEnabled = setEnabled;
  _readyState = readyState;
  _stop = stop;
  _onEnded = onEnded;
  _clone = clone;
  _dispose = dispose;
  _isOnAudioLevelAvailable = isOnAudioLevelAvailable;
  _onAudioLevelChanged = onAudioLevelChanged;

  _media_stream_track__id__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__id__set_error');
  _media_stream_track__device_id__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__device_id__set_error');
  _media_stream_track__kind__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__kind__set_error');
  _media_stream_track__facing_mode__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__facing_mode__set_error');
  _media_stream_track__height__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__height__set_error');
  _media_stream_track__width__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__width__set_error');
  _media_stream_track__enabled__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__enabled__set_error');
  _media_stream_track__set_enabled__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__set_enabled__set_error');
  _media_stream_track__ready_state__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__ready_state__set_error');
  _media_stream_track__stop__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__stop__set_error');
  _media_stream_track__on_ended__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__on_ended__set_error');
  _media_stream_track__clone__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__clone__set_error');
  _media_stream_track__dispose__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__dispose__set_error');
  _media_stream_track__is_on_audio_level_available__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__is_on_audio_level_available__set_error');
  _media_stream_track__on_audio_level_changed__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'media_stream_track__on_audio_level_changed__set_error');

  Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> id_native =
      Pointer.fromFunction(
    _idProxy,
  );
  Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> deviceId_native =
      Pointer.fromFunction(
    _deviceIdProxy,
  );
  Pointer<NativeFunction<Int64 Function(Handle)>> kind_native =
      Pointer.fromFunction(_kindProxy, 0);
  Pointer<NativeFunction<Pointer Function(Handle)>> facingMode_native =
      Pointer.fromFunction(
    _facingModeProxy,
  );
  Pointer<NativeFunction<Pointer Function(Handle)>> height_native =
      Pointer.fromFunction(
    _heightProxy,
  );
  Pointer<NativeFunction<Pointer Function(Handle)>> width_native =
      Pointer.fromFunction(
    _widthProxy,
  );
  Pointer<NativeFunction<Bool Function(Handle)>> enabled_native =
      Pointer.fromFunction(_enabledProxy, false);
  Pointer<NativeFunction<Void Function(Handle, Bool)>> setEnabled_native =
      Pointer.fromFunction(
    _setEnabledProxy,
  );
  Pointer<NativeFunction<Handle Function(Handle)>> readyState_native =
      Pointer.fromFunction(
    _readyStateProxy,
  );
  Pointer<NativeFunction<Handle Function(Handle)>> stop_native =
      Pointer.fromFunction(
    _stopProxy,
  );
  Pointer<NativeFunction<Void Function(Handle, Handle)>> onEnded_native =
      Pointer.fromFunction(
    _onEndedProxy,
  );
  Pointer<NativeFunction<Handle Function(Handle)>> clone_native =
      Pointer.fromFunction(
    _cloneProxy,
  );
  Pointer<NativeFunction<Handle Function(Handle)>> dispose_native =
      Pointer.fromFunction(
    _disposeProxy,
  );
  Pointer<NativeFunction<Bool Function(Handle)>>
      isOnAudioLevelAvailable_native =
      Pointer.fromFunction(_isOnAudioLevelAvailableProxy, false);
  Pointer<NativeFunction<Void Function(Handle, Handle)>>
      onAudioLevelChanged_native = Pointer.fromFunction(
    _onAudioLevelChangedProxy,
  );

  dl.lookupFunction<
      Void Function(
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer),
      void Function(
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer,
          Pointer)>('register_media_stream_track')(
    id_native,
    deviceId_native,
    kind_native,
    facingMode_native,
    height_native,
    width_native,
    enabled_native,
    setEnabled_native,
    readyState_native,
    stop_native,
    onEnded_native,
    clone_native,
    dispose_native,
    isOnAudioLevelAvailable_native,
    onAudioLevelChanged_native,
  );
}

Pointer<Utf8> _idProxy(Object arg0) {
  try {
    return _id!(arg0);
  } catch (e) {
    _media_stream_track__id__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

Pointer<Utf8> _deviceIdProxy(Object arg0) {
  try {
    return _deviceId!(arg0);
  } catch (e) {
    _media_stream_track__device_id__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

int _kindProxy(Object arg0) {
  try {
    return _kind!(arg0);
  } catch (e) {
    _media_stream_track__kind__set_error!(e);
    return 0;
  }
}

Pointer _facingModeProxy(Object arg0) {
  try {
    return _facingMode!(arg0);
  } catch (e) {
    _media_stream_track__facing_mode__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

Pointer _heightProxy(Object arg0) {
  try {
    return _height!(arg0);
  } catch (e) {
    _media_stream_track__height__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

Pointer _widthProxy(Object arg0) {
  try {
    return _width!(arg0);
  } catch (e) {
    _media_stream_track__width__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

bool _enabledProxy(Object arg0) {
  try {
    return _enabled!(arg0);
  } catch (e) {
    _media_stream_track__enabled__set_error!(e);
    return false;
  }
}

void _setEnabledProxy(Object arg0, bool arg1) {
  try {
    return _setEnabled!(arg0, arg1);
  } catch (e) {
    _media_stream_track__set_enabled__set_error!(e);
    return;
  }
}

Object _readyStateProxy(Object arg0) {
  try {
    return _readyState!(arg0);
  } catch (e) {
    _media_stream_track__ready_state__set_error!(e);
    return 0;
  }
}

Object _stopProxy(Object arg0) {
  try {
    return _stop!(arg0);
  } catch (e) {
    _media_stream_track__stop__set_error!(e);
    return 0;
  }
}

void _onEndedProxy(Object arg0, Object arg1) {
  try {
    return _onEnded!(arg0, arg1);
  } catch (e) {
    _media_stream_track__on_ended__set_error!(e);
    return;
  }
}

Object _cloneProxy(Object arg0) {
  try {
    return _clone!(arg0);
  } catch (e) {
    _media_stream_track__clone__set_error!(e);
    return 0;
  }
}

Object _disposeProxy(Object arg0) {
  try {
    return _dispose!(arg0);
  } catch (e) {
    _media_stream_track__dispose__set_error!(e);
    return 0;
  }
}

bool _isOnAudioLevelAvailableProxy(Object arg0) {
  try {
    return _isOnAudioLevelAvailable!(arg0);
  } catch (e) {
    _media_stream_track__is_on_audio_level_available__set_error!(e);
    return false;
  }
}

void _onAudioLevelChangedProxy(Object arg0, Object arg1) {
  try {
    return _onAudioLevelChanged!(arg0, arg1);
  } catch (e) {
    _media_stream_track__on_audio_level_changed__set_error!(e);
    return;
  }
}
