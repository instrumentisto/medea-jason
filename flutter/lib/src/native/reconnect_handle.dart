import 'dart:ffi';

import '../interface/reconnect_handle.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as api;
import 'jason.dart';

class NativeReconnectHandle extends ReconnectHandle {
  /// [Pointer] to the Rust struct backing this object.
  late api.ReconnectHandle opaque;

  /// Constructs a new [ReconnectHandle] backed by the Rust struct behind the
  /// provided [Pointer].

  NativeReconnectHandle.opaque(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<void> reconnectWithDelay(int delayMs) async {
    await rust2dart(impl_api.reconnectHandleReconnectWithDelay(
        reconnectHandle: opaque, delayMs: delayMs));
  }

  @override
  Future<void> reconnectWithBackoff(
      int startingDelayMs, double multiplier, int maxDelay,
      [int? maxElapsedTimeMs]) async {
    await rust2dart(impl_api.reconnectHandleReconnectWithBackoff(
        reconnectHandle: opaque,
        startingDelay: startingDelayMs,
        multiplier: multiplier,
        maxDelay: maxDelay,
        maxElapsedTimeMs: maxElapsedTimeMs));
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
