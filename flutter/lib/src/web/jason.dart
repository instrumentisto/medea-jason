import '../interface/jason.dart' as base;
import '../interface/media_manager.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;
import 'media_manager.dart';
import 'room_handle.dart';

class Jason extends base.Jason {
  final wasm.Jason obj = wasm.Jason();

  /// Creates a new instance of [Jason].
  static Future<Jason> init() async {
    return Jason._();
  }

  Jason._();

  @override
  MediaManagerHandle mediaManager() {
    return fallibleFunction(() => WebMediaManagerHandle(obj.media_manager()));
  }

  @override
  RoomHandle initRoom() {
    return fallibleFunction(() => WebRoomHandle(obj.init_room()));
  }

  @override
  void closeRoom(@moveSemantics RoomHandle room) {
    fallibleFunction(() => obj.close_room((room as WebRoomHandle).obj));
  }

  @override
  @moveSemantics
  void free() {
    obj.free();
  }
}
