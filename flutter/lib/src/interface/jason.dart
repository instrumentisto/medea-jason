import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'media_manager.dart';
import 'room_handle.dart';

/// General library interface.
///
/// Responsible for managing shared transports, local media and room
/// initialization.
abstract class Jason implements SyncPlatformHandle {
  /// Returns a [MediaManagerHandle] to the `MediaManager` of this [Jason].
  MediaManagerHandle mediaManager();

  /// Creates a new `Room` and returns its [RoomHandle].
  RoomHandle initRoom();

  /// Closes the `Room` by the provided [RoomHandle].
  void closeRoom(@moveSemantics RoomHandle room);

  /// Notifies this [Jason] about a network change event (interface switch or
  /// similar).
  ///
  /// Drops and recreates active connections and schedules [ICE] restart after
  /// reconnection.
  ///
  /// Throws an [RpcClientException] if the reconnection attempt fails.
  ///
  /// [ICE]: https://webrtcglossary.com/ice
  Future<void> networkChanged();
}
