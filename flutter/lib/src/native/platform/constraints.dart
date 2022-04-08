import 'dart:ffi';

import 'package:flutter_webrtc/src/model/constraints.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

import 'constraints.g.dart' as bridge;

/// Registers functions allowing Rust to operate Dart
/// [MediaStreamConstraints][0].
///
/// [0]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    initDeviceConstraints: Pointer.fromFunction(_newDeviceConstraints),
    initDisplayConstraints: Pointer.fromFunction(_newDisplayConstraints),
    newVideoConstraints: Pointer.fromFunction(_newVideoConstraints),
    newAudioConstraints: Pointer.fromFunction(_newAudioConstraints),
    setVideoConstraintValue: Pointer.fromFunction(_setVideoConstraintValue),
    setAudioConstraintValue: Pointer.fromFunction(_setAudioConstraintValue),
    setVideoConstraint: Pointer.fromFunction(_setVideoConstraint),
    setAudioConstraint: Pointer.fromFunction(_setAudioConstraint),
  );
}

/// Kind of a [MediaStreamConstraints.video][0] setting.
///
/// [0]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints-video
enum VideoConstraintKind {
  facingMode,
  deviceId,
  width,
  height,
}

/// Kind of a [MediaStreamConstraints.audio][0] setting.
///
/// [0]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints-audio
enum AudioConstraintKind {
  deviceId,
}

/// Indicates necessity of a [AudioConstraints] or [VideoConstraints] setting.
///
/// [0]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
enum ConstraintType {
  optional,
  mandatory,
}

/// Returns new empty [DeviceConstraints].
Object _newDeviceConstraints() {
  return DeviceConstraints();
}

///Returns new empty [DisplayConstraints].
Object _newDisplayConstraints() {
  return DisplayConstraints();
}

/// Returns new empty [DeviceVideoConstraints].
Object _newVideoConstraints() {
  return DeviceVideoConstraints();
}

/// Returns new empty [AudioConstraints].
Object _newAudioConstraints() {
  return AudioConstraints();
}

/// Specifies the provided setting of a [MediaStreamConstraints.video][0].
///
/// [0]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints-video
void _setVideoConstraintValue(
    DeviceVideoConstraints cons, int kind, ForeignValue value) {
  switch (VideoConstraintKind.values[kind]) {
    case VideoConstraintKind.deviceId:
      cons.deviceId = value.toDart() as String;
      break;
    case VideoConstraintKind.facingMode:
      cons.facingMode = FacingMode.values[value.toDart() as int];
      break;
    case VideoConstraintKind.width:
      cons.width = value.toDart();
      break;
    case VideoConstraintKind.height:
      cons.height = value.toDart();
      break;
  }
}

/// Specifies the provided setting of a [MediaStreamConstraints.audio][0].
///
/// [0]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints-audio
void _setAudioConstraintValue(
    AudioConstraints cons, int kind, ForeignValue value) {
  switch (AudioConstraintKind.values[kind]) {
    case AudioConstraintKind.deviceId:
      cons.deviceId = value.toDart() as String;
      break;
  }
}

/// Specifies the provided nature and settings of a video track to the given
/// [DeviceConstraints].
void _setVideoConstraint(
    DeviceConstraints cons, int type, DeviceVideoConstraints video) {
  switch (ConstraintType.values[type]) {
    case ConstraintType.optional:
      cons.video.optional = video;
      break;
    case ConstraintType.mandatory:
      cons.video.mandatory = video;
      break;
  }
}

/// Specifies the provided nature and settings of a audio track to the given
/// [DeviceConstraints].
void _setAudioConstraint(
    DeviceConstraints cons, int type, AudioConstraints audio) {
  switch (ConstraintType.values[type]) {
    case ConstraintType.optional:
      cons.audio.optional = audio;
      break;
    case ConstraintType.mandatory:
      cons.audio.mandatory = audio;
      break;
  }
}
