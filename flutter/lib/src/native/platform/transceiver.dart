import 'dart:ffi';

import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

import 'transceiver.g.dart' as bridge;

/// Registers [RTCRtpTransceiver] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    getCurrentDirection: getCurrentDirection,
    replaceTrack: replaceSendTrack,
    getSendTrack: getSendTrack,
    setSendTrackEnabled: setSendTrackEnabled,
    dropSender: dropSender,
    isStopped: isStopped,
    mid: mid,
    hasSendTrack: hasSendTrack,
    setDirection: setDirection,
  );
}

/// Sets [TransceiverDirection] of the provided [RTCRtpTransceiver] to the
/// provided one.
Object setDirection(Object transceiver, int direction) {
  transceiver as RTCRtpTransceiver;
  return transceiver.setDirection(TransceiverDirection.values[direction]);
}

/// Returns current [TransceiverDirection] of the provided [RTCRtpTransceiver].
Object getCurrentDirection(Object transceiver) {
  transceiver as RTCRtpTransceiver;
  return transceiver.getCurrentDirection().then((d) => d?.index);
}

/// Returns current MID of the provided [RTCRtpTransceiver].
Pointer mid(Object transceiver) {
  transceiver as RTCRtpTransceiver;
  if (transceiver.mid.isNotEmpty) {
    return ForeignValue.fromString(transceiver.mid).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns current [RTCRtpTransceiver.sender]'s track of the provided
/// [RTCRtpTransceiver].
Pointer getSendTrack(Object transceiver) {
  transceiver as RTCRtpTransceiver;
  if (transceiver.sender.track != null) {
    return ForeignValue.fromHandle(transceiver.sender.track!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns `1` if provided [RTCRtpTransceiver]'s [RTCRtpTransceiver.sender]
/// has some [MediaStreamTrack].
int hasSendTrack(Object transceiver) {
  transceiver as RTCRtpTransceiver;
  if (transceiver.sender.track == null) {
    return 0;
  } else {
    return 1;
  }
}

/// Replaces [RTCRtpTransceiver.sender]'s [MediaStreamTrack] of the provided
/// [RTCRtpTransceiver] with a provided [MediaStreamTrack].
Object replaceSendTrack(
    Object transceiver, Object track) async {
  transceiver as RTCRtpTransceiver;
  track as MediaStreamTrack;
  await transceiver.sender.setTrack(track);
  return ForeignValue.none().ref;
}

/// Sets [MediaStreamTrack.enabled] status in the [RTCRtpTransceiver.sender] of
/// the provided [RTCRtpTransceiver].
void setSendTrackEnabled(Object transceiver, int enabled) {
  transceiver as RTCRtpTransceiver;
  if (transceiver.sender.track != null) {
    transceiver.sender.track!.enabled = enabled == 1;
  }
}

/// Drops [RTCRtpTransceiver.sender] of the provided [RTCRtpTransceiver].
Object dropSender(Object transceiver) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  transceiver as RTCRtpTransceiver;
  if (transceiver.sender.track == null) {
    return () => Future.value();
  } else {
    return () => transceiver.sender.track!.stop();
  }
}

/// Returns `1` if [RTCRtpTransceiver.sender]'s [MediaStreamTrack] is stopped.
///
/// Returns [ForeignValue.none] if [RTCRtpTransceiver.sender] is `null`.
Pointer isStopped(Object transceiver) {
  transceiver as RTCRtpTransceiver;
  if (transceiver.sender.track != null &&
      transceiver.sender.track!.muted != null) {
    return ForeignValue.fromInt(transceiver.sender.track!.muted! ? 1 : 0)
        .intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}
