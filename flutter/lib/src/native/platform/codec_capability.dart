import 'dart:convert';
import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'codec_capability.g.dart' as bridge;

/// Registers [RtpCodecCapability] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    getSenderCodecCapabilities: _getSenderCodecCapabilities,
    getReceiverCodecCapabilities: _getReceiverCodecCapabilities,
    mimeType: _mimeType,
    clockRate: _clockRate,
    channels: _channels,
    parameters: _parameters,
  );
}

/// Returns available [RtpCodecCapability]s for an [RtpSender].
Future<List<RtpCodecCapability>> Function() _getSenderCodecCapabilities(
    int kind) {
  return () => RtpSender.getCapabilities(MediaKind.values[kind])
      .then((res) => res.codecs);
}

/// Returns available [RtpCodecCapability]s for an [RtpReceiver].
Future<List<RtpCodecCapability>> Function() _getReceiverCodecCapabilities(
    int kind) {
  return () => RtpReceiver.getCapabilities(MediaKind.values[kind])
      .then((res) => res.codecs);
}

/// Returns [RtpCodecCapability.mimeType].
Pointer<Utf8> _mimeType(Object codecCapability) {
  codecCapability as RtpCodecCapability;

  return codecCapability.mimeType.toNativeUtf8();
}

/// Returns [RtpCodecCapability.clockRate].
Pointer _clockRate(Object codecCapability) {
  codecCapability as RtpCodecCapability;

  if (codecCapability.clockRate != null) {
    return ForeignValue.fromInt(codecCapability.clockRate!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns [RtpCodecCapability.numChannels].
Pointer _channels(Object codecCapability) {
  codecCapability as RtpCodecCapability;

  if (codecCapability.numChannels != null) {
    return ForeignValue.fromInt(codecCapability.numChannels!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns [RtpCodecCapability.parameters].
Pointer<Utf8> _parameters(Object codecCapability) {
  codecCapability as RtpCodecCapability;

  return json.encode(codecCapability.parameters).toNativeUtf8();
}
