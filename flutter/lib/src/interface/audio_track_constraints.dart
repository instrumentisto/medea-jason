import '/src/util/rust_handles_storage.dart';

/// Constraints applicable to audio tracks.
abstract class AudioTrackConstraints implements SyncPlatformHandle {
  /// Sets an exact [`deviceId`][1] constraint.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
  void deviceId(String deviceId);

  /// Sets an exact [autoGainControl][1] constraint.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-constrainboolean
  void exactAutoGainControl(bool autoGainControl);

  /// Sets an ideal [autoGainControl][1] constraint.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-constrainboolean
  void idealAutoGainControl(bool autoGainControl);
}
