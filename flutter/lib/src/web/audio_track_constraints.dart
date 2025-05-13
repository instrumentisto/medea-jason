import '../interface/audio_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class AudioTrackConstraints implements base.AudioTrackConstraints {
  final wasm.AudioTrackConstraints obj = wasm.AudioTrackConstraints();

  @override
  void deviceId(String deviceId) {
    fallibleFunction(() => obj.device_id(deviceId));
  }

  @override
  void exactAutoGainControl(bool autoGainControl) {
    fallibleFunction(() => obj.exact_auto_gain_control(autoGainControl));
  }

  @override
  void idealAutoGainControl(bool autoGainControl) {
    fallibleFunction(() => obj.ideal_auto_gain_control(autoGainControl));
  }

  @override
  void idealEchoCancellation(bool echoCancellation) {
    fallibleFunction(() => obj.ideal_echo_cancellation(echoCancellation));
  }

  @override
  void exactEchoCancellation(bool echoCancellation) {
    fallibleFunction(() => obj.exact_echo_cancellation(echoCancellation));
  }

  @override
  void idealNoiseSuppression(bool noiseSuppression) {
    fallibleFunction(() => obj.ideal_noise_suppression(noiseSuppression));
  }

  @override
  void exactNoiseSuppression(bool noiseSuppression) {
    fallibleFunction(() => obj.exact_noise_suppression(noiseSuppression));
  }

  @override
  void exactHighPassFilter(bool autoGainControl) {
    // Not supported on web.
  }

  @override
  void idealHighPassFilter(bool autoGainControl) {
    // Not supported on web.
  }

  @override
  void noiseSuppressionLevel(base.NoiseSuppressionLevel noiseSuppressionLevel) {
    // Not supported on web.
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
