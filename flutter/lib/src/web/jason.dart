import '../interface/jason.dart';
import '../interface/media_manager.dart';
import '../interface/room_handle.dart';
import '../web/jason_wasm.dart' as wasm;
import '../web/exceptions.dart';
import '../util/move_semantic.dart';
import '../web/media_manager.dart';
import '../web/room_handle.dart';

class Jason extends IJason {
  final wasm.Jason obj = wasm.Jason();

  @override
  MediaManagerHandle mediaManager() {
    return failableFunction(() => WebMediaManagerHandle(obj.media_manager()));
  }

  @override
  RoomHandle initRoom() {
    return failableFunction(() => WebRoomHandle(obj.init_room()));
  }

  @override
  void closeRoom(@moveSemantics RoomHandle room) {
    failableFunction(() => obj.close_room((room as WebRoomHandle).obj));
  }

  @override
  @moveSemantics
  void free() {
    obj.free();
  }
}
