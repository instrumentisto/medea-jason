import '../util/move_semantic.dart';
import 'media_manager.dart';
import 'room_handle.dart';

abstract class IJason {
  MediaManagerHandle mediaManager() {
    throw UnimplementedError();
  }

  /// Creates a new `Room` and returns its [RoomHandle].
  RoomHandle initRoom() {
    throw UnimplementedError();
  }

  /// Closes the `Room` by the provided [RoomHandle].
  void closeRoom(@moveSemantics RoomHandle room) {
    throw UnimplementedError();
  }

  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free() {
    throw UnimplementedError();
  }
}
