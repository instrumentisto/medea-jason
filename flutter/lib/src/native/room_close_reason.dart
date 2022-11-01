import 'dart:ffi';

import '../interface/room_close_reason.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as api;
import 'jason.dart';

class NativeRoomCloseReason extends RoomCloseReason {
  /// [Pointer] to the Rust struct that backing this object.
  late api.RoomCloseReason opaque;

  /// Constructs a new [RoomCloseReason] backed by the Rust struct behind the
  /// provided [Pointer].

  NativeRoomCloseReason.opaque(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String reason() {
    return impl_api.roomCloseReasonReason(roomCloseReason: opaque);
  }

  @override
  bool isClosedByServer() {
    return impl_api.roomCloseReasonIsClosedByServer(roomCloseReason: opaque);
  }

  @override
  bool isErr() {
    return impl_api.roomCloseReasonIsErr(roomCloseReason: opaque);
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
