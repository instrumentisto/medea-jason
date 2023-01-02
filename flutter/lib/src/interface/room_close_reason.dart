import '/src/util/rust_handles_storage.dart';

/// Reason of why a `Room` has been closed.
///
/// This struct is passed into the `RoomHandle.onClose()` callback.
abstract class RoomCloseReason extends SyncPlatformHandle {
  /// Returns a close reason of the `Room`.
  String reason();

  /// Indicates whether the `Room` was closed by server.
  bool isClosedByServer();

  /// Indicates whether the `Room`'s close reason is considered as an error.
  bool isErr();
}
