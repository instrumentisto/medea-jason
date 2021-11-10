import 'dart:ffi';

import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

/// Registers [RTCRtpTransceiver] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__get_current_direction')(
      Pointer.fromFunction<Handle Function(Handle)>(getCurrentDirection));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__replace_track')(
      Pointer.fromFunction<Handle Function(Handle, Handle)>(replaceSendTrack));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__get_send_track')(
      Pointer.fromFunction<Pointer Function(Handle)>(getSendTrack));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__set_send_track_enabled')(
      Pointer.fromFunction<Handle Function(Handle, Int8)>(setSendTrackEnabled));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__drop_sender')(
      Pointer.fromFunction<Handle Function(Handle)>(dropSender));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__is_stopped')(
      Pointer.fromFunction<Pointer Function(Handle)>(isStopped));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__mid')(
      Pointer.fromFunction<Pointer Function(Handle)>(mid));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__has_send_track')(
      Pointer.fromFunction<Int8 Function(Handle)>(hasSendTrack, 0));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__set_direction')(
      Pointer.fromFunction<Handle Function(Handle, Int64)>(setDirection));
}

/// Sets [TransceiverDirection] of the provided [RTCRtpTransceiver] to the
/// provided one.
Object setDirection(RTCRtpTransceiver transceiver, int direction) {
  return transceiver.setDirection(TransceiverDirection.values[direction]);
}

/// Returns current [TransceiverDirection] of the provided [RTCRtpTransceiver].
Object getCurrentDirection(RTCRtpTransceiver transceiver) {
  return transceiver.getCurrentDirection().then((d) => d?.index);
}

/// Returns current MID of the provided [RTCRtpTransceiver].
Pointer mid(RTCRtpTransceiver transceiver) {
  if (transceiver.mid.isNotEmpty) {
    return ForeignValue.fromString(transceiver.mid).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns current [RTCRtpTransceiver.sender]'s track of the provided
/// [RTCRtpTransceiver].
Pointer getSendTrack(RTCRtpTransceiver transceiver) {
  if (transceiver.sender.track != null) {
    return ForeignValue.fromHandle(transceiver.sender.track!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns `1` if provided [RTCRtpTransceiver]'s [RTCRtpTransceiver.sender]
/// has some [MediaStreamTrack].
int hasSendTrack(RTCRtpTransceiver transceiver) {
  if (transceiver.sender.track == null) {
    return 0;
  } else {
    return 1;
  }
}

/// Replaces [RTCRtpTransceiver.sender]'s [MediaStreamTrack] of the provided
/// [RTCRtpTransceiver] with a provided [MediaStreamTrack].
Object replaceSendTrack(
    RTCRtpTransceiver transceiver, MediaStreamTrack track) async {
  await transceiver.sender.setTrack(track);
  return ForeignValue.none().ref;
}

/// Sets [MediaStreamTrack.enabled] status in the [RTCRtpTransceiver.sender] of
/// the provided [RTCRtpTransceiver].
void setSendTrackEnabled(RTCRtpTransceiver transceiver, int enabled) {
  if (transceiver.sender.track != null) {
    transceiver.sender.track!.enabled = enabled == 1;
  }
}

/// Drops [RTCRtpTransceiver.sender] of the provided [RTCRtpTransceiver].
Object dropSender(RTCRtpTransceiver transceiver) {
  // TODO: Correct implementation requires flutter_webrtc-side fixes.
  if (transceiver.sender.track == null) {
    return () => Future.value();
  } else {
    return () => transceiver.sender.track!.stop();
  }
}

/// Returns `1` if [RTCRtpTransceiver.sender]'s [MediaStreamTrack] is stopped.
///
/// Returns [ForeignValue.none] if [RTCRtpTransceiver.sender] is `null`.
Pointer isStopped(RTCRtpTransceiver transceiver) {
  if (transceiver.sender.track != null &&
      transceiver.sender.track!.muted != null) {
    return ForeignValue.fromInt(transceiver.sender.track!.muted! ? 1 : 0)
        .intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}
