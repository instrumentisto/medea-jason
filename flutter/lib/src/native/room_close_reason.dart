import '../interface/room_close_reason.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';

class NativeRoomCloseReason extends RoomCloseReason {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late RustOpaque<frb.RoomCloseReason> opaque;

  /// Constructs a new [RoomCloseReason] backed by the Rust struct behind the
  /// provided [frb.RoomCloseReason].
  NativeRoomCloseReason(frb.RoomCloseReason roomCloseReason)
      : opaque = RustOpaque(roomCloseReason) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String reason() {
    return api.roomCloseReasonReason(roomCloseReason: opaque.innerOpaque);
  }

  @override
  bool isClosedByServer() {
    return api.roomCloseReasonIsClosedByServer(
        roomCloseReason: opaque.innerOpaque);
  }

  @override
  bool isErr() {
    return api.roomCloseReasonIsErr(roomCloseReason: opaque.innerOpaque);
  }

  @moveSemantics
  @override
  void free() {
    if (!opaque.isStale()) {
      RustHandlesStorage().removeHandle(this);

      opaque.dispose();
    }
  }
}
