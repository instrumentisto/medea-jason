import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'codec_capability.g.dart' as bridge;

/// Registers [RtpCodecCapability] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    getSenderCodecCapabilities: _getSenderCodecCapabilities,
    mimeType: _mimeType,
  );
}

/// Returns available [RtpCodecCapability]s for an [RtpSender].
Future<List<RtpCodecCapability>> Function() _getSenderCodecCapabilities(
    int kind) {
  return () => RtpSender.getCapabilities(MediaKind.values[kind])
      .then((res) => res.codecs);
}

/// Returns [RtpCodecCapability.mimeType].
Pointer<Utf8> _mimeType(Object codecCapability) {
  codecCapability as RtpCodecCapability;

  return codecCapability.mimeType.toNativeUtf8();
}
