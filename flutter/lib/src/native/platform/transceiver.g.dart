import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function(Object)? _getDirection;
Object Function(Object, Object)? _replaceTrack;
Object Function(Object)? _dropSender;
bool Function(Object)? _isStopped;
Pointer Function(Object)? _mid;
Object Function(Object, bool)? _setRecv;
Object Function(Object, bool)? _setSend;
Object Function(Object)? _dispose;
Object Function(int)? _createTransceiverInit;
void Function(Object, Object)? _addSendingEncodings;
Object Function(Object)? _getSendParameters;
Object Function(Object, Object)? _setSendParameters;
void Function(Object, Object)? _setCodecPreferences;

_ErrorSetterFnDart? _transceiver__get_direction__set_error;
_ErrorSetterFnDart? _transceiver__replace_track__set_error;
_ErrorSetterFnDart? _transceiver__drop_sender__set_error;
_ErrorSetterFnDart? _transceiver__is_stopped__set_error;
_ErrorSetterFnDart? _transceiver__mid__set_error;
_ErrorSetterFnDart? _transceiver__set_recv__set_error;
_ErrorSetterFnDart? _transceiver__set_send__set_error;
_ErrorSetterFnDart? _transceiver__dispose__set_error;
_ErrorSetterFnDart? _transceiver__create_transceiver_init__set_error;
_ErrorSetterFnDart? _transceiver__add_sending_encodings__set_error;
_ErrorSetterFnDart? _transceiver__get_send_parameters__set_error;
_ErrorSetterFnDart? _transceiver__set_send_parameters__set_error;
_ErrorSetterFnDart? _transceiver__set_codec_preferences__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Object Function(Object) getDirection,
  required Object Function(Object, Object) replaceTrack,
  required Object Function(Object) dropSender,
  required bool Function(Object) isStopped,
  required Pointer Function(Object) mid,
  required Object Function(Object, bool) setRecv,
  required Object Function(Object, bool) setSend,
  required Object Function(Object) dispose,
  required Object Function(int) createTransceiverInit,
  required void Function(Object, Object) addSendingEncodings,
  required Object Function(Object) getSendParameters,
  required Object Function(Object, Object) setSendParameters,
  required void Function(Object, Object) setCodecPreferences,
}) {
  _getDirection = getDirection;
  _replaceTrack = replaceTrack;
  _dropSender = dropSender;
  _isStopped = isStopped;
  _mid = mid;
  _setRecv = setRecv;
  _setSend = setSend;
  _dispose = dispose;
  _createTransceiverInit = createTransceiverInit;
  _addSendingEncodings = addSendingEncodings;
  _getSendParameters = getSendParameters;
  _setSendParameters = setSendParameters;
  _setCodecPreferences = setCodecPreferences;

  _transceiver__get_direction__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transceiver__get_direction__set_error');
  _transceiver__replace_track__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transceiver__replace_track__set_error');
  _transceiver__drop_sender__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transceiver__drop_sender__set_error');
  _transceiver__is_stopped__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transceiver__is_stopped__set_error');
  _transceiver__mid__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transceiver__mid__set_error');
  _transceiver__set_recv__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transceiver__set_recv__set_error');
  _transceiver__set_send__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transceiver__set_send__set_error');
  _transceiver__dispose__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transceiver__dispose__set_error');
  _transceiver__create_transceiver_init__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transceiver__create_transceiver_init__set_error');
  _transceiver__add_sending_encodings__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transceiver__add_sending_encodings__set_error');
  _transceiver__get_send_parameters__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transceiver__get_send_parameters__set_error');
  _transceiver__set_send_parameters__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transceiver__set_send_parameters__set_error');
  _transceiver__set_codec_preferences__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'transceiver__set_codec_preferences__set_error');

  Pointer<NativeFunction<Handle Function(Handle)>> getDirection_native =
      Pointer.fromFunction(
    _getDirectionProxy,
  );
  Pointer<NativeFunction<Handle Function(Handle, Handle)>> replaceTrack_native =
      Pointer.fromFunction(
    _replaceTrackProxy,
  );
  Pointer<NativeFunction<Handle Function(Handle)>> dropSender_native =
      Pointer.fromFunction(
    _dropSenderProxy,
  );
  Pointer<NativeFunction<Bool Function(Handle)>> isStopped_native =
      Pointer.fromFunction(_isStoppedProxy, false);
  Pointer<NativeFunction<Pointer Function(Handle)>> mid_native =
      Pointer.fromFunction(
    _midProxy,
  );
  Pointer<NativeFunction<Handle Function(Handle, Bool)>> setRecv_native =
      Pointer.fromFunction(
    _setRecvProxy,
  );
  Pointer<NativeFunction<Handle Function(Handle, Bool)>> setSend_native =
      Pointer.fromFunction(
    _setSendProxy,
  );
  Pointer<NativeFunction<Handle Function(Handle)>> dispose_native =
      Pointer.fromFunction(
    _disposeProxy,
  );
  Pointer<NativeFunction<Handle Function(Int64)>> createTransceiverInit_native =
      Pointer.fromFunction(
    _createTransceiverInitProxy,
  );
  Pointer<NativeFunction<Void Function(Handle, Handle)>>
      addSendingEncodings_native = Pointer.fromFunction(
    _addSendingEncodingsProxy,
  );
  Pointer<NativeFunction<Handle Function(Handle)>> getSendParameters_native =
      Pointer.fromFunction(
    _getSendParametersProxy,
  );
  Pointer<NativeFunction<Handle Function(Handle, Handle)>>
      setSendParameters_native = Pointer.fromFunction(
    _setSendParametersProxy,
  );
  Pointer<NativeFunction<Void Function(Handle, Handle)>>
      setCodecPreferences_native = Pointer.fromFunction(
    _setCodecPreferencesProxy,
  );

  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer),
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
          Pointer)>('register_transceiver')(
    getDirection_native,
    replaceTrack_native,
    dropSender_native,
    isStopped_native,
    mid_native,
    setRecv_native,
    setSend_native,
    dispose_native,
    createTransceiverInit_native,
    addSendingEncodings_native,
    getSendParameters_native,
    setSendParameters_native,
    setCodecPreferences_native,
  );
}

