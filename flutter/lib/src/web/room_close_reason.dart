import '../interface/room_close_reason.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class WebRoomCloseReason implements RoomCloseReason {
  late wasm.RoomCloseReason obj;

  WebRoomCloseReason(this.obj);

  @override
  RoomCloseKind reason() {
    return fallibleFunction(() => RoomCloseKind.values[obj.reason().toInt()]);
  }

  @override
  bool isClosedByServer() {
    return fallibleFunction(() => obj.is_closed_by_server());
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
