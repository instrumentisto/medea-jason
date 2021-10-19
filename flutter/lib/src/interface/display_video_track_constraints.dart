import '../util/move_semantic.dart';

/// Constraints applicable to video tracks sourced from a screen capturing.
abstract class DisplayVideoTrackConstraints {
  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free();
}
