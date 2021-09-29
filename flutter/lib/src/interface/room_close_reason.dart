import '../util/move_semantic.dart';

/// Reason of why a `Room` has been closed.
///
/// This struct is passed into the `RoomHandle.onClose()` callback.
abstract class RoomCloseReason {
  /// Returns a close reason of the `Room`.
  String reason() {
    throw UnimplementedError();
  }

  /// Indicates whether the `Room` was closed by server.
  bool isClosedByServer() {
    throw UnimplementedError();
  }

  /// Indicates whether the `Room`'s close reason is considered as an error.
  bool isErr() {
    throw UnimplementedError();
  }

  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free() {
    throw UnimplementedError();
  }
}
