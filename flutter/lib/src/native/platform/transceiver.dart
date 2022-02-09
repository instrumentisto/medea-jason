import 'dart:ffi';

import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'package:flutter_webrtc/src/model/transceiver_direction.dart';

import 'transceiver.g.dart' as bridge;

/// Registers [RTCRtpTransceiver] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    getDirection: Pointer.fromFunction(_getDirection),
    replaceTrack: Pointer.fromFunction(_replaceSendTrack),
    getSendTrack: Pointer.fromFunction(_getSendTrack),
    setSendTrackEnabled: Pointer.fromFunction(_setSendTrackEnabled),
    dropSender: Pointer.fromFunction(_dropSender),
    isStopped: Pointer.fromFunction(_isStopped, true),
    mid: Pointer.fromFunction(_mid),
    hasSendTrack: Pointer.fromFunction(_hasSendTrack, false),
    setDirection: Pointer.fromFunction(_setDirection),
  );
}

/// Sets [TransceiverDirection] of the provided [RTCRtpTransceiver] to the
/// provided one.
Object _setDirection(RtpTransceiver transceiver, int direction) {
  return () => transceiver.setDirection(TransceiverDirection.values[direction]);
}

/// Returns current [TransceiverDirection] of the provided [RTCRtpTransceiver].
Object _getDirection(RtpTransceiver transceiver) {
  return () => transceiver.getDirection().then((d) => d.index);
}

/// Returns current MID of the provided [RTCRtpTransceiver].
Pointer _mid(RtpTransceiver transceiver) {
  if (transceiver.mid != null) {
    return ForeignValue.fromString(transceiver.mid!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns current [RTCRtpTransceiver.sender]'s track of the provided
/// [RTCRtpTransceiver].
Pointer _getSendTrack(RtpTransceiver transceiver) {
  if (transceiver.sender.track != null) {
    return ForeignValue.fromHandle(transceiver.sender.track!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Indicates whether the provided [RTCRtpTransceiver]'s
/// [RTCRtpTransceiver.sender] has some [MediaStreamTrack].
bool _hasSendTrack(RtpTransceiver transceiver) {
  return transceiver.sender.track != null;
}

/// Replaces [RTCRtpTransceiver.sender]'s [MediaStreamTrack] of the provided
/// [RTCRtpTransceiver] with a provided [MediaStreamTrack].
Object _replaceSendTrack(RtpTransceiver transceiver, MediaStreamTrack track) {
  return () => transceiver.sender.setTrack(track);
}

/// Sets [MediaStreamTrack.enabled] status in the [RTCRtpTransceiver.sender] of
/// the provided [RTCRtpTransceiver].
void _setSendTrackEnabled(RtpTransceiver transceiver, bool enabled) {
  if (transceiver.sender.track != null) {
    transceiver.sender.track!.setEnabled(enabled);
  }
}

/// Drops [RTCRtpTransceiver.sender] of the provided [RTCRtpTransceiver].
Object _dropSender(RtpTransceiver transceiver) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  if (transceiver.sender.track == null) {
    return () => Future.value();
  } else {
    return () => transceiver.sender.track!.stop();
  }
}

/// Indicates whether the [RTCRtpTransceiver.sender]'s [MediaStreamTrack] is
/// stopped.
bool _isStopped(RtpTransceiver transceiver) {
  return transceiver.isStopped();
}
