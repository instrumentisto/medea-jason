import '../util/move_semantic.dart';

/// Constraints applicable to audio tracks.
abstract class IAudioTrackConstraints {
  /// Sets an exact [`deviceId`][1] constraint.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
  void deviceId(String deviceId);

  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free();
}
