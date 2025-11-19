import '../interface/room_close_reason.dart';
import 'ffi/frb/frb.dart' as frb;

class NativeRoomCloseReason implements RoomCloseReason {
  /// Rust `flutter_rust_bridge` api representation.
  final frb.RoomCloseReason _closeReason;

  /// Constructs a new [RoomCloseReason] backed by the Rust struct behind the
  /// provided [frb.RoomCloseReason].
  NativeRoomCloseReason(this._closeReason);

  @override
  RoomCloseKind reason() {
    return _closeReason.reason;
  }

  @override
  bool isClosedByServer() {
    return _closeReason.isClosedByServer;
  }

  @override
  void free() {}
}
