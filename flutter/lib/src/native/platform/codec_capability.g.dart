import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function(int)? _getSenderCodecCapabilities;
Object Function(int)? _getReceiverCodecCapabilities;
Pointer<Utf8> Function(Object)? _mimeType;
Pointer Function(Object)? _clockRate;
Pointer Function(Object)? _channels;
Pointer<Utf8> Function(Object)? _parameters;

_ErrorSetterFnDart? _codec_capability__get_sender_codec_capabilities__set_error;
_ErrorSetterFnDart?
    _codec_capability__get_receiver_codec_capabilities__set_error;
_ErrorSetterFnDart? _codec_capability__mime_type__set_error;
_ErrorSetterFnDart? _codec_capability__clock_rate__set_error;
_ErrorSetterFnDart? _codec_capability__channels__set_error;
_ErrorSetterFnDart? _codec_capability__parameters__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Object Function(int) getSenderCodecCapabilities,
  required Object Function(int) getReceiverCodecCapabilities,
  required Pointer<Utf8> Function(Object) mimeType,
  required Pointer Function(Object) clockRate,
  required Pointer Function(Object) channels,
  required Pointer<Utf8> Function(Object) parameters,
}) {
  _getSenderCodecCapabilities = getSenderCodecCapabilities;
  _getReceiverCodecCapabilities = getReceiverCodecCapabilities;
  _mimeType = mimeType;
  _clockRate = clockRate;
  _channels = channels;
  _parameters = parameters;

  _codec_capability__get_sender_codec_capabilities__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'codec_capability__get_sender_codec_capabilities__set_error',
  );
  _codec_capability__get_receiver_codec_capabilities__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'codec_capability__get_receiver_codec_capabilities__set_error',
  );
  _codec_capability__mime_type__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'codec_capability__mime_type__set_error',
  );
  _codec_capability__clock_rate__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'codec_capability__clock_rate__set_error',
  );
  _codec_capability__channels__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'codec_capability__channels__set_error',
  );
  _codec_capability__parameters__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
    'codec_capability__parameters__set_error',
  );

  Pointer<NativeFunction<Handle Function(Int64)>>
      getSenderCodecCapabilities_native = Pointer.fromFunction(
    _getSenderCodecCapabilitiesProxy,
  );
  Pointer<NativeFunction<Handle Function(Int64)>>
      getReceiverCodecCapabilities_native = Pointer.fromFunction(
    _getReceiverCodecCapabilitiesProxy,
  );
  Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> mimeType_native =
      Pointer.fromFunction(_mimeTypeProxy);
  Pointer<NativeFunction<Pointer Function(Handle)>> clockRate_native =
      Pointer.fromFunction(_clockRateProxy);
  Pointer<NativeFunction<Pointer Function(Handle)>> channels_native =
      Pointer.fromFunction(_channelsProxy);
  Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> parameters_native =
      Pointer.fromFunction(_parametersProxy);

  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer)>('register_codec_capability')(
    getSenderCodecCapabilities_native,
    getReceiverCodecCapabilities_native,
    mimeType_native,
    clockRate_native,
    channels_native,
    parameters_native,
  );
}

Object _getSenderCodecCapabilitiesProxy(int arg0) {
  try {
    return _getSenderCodecCapabilities!(arg0);
  } catch (e) {
    _codec_capability__get_sender_codec_capabilities__set_error!(e);
    return 0;
  }
}

Object _getReceiverCodecCapabilitiesProxy(int arg0) {
  try {
    return _getReceiverCodecCapabilities!(arg0);
  } catch (e) {
    _codec_capability__get_receiver_codec_capabilities__set_error!(e);
    return 0;
  }
}

Pointer<Utf8> _mimeTypeProxy(Object arg0) {
  try {
    return _mimeType!(arg0);
  } catch (e) {
    _codec_capability__mime_type__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

Pointer _clockRateProxy(Object arg0) {
  try {
    return _clockRate!(arg0);
  } catch (e) {
    _codec_capability__clock_rate__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

Pointer _channelsProxy(Object arg0) {
  try {
    return _channels!(arg0);
  } catch (e) {
    _codec_capability__channels__set_error!(e);
    return Pointer.fromAddress(0);
  }
}

Pointer<Utf8> _parametersProxy(Object arg0) {
  try {
    return _parameters!(arg0);
  } catch (e) {
    _codec_capability__parameters__set_error!(e);
    return Pointer.fromAddress(0);
  }
}
