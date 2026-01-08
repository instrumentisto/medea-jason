import 'dart:ffi';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart';

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'transceiver.g.dart' as bridge;

/// Registers an [RtpTransceiver] related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    getDirection: _getDirection,
    replaceTrack: _replaceSendTrack,
    dropSender: _dropSender,
    isStopped: _isStopped,
    mid: _mid,
    setRecv: _setRecv,
    setSend: _setSend,
    dispose: _dispose,
    createTransceiverInit: _createTransceiverInit,
    addSendingEncodings: _addSendingEncodings,
    getSendParameters: _getSendParameters,
    setSendParameters: _setSendParameters,
    setCodecPreferences: _setCodecPreferences,
  );
}

/// Creates a new [RtpTransceiverInit].
RtpTransceiverInit _createTransceiverInit(int direction) {
  return RtpTransceiverInit(TransceiverDirection.values[direction]);
}

/// Adds [SendEncodingParameters] to the provided
/// [RtpTransceiverInit.sendEncodings].
void _addSendingEncodings(Object init, Object encoding) {
  init as RtpTransceiverInit;
  encoding as SendEncodingParameters;
  init.sendEncodings.add(encoding);
}

/// Changes the receive direction of the provided [RtpTransceiver].
Future<void> Function() _setRecv(Object transceiver, bool active) {
  transceiver as RtpTransceiver;
  return () => transceiver.setRecv(active);
}

/// Changes the send direction of the provided [RtpTransceiver].
Future<void> Function() _setSend(Object transceiver, bool active) {
  transceiver as RtpTransceiver;
  return () => transceiver.setSend(active);
}

/// Returns the current [TransceiverDirection] of the provided [RtpTransceiver].
Future<int> Function() _getDirection(Object transceiver) {
  transceiver as RtpTransceiver;
  return () => transceiver.getDirection().then((d) => d.index);
}

/// Returns the current mID of the provided [RtpTransceiver].
Pointer _mid(Object transceiver) {
  transceiver as RtpTransceiver;
  if (transceiver.mid != null) {
    return ForeignValue.fromString(transceiver.mid!).intoRustOwned();
  } else {
    return ForeignValue.none().intoRustOwned();
  }
}

/// Replaces [RtpTransceiver.sender]'s [MediaStreamTrack] of the provided
/// [RtpTransceiver] with a provided [MediaStreamTrack].
Future<void> Function() _replaceSendTrack(Object transceiver, Object track) {
  transceiver as RtpTransceiver;
  track as MediaStreamTrack;
  return () => transceiver.sender.replaceTrack(track);
}

/// Drops the [RtpTransceiver.sender] of the provided [RtpTransceiver].
Future Function() _dropSender(Object transceiver) {
  transceiver as RtpTransceiver;
  if (transceiver.sender.track != null) {
    return () => transceiver.sender.replaceTrack(null);
  } else {
    return () => Future.value();
  }
}

/// Indicates whether the [RtpTransceiver.sender]'s [MediaStreamTrack] is
/// stopped.
bool _isStopped(Object transceiver) {
  transceiver as RtpTransceiver;
  return transceiver.isStopped();
}

/// Disposes of this [RtpTransceiver].
Future<void> Function() _dispose(Object transceiver) {
  transceiver as RtpTransceiver;
  return () => transceiver.dispose();
}

/// Returns [RtpParameters] from the provided [RtpTransceiver.sender].
Future<RtpParameters> Function() _getSendParameters(Object transceiver) {
  transceiver as RtpTransceiver;
  return () => transceiver.sender.getParameters();
}

/// Sets [RtpParameters] into the provided [RtpTransceiver.sender].
Future<void> Function() _setSendParameters(
  Object transceiver,
  Object parameters,
) {
  transceiver as RtpTransceiver;
  parameters as RtpParameters;
  return () => transceiver.sender.setParameters(parameters);
}

/// Sets the provided [RtpCodecCapability] as the only preferred
/// [RtpCodecCapability] for the provided [RtpTransceiver].
void _setCodecPreferences(Object transceiver, Object codecCapability) {
  transceiver as RtpTransceiver;
  codecCapability as List;
  transceiver.setCodecPreferences(codecCapability.cast<RtpCodecCapability>());
}
