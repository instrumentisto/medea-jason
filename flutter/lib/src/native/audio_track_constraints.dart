import '../interface/audio_track_constraints.dart' as base;
import 'ffi/frb/frb.dart' as frb;
import 'ffi/frb/media/constraints.dart';

class AudioTrackConstraints implements base.AudioTrackConstraints {
  /// Rust `flutter_rust_bridge` API representation.
  final frb.ApiAudioConstraints constraints = frb.ApiAudioConstraints(
    deviceId: null,
  );

  @override
  void deviceId(String deviceId) {
    constraints.deviceId = deviceId;
  }

  @override
  void exactAutoGainControl(bool autoGainControl) {
    constraints.autoGainControl = ConstrainBoolean.exact(autoGainControl);
  }

  @override
  void idealAutoGainControl(bool autoGainControl) {
    constraints.autoGainControl = ConstrainBoolean.ideal(autoGainControl);
  }

  @override
  void exactEchoCancellation(bool echoCancellation) {
    constraints.echoCancellation = ConstrainBoolean.exact(echoCancellation);
  }

  @override
  void idealEchoCancellation(bool echoCancellation) {
    constraints.echoCancellation = ConstrainBoolean.ideal(echoCancellation);
  }

  @override
  void exactNoiseSuppression(bool noiseSuppression) {
    constraints.noiseSuppression = ConstrainBoolean.exact(noiseSuppression);
  }

  @override
  void idealNoiseSuppression(bool noiseSuppression) {
    constraints.noiseSuppression = ConstrainBoolean.ideal(noiseSuppression);
  }

  @override
  void noiseSuppressionLevel(NoiseSuppressionLevel noiseSuppressionLevel) {
    constraints.noiseSuppressionLevel = noiseSuppressionLevel;
  }

  @override
  void idealHighPassFilter(bool highPassFilter) {
    constraints.highPassFilter = ConstrainBoolean.ideal(highPassFilter);
  }

  @override
  void exactHighPassFilter(bool highPassFilter) {
    constraints.highPassFilter = ConstrainBoolean.exact(highPassFilter);
  }

  @override
  void free() {}
}
