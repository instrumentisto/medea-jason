import 'package:medea_jason/src/util/rust_handles_storage.dart';

import '../util/move_semantic.dart';

/// Constraints applicable to video tracks sourced from a screen capturing.
abstract class DisplayVideoTrackConstraints implements FreeableHandle {
  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free();
}
