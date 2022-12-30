import '../interface/room_close_reason.dart';
import '../util/move_semantic.dart';
import 'ffi/jason_api.g.dart' as frb;

class NativeRoomCloseReason extends RoomCloseReason {
  /// Rust `flutter_rust_bridge` api representation.
  final frb.RoomCloseReason _closeReason;

  /// Constructs a new [RoomCloseReason] backed by the Rust struct behind the
  /// provided [frb.RoomCloseReason].
  NativeRoomCloseReason(this._closeReason);

  @override
  String reason() {
    return _closeReason.reason;
  }

  @override
  bool isClosedByServer() {
    return _closeReason.isClosedByServer;
  }

  @override
  bool isErr() {
    return _closeReason.isErr;
  }

  @moveSemantics
  @override
  void free() {}
}
