import 'dart:ffi';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'transceiver.g.dart' as bridge;

/// Registers an [RtpTransceiver] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    getDirection: Pointer.fromFunction(_getDirection),
    replaceTrack: Pointer.fromFunction(_replaceSendTrack),
    getSendTrack: Pointer.fromFunction(_getSendTrack),
    dropSender: Pointer.fromFunction(_dropSender),
    isStopped: Pointer.fromFunction(_isStopped, true),
    mid: Pointer.fromFunction(_mid),
    setRecv: Pointer.fromFunction(_setRecv),
    setSend: Pointer.fromFunction(_setSend),
    dispose: Pointer.fromFunction(_dispose),
  );
}

/// Changes the receive direction of the provided [RtpTransceiver].
Object _setRecv(RtpTransceiver transceiver, bool active) {
  return () => transceiver.setRecv(active);
}

/// Changes the send direction of the provided [RtpTransceiver].
Object _setSend(RtpTransceiver transceiver, bool active) {
  return () => transceiver.setSend(active);
}

/// Returns the current [TransceiverDirection] of the provided [RtpTransceiver].
Object _getDirection(RtpTransceiver transceiver) {
  return () => transceiver.getDirection().then((d) => d.index);
}

/// Returns the current mID of the provided [RtpTransceiver].
Pointer _mid(RtpTransceiver transceiver) {
  if (transceiver.mid != null) {
    return ForeignValue.fromString(transceiver.mid!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Returns the current [RtpTransceiver.sender]'s track of the provided
/// [RtpTransceiver].
Pointer _getSendTrack(RtpTransceiver transceiver) {
  if (transceiver.sender.track != null) {
    return ForeignValue.fromHandle(transceiver.sender.track!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Replaces [RtpTransceiver.sender]'s [MediaStreamTrack] of the provided
/// [RtpTransceiver] with a provided [MediaStreamTrack].
Object _replaceSendTrack(RtpTransceiver transceiver, MediaStreamTrack track) {
  return () => transceiver.sender.replaceTrack(track);
}

/// Drops the [RtpTransceiver.sender] of the provided [RtpTransceiver].
Object _dropSender(RtpTransceiver transceiver) {
  if (transceiver.sender.track == null) {
    return () => transceiver.sender.replaceTrack(null);
  } else {
    return () => Future.value();
  }
}

/// Indicates whether the [RtpTransceiver.sender]'s [MediaStreamTrack] is
/// stopped.
bool _isStopped(RtpTransceiver transceiver) {
  return transceiver.isStopped();
}

/// Disposes of this [RtpTransceiver].
void _dispose(RtpTransceiver transceiver) {
  transceiver.dispose();
}
