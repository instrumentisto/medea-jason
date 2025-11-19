import '/src/util/rust_handles_storage.dart';
import 'enums.dart' show RoomCloseKind;

export 'enums.dart' show RoomCloseKind;

/// Reason of why a `Room` has been closed.
///
/// This struct is passed into the `RoomHandle.onClose()` callback.
abstract class RoomCloseReason implements SyncPlatformHandle {
  /// Returns a close reason of the `Room`.
  RoomCloseKind reason();

  /// Indicates whether the `Room` was closed by server.
  bool isClosedByServer();
}
