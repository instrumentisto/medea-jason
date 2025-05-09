import 'dart:ffi';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'constraints.g.dart' as bridge;

/// Registers functions allowing Rust to operate Dart
/// [MediaStreamConstraints][0].
///
/// [0]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    initDeviceConstraints: _newDeviceConstraints,
    initDisplayConstraints: _newDisplayConstraints,
    newVideoConstraints: _newVideoConstraints,
    newAudioConstraints: _newAudioConstraints,
    setVideoConstraintValue: _setVideoConstraintValue,
    setAudioConstraintValue: _setAudioConstraintValue,
    setVideoConstraint: _setVideoConstraint,
    setAudioConstraint: _setAudioConstraint,
    setDisplayVideoConstraint: _setDisplayVideoConstraint,
  );
}

/// Kind of a [MediaStreamConstraints.video][0] setting.
///
/// [0]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints-video
enum VideoConstraintKind { facingMode, deviceId, width, height, frameRate }

/// Kind of a [MediaStreamConstraints.audio][0] setting.
///
/// [0]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints-audio
enum AudioConstraintKind {
  deviceId,
  autoGainControl,
  noiseSuppression,
  noiseSuppressionLevel,
  highPassFilter,
  echoCancellation,
}

/// Indicates necessity of a [AudioConstraints] or [VideoConstraints] setting.
///
/// [0]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
enum ConstraintType { optional, mandatory }

/// Returns new empty [DeviceConstraints].
webrtc.DeviceConstraints _newDeviceConstraints() {
  return webrtc.DeviceConstraints();
}

///Returns new empty [DisplayConstraints].
webrtc.DisplayConstraints _newDisplayConstraints() {
  return webrtc.DisplayConstraints();
}

/// Returns new empty [DeviceVideoConstraints].
webrtc.DeviceVideoConstraints _newVideoConstraints() {
  return webrtc.DeviceVideoConstraints();
}

/// Returns new empty [AudioConstraints].
webrtc.AudioConstraints _newAudioConstraints() {
  return webrtc.AudioConstraints();
}

/// Specifies the provided setting of a [MediaStreamConstraints.video][0].
///
/// [0]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints-video
void _setVideoConstraintValue(Object cons, int kind, ForeignValue value) {
  cons as webrtc.DeviceVideoConstraints;

  switch (VideoConstraintKind.values[kind]) {
    case VideoConstraintKind.deviceId:
      cons.deviceId = value.toDart() as String;
      break;
    case VideoConstraintKind.facingMode:
      cons.facingMode = webrtc.FacingMode.values[value.toDart() as int];
      break;
    case VideoConstraintKind.width:
      cons.width = value.toDart();
      break;
    case VideoConstraintKind.height:
      cons.height = value.toDart();
      break;
    case VideoConstraintKind.frameRate:
      cons.fps = value.toDart();
      break;
  }
}

/// Specifies the provided setting of a [MediaStreamConstraints.audio][0].
///
/// [0]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints-audio
void _setAudioConstraintValue(Object cons, int kind, ForeignValue value) {
  cons as webrtc.AudioConstraints;

  switch (AudioConstraintKind.values[kind]) {
    case AudioConstraintKind.deviceId:
      cons.deviceId = value.toDart() as String;
    case AudioConstraintKind.autoGainControl:
      cons.autoGainControl = value.toDart() as bool;
    case AudioConstraintKind.noiseSuppression:
      cons.noiseSuppression = value.toDart() as bool;
    case AudioConstraintKind.noiseSuppressionLevel:
      cons.noiseSuppressionLevel =
          webrtc.NoiseSuppressionLevel.values[value.toDart() as int];
    case AudioConstraintKind.highPassFilter:
      cons.highPassFilter = value.toDart() as bool;
    case AudioConstraintKind.echoCancellation:
      cons.echoCancellation = value.toDart() as bool;
  }
}

/// Specifies the provided nature and settings of a video track to the given
/// [DeviceConstraints].
void _setVideoConstraint(Object cons, int type, Object video) {
  cons as webrtc.DeviceConstraints;
  video as webrtc.DeviceVideoConstraints;

  switch (ConstraintType.values[type]) {
    case ConstraintType.optional:
      cons.video.optional = video;
      break;
    case ConstraintType.mandatory:
      cons.video.mandatory = video;
      break;
  }
}

/// Specifies the provided nature and settings of a display video track to the
/// given [DisplayConstraints].
void _setDisplayVideoConstraint(Object cons, int type, Object video) {
  cons as webrtc.DisplayConstraints;
  video as webrtc.DeviceVideoConstraints;

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
void _setAudioConstraint(Object cons, int type, Object audio) {
  cons as webrtc.DeviceConstraints;
  audio as webrtc.AudioConstraints;

  switch (ConstraintType.values[type]) {
    case ConstraintType.optional:
      cons.audio.optional = audio;
      break;
    case ConstraintType.mandatory:
      cons.audio.mandatory = audio;
      break;
  }
}
