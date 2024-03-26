import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'codec_capability.g.dart' as bridge;

void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    getSenderCodecCapabilities:
        Pointer.fromFunction(_getSenderCodecCapabilities),
    mimeType: Pointer.fromFunction(_mimeType),
  );
}

Object _getSenderCodecCapabilities(int kind) {
  return () => RtpSender.getCapabilities(MediaKind.values[kind])
      .then((res) => res.codecs);
}

Pointer<Utf8> _mimeType(RtpCodecCapability codecCapability) {
  return codecCapability.mimeType.toNativeUtf8();
}
