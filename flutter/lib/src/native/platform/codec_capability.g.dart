import 'dart:ffi';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function(int)? _getSenderCodecCapabilities;
Pointer<Utf8> Function(Object)? _mimeType;

_ErrorSetterFnDart? _codec_capability__get_sender_codec_capabilities__set_error;
_ErrorSetterFnDart? _codec_capability__mime_type__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Object Function(int) getSenderCodecCapabilities,
  required Pointer<Utf8> Function(Object) mimeType,
}) {
  _getSenderCodecCapabilities = getSenderCodecCapabilities;
  _mimeType = mimeType;

  _codec_capability__get_sender_codec_capabilities__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'codec_capability__get_sender_codec_capabilities__set_error');
  _codec_capability__mime_type__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'codec_capability__mime_type__set_error');

  Pointer<NativeFunction<Handle Function(Int64)>>
      getSenderCodecCapabilities_native = Pointer.fromFunction(
    _getSenderCodecCapabilitiesProxy,
  );
  Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> mimeType_native =
      Pointer.fromFunction(
    _mimeTypeProxy,
  );

  dl.lookupFunction<Void Function(Pointer, Pointer),
      void Function(Pointer, Pointer)>('register_codec_capability')(
    getSenderCodecCapabilities_native,
    mimeType_native,
  );
}

Object _getSenderCodecCapabilitiesProxy(int a) {
  try {
    return _getSenderCodecCapabilities!(a);
  } catch (e) {
    _codec_capability__get_sender_codec_capabilities__set_error!(e);
    return 0;
  }
}

Pointer<Utf8> _mimeTypeProxy(Object a) {
  try {
    return _mimeType!(a);
  } catch (e) {
    _codec_capability__mime_type__set_error!(e);
    return Pointer.fromAddress(0);
  }
}
