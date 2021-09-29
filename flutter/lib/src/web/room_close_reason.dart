import '../interface/room_close_reason.dart';
import '../web/jason_wasm.dart' as wasm;
import '../util/move_semantic.dart';

class WebRoomCloseReason extends RoomCloseReason {
  late wasm.RoomCloseReason obj;

  WebRoomCloseReason(this.obj);

  @override
  String reason() {
    return obj.reason();
  }

  @override
  bool isClosedByServer() {
    return obj.is_closed_by_server();
  }

  @override
  bool isErr() {
    return obj.is_err();
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