Object _getDirectionProxy(Object arg0) {
  try {
    return _getDirection!(arg0);
  } catch (e) {
    _transceiver__get_direction__set_error!(e);
    return 0;
  }
}

Object _replaceTrackProxy(Object arg0, Object arg1) {
  try {
    return _replaceTrack!(arg0, arg1);
  } catch (e) {
    _transceiver__replace_track__set_error!(e);
    return 0;
  }
}

Object _dropSenderProxy(Object arg0) {
  try {
    return _dropSender!(arg0);
  } catch (e) {
    _transceiver__drop_sender__set_error!(e);
    return 0;
  }
}

bool _isStoppedProxy(Object arg0) {
  try {
    return _isStopped!(arg0);
  } catch (e) {
    _transceiver__is_stopped__set_error!(e);
    return false;
  }
}

Pointer _midProxy(Object arg0) {
  try {
    return _mid!(arg0);
  } catch (e) {
    _transceiver__mid__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

Object _setRecvProxy(Object arg0, bool arg1) {
  try {
    return _setRecv!(arg0, arg1);
  } catch (e) {
    _transceiver__set_recv__set_error!(e);
    return 0;
  }
}

Object _setSendProxy(Object arg0, bool arg1) {
  try {
    return _setSend!(arg0, arg1);
  } catch (e) {
    _transceiver__set_send__set_error!(e);
    return 0;
  }
}

Object _disposeProxy(Object arg0) {
  try {
    return _dispose!(arg0);
  } catch (e) {
    _transceiver__dispose__set_error!(e);
    return 0;
  }
}

Object _createTransceiverInitProxy(int arg0) {
  try {
    return _createTransceiverInit!(arg0);
  } catch (e) {
    _transceiver__create_transceiver_init__set_error!(e);
    return 0;
  }
}

void _addSendingEncodingsProxy(Object arg0, Object arg1) {
  try {
    return _addSendingEncodings!(arg0, arg1);
  } catch (e) {
    _transceiver__add_sending_encodings__set_error!(e);
    return;
  }
}

Object _getSendParametersProxy(Object arg0) {
  try {
    return _getSendParameters!(arg0);
  } catch (e) {
    _transceiver__get_send_parameters__set_error!(e);
    return 0;
  }
}

Object _setSendParametersProxy(Object arg0, Object arg1) {
  try {
    return _setSendParameters!(arg0, arg1);
  } catch (e) {
    _transceiver__set_send_parameters__set_error!(e);
    return 0;
  }
}

void _setCodecPreferencesProxy(Object arg0, Object arg1) {
  try {
    return _setCodecPreferences!(arg0, arg1);
  } catch (e) {
    _transceiver__set_codec_preferences__set_error!(e);
    return;
  }
}
