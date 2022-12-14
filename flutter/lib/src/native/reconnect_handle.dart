import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import '../interface/reconnect_handle.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';

class NativeReconnectHandle extends ReconnectHandle {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late frb.ReconnectHandle opaque;

  /// Constructs a new [ReconnectHandle] backed by the Rust struct behind the
  /// provided [frb.ReconnectHandle].
  NativeReconnectHandle(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<void> reconnectWithDelay(int delayMs) async {
    try {
      await (api.reconnectHandleReconnectWithDelay(
          reconnectHandle: opaque, delayMs: delayMs) as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> reconnectWithBackoff(
      int startingDelayMs, double multiplier, int maxDelay,
      [int? maxElapsedTimeMs]) async {
    try {
      await (api.reconnectHandleReconnectWithBackoff(
          reconnectHandle: opaque,
          startingDelay: startingDelayMs,
          multiplier: multiplier,
          maxDelay: maxDelay,
          maxElapsedTimeMs: maxElapsedTimeMs) as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
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
