import '../util/move_semantic.dart';
import 'media_manager.dart';
import 'room_handle.dart';

/// General library interface.
///
/// Responsible for managing shared transports, local media and room
/// initialization.
abstract class Jason {
  /// Returns a [MediaManagerHandle] to the `MediaManager` of this [Jason].
  MediaManagerHandle mediaManager();

  /// Creates a new `Room` and returns its [RoomHandle].
  RoomHandle initRoom();

  /// Closes the `Room` by the provided [RoomHandle].
  void closeRoom(@moveSemantics RoomHandle room);

  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free();
}
