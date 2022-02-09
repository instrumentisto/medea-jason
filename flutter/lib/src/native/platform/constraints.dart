import 'dart:ffi';

import 'package:flutter_webrtc/src/model/constraints.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

import 'constraints.g.dart' as bridge;

/// Registers functions allowing Rust to operate Dart [MediaStreamConstraints].
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    init: Pointer.fromFunction(_new),
    newVideoConstraints: Pointer.fromFunction(_newVideoConstraints),
    newAudioConstraints: Pointer.fromFunction(_newAudioConstraints),
    setVideoConstraintValue: Pointer.fromFunction(_setVideoConstraintValue),
    setAudioConstraintValue: Pointer.fromFunction(_setAudioConstraintValue),
    setVideoConstraint: Pointer.fromFunction(_setVideoConstraint),
    setAudioConstraint: Pointer.fromFunction(_setAudioConstraint),
  );
}

/// Kind of [MediaStreamConstraints.video][0] setting.
///
/// [0]: https://www.w3.org/TR/mediacapture-streams/#dom-mediastreamconstraints-video
enum VideoConstraintKind {
  facingMode,
  deviceId,
}

/// Kind of [MediaStreamConstraints.audio][0] setting.
///
/// [0]: https://www.w3.org/TR/mediacapture-streams/#dom-mediastreamconstraints-audio
enum AudioConstraintKind {
  deviceId,
}

/// Indicator of necessity of [AudioConstraints] or [VideoConstraints] setting.
///
/// [0]: https://www.w3.org/TR/mediacapture-streams/#dom-mediastreamconstraints
enum ConstraintType {
  optional,
  mandatory,
}

/// Returns empty [Constraints].
Object _new() {
  return Constraints();
}

/// Returns new empty [VideoConstraints].
Object _newVideoConstraints() {
  return VideoConstraints();
}

/// Returns new empty [AudioConstraints].
Object _newAudioConstraints() {
  return AudioConstraints();
}

/// Specifies setting of the [MediaStreamConstraints.video][0] (for example `facingMode`).
///
/// [0]: https://www.w3.org/TR/mediacapture-streams/#dom-mediastreamconstraints-video
void _setVideoConstraintValue(
    VideoConstraints cons, int kind, ForeignValue value) {
  switch (VideoConstraintKind.values[kind]) {
    case VideoConstraintKind.deviceId:
      cons.deviceId = value.toDart() as String;
      break;
    case VideoConstraintKind.facingMode:
      cons.facingMode = FacingMode.values[value.toDart() as int];
      break;
  }
}

/// Specifies setting of the [MediaStreamConstraints.audio][0] (for example `deviceId`).
///
/// [0]: https://www.w3.org/TR/mediacapture-streams/#dom-mediastreamconstraints-audio
void _setAudioConstraintValue(
    AudioConstraints cons, int kind, ForeignValue value) {
  switch (AudioConstraintKind.values[kind]) {
    case AudioConstraintKind.deviceId:
      cons.deviceId = value.toDart() as String;
      break;
  }
}

/// Specifies the provided nature and settings of a video track to the given [Constraints].
void _setVideoConstraint(Constraints cons, int type, VideoConstraints video) {
  switch (ConstraintType.values[type]) {
    case ConstraintType.optional:
      cons.video.optional = video;
      break;
    case ConstraintType.mandatory:
      cons.video.mandatory = video;
      break;
  }
}

/// Specifies the provided nature and settings of a audio track to the given [Constraints].
void _setAudioConstraint(Constraints cons, int type, AudioConstraints audio) {
  switch (ConstraintType.values[type]) {
    case ConstraintType.optional:
      cons.audio.optional = audio;
      break;
    case ConstraintType.mandatory:
      cons.audio.mandatory = audio;
      break;
  }
}
