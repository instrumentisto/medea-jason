import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'codec_capability.g.dart' as bridge;

/// Registers [RtpCodecCapability] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    getSenderCodecCapabilities:
        Pointer.fromFunction(_getSenderCodecCapabilities),
    mimeType: Pointer.fromFunction(_mimeType),
  );
}

/// Returns available [RtpCodecCapability]s for [RtpSender].
Object _getSenderCodecCapabilities(int kind) {
  return () => RtpSender.getCapabilities(MediaKind.values[kind])
      .then((res) => res.codecs);
}

/// Returns [RtpCodecCapability.mimeType].
Pointer<Utf8> _mimeType(RtpCodecCapability codecCapability) {
  return codecCapability.mimeType.toNativeUtf8();
}
