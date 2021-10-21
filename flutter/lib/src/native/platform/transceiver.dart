import 'dart:ffi';

import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__current_direction')(
      Pointer.fromFunction<ForeignValue Function(Handle)>(currentDirection));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__get_current_direction')(
      Pointer.fromFunction<Handle Function(Handle)>(getCurrentDirection));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__replace_track')(
      Pointer.fromFunction<Handle Function(Handle, Handle)>(replaceSendTrack));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__get_send_track')(
      Pointer.fromFunction<ForeignValue Function(Handle)>(getSendTrack));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__set_send_track_enabled')(
      Pointer.fromFunction<Handle Function(Handle, Int8)>(setSendTrackEnabled));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__drop_sender')(
      Pointer.fromFunction<Void Function(Handle)>(dropSender));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__is_stopped')(
      Pointer.fromFunction<ForeignValue Function(Handle)>(isStopped));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__mid')(
      Pointer.fromFunction<ForeignValue Function(Handle)>(mid));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__send_track')(
      Pointer.fromFunction<ForeignValue Function(Handle)>(sendTrack));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__has_send_track')(
      Pointer.fromFunction<Int8 Function(Handle)>(hasSendTrack, 0));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Transceiver__set_direction')(
      Pointer.fromFunction<Handle Function(Handle, Int32)>(setDirection));
}

Object setDirection(RTCRtpTransceiver transceiver, int direction) {
    return transceiver.setDirection(TransceiverDirection.values[direction]);
}

Object getCurrentDirection(RTCRtpTransceiver transceiver) {
  return transceiver.getCurrentDirection().then((d) => ForeignValue.fromInt(d!.index).ref);
}

ForeignValue currentDirection(RTCRtpTransceiver transceiver) {
  if (transceiver.currentDirection != null) {
    var curDir = transceiver.currentDirection!;
    return ForeignValue.fromInt(curDir.index).ref;
  } else {
    return ForeignValue.none().ref;
  }
}

ForeignValue mid(RTCRtpTransceiver transceiver) {
  if (transceiver.mid != null) {
    return ForeignValue.fromString(transceiver.mid).ref;
  } else {
    return ForeignValue.none().ref;
  }
}

ForeignValue sendTrack(RTCRtpTransceiver transceiver) {
  if (transceiver.sender.track != null) {
    return ForeignValue.fromHandle(transceiver.sender.track!).ref;
  } else {
    return ForeignValue.none().ref;
  }
}

ForeignValue getSendTrack(Object transceiver) {
  transceiver = transceiver as RTCRtpTransceiver;
  if (transceiver.sender.track != null) {
    return ForeignValue.fromHandle(transceiver.sender.track!).ref;
  } else {
    return ForeignValue.none().ref;
  }
}

int hasSendTrack(RTCRtpTransceiver transceiver) {
  if (transceiver.sender.track == null) {
    return 0;
  } else {
    return 1;
  }
}

Object replaceSendTrack(RTCRtpTransceiver transceiver, MediaStreamTrack track) async {
  await transceiver.sender.setTrack(track);
  return ForeignValue.none().ref;
}

void setSendTrackEnabled(RTCRtpTransceiver transceiver, int enabled) {
  if (transceiver.sender.track != null) {
    transceiver.sender.track!.enabled = enabled == 1;
  }
}

void dropSender(Object transceiver) {
  if (transceiver is RTCRtpTransceiver) {
    // TODO:
    // transceiver.sender.setTrack(null);
  }
}

ForeignValue isStopped(RTCRtpTransceiver transceiver) {
  if (transceiver.sender.track != null &&
      transceiver.sender.track!.muted != null) {
    return ForeignValue.fromInt(transceiver.sender.track!.muted! ? 1 : 0).ref;
  } else {
    return ForeignValue.none().ref;
  }
}
