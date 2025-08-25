import '/src/util/rust_handles_storage.dart';
import 'enums.dart' show NoiseSuppressionLevel;

export 'enums.dart' show NoiseSuppressionLevel;

/// Constraints applicable to device audio tracks (microphone).
abstract class DeviceAudioTrackConstraints implements SyncPlatformHandle {
  /// Sets an exact [`deviceId`][1] constraint.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
  void deviceId(String deviceId);

  /// Sets an exact [autoGainControl][1] constraint.
  ///
  /// [1]: https://www.w3.org/TR/mediacapture-streams/#dfn-autogaincontrol
  void exactAutoGainControl(bool autoGainControl);

  /// Sets an ideal [autoGainControl][1] constraint.
  ///
  /// [1]: https://www.w3.org/TR/mediacapture-streams/#dfn-autogaincontrol
  void idealAutoGainControl(bool autoGainControl);

  /// Sets an exact [echoCancellation][1] constraint.
  ///
  /// [1]: https://www.w3.org/TR/mediacapture-streams/#dfn-echocancellation
  void exactEchoCancellation(bool echoCancellation);

  /// Sets an ideal [echoCancellation][1] constraint.
  ///
  /// [1]: https://www.w3.org/TR/mediacapture-streams/#dfn-echocancellation
  void idealEchoCancellation(bool echoCancellation);

  /// Sets an exact [noiseSuppression][1] constraint.
  ///
  /// [1]: https://www.w3.org/TR/mediacapture-streams/#dfn-noisesuppression
  void exactNoiseSuppression(bool noiseSuppression);

  /// Sets an ideal [noiseSuppression][1] constraint.
  ///
  /// [1]: https://www.w3.org/TR/mediacapture-streams/#dfn-noisesuppression
  void idealNoiseSuppression(bool noiseSuppression);

  /// Constrains [`NoiseSuppressionLevel`].
  ///
  /// __NOTE__: Only supported on desktop platforms, ignored otherwise.
  void noiseSuppressionLevel(NoiseSuppressionLevel noiseSuppressionLevel);

  /// Sets an exact high-pass filter constraint.
  ///
  /// __NOTE__: Only supported on desktop platforms, ignored otherwise.
  void exactHighPassFilter(bool autoGainControl);

  /// Sets an ideal high-pass filter constraint.
  ///
  /// __NOTE__: Only supported on desktop platforms, ignored otherwise.
  void idealHighPassFilter(bool autoGainControl);
}

/// Constraints applicable to display audio tracks (system audio capture).
/// Display audio does not support audio processing features.
abstract class DisplayAudioTrackConstraints implements SyncPlatformHandle {
  // No methods - display audio doesn't support audio processing
}
