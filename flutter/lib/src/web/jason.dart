import '../interface/jason.dart' as base;
import '../interface/media_manager.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import 'jason_wasm.dart' as wasm;
import 'media_manager.dart';
import 'room_handle.dart';

class Jason extends base.Jason {
  final wasm.Jason obj = wasm.Jason();

  @override
  MediaManagerHandle mediaManager() {
    return WebMediaManagerHandle(obj.media_manager());
  }

  @override
  RoomHandle initRoom() {
    return WebRoomHandle(obj.init_room());
  }

  @override
  void closeRoom(@moveSemantics RoomHandle room) {
    obj.close_room((room as WebRoomHandle).obj);
  }

  @override
  @moveSemantics
  void free() {
    obj.free();
  }
}
