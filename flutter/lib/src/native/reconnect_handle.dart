import '../interface/reconnect_handle.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/frb/frb.dart' as frb;

class NativeReconnectHandle implements ReconnectHandle {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final RustOpaque<frb.ReconnectHandle> opaque;

  /// Constructs a new [ReconnectHandle] backed by the Rust struct behind the
  /// provided [frb.ReconnectHandle].
  NativeReconnectHandle(frb.ReconnectHandle reconnectHandle)
      : opaque = RustOpaque(reconnectHandle) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<void> reconnectWithDelay(int delayMs) async {
    if (delayMs.isNegative || delayMs.bitLength > 32) {
      throw ArgumentError.value(delayMs, 'delayMs', 'Expected `u32`');
    }

    await (opaque.inner.reconnectWithDelay(delayMs: delayMs) as Future);
  }

  @override
  Future<void> reconnectWithBackoff(
      int startingDelayMs, double multiplier, int maxDelay,
      [int? maxElapsedTimeMs]) async {
    if (startingDelayMs.isNegative || startingDelayMs.bitLength > 32) {
      throw ArgumentError.value(
          startingDelayMs, 'startingDelayMs', 'Expected `u32`');
    }

    if (maxDelay.isNegative || maxDelay.bitLength > 32) {
      throw ArgumentError.value(maxDelay, 'maxDelay', 'Expected `u32`');
    }

    if (maxElapsedTimeMs != null) {
      if (maxElapsedTimeMs.isNegative || maxElapsedTimeMs.bitLength > 32) {
        throw ArgumentError.value(
            maxElapsedTimeMs, 'maxElapsedTimeMs', 'Expected `u32`');
      }
    }

    await (opaque.inner.reconnectWithBackoff(
        startingDelay: startingDelayMs,
        multiplier: multiplier,
        maxDelay: maxDelay,
        maxElapsedTimeMs: maxElapsedTimeMs) as Future);
  }

  @moveSemantics
  @override
  void free() {
    if (!opaque.isDisposed) {
      RustHandlesStorage().removeHandle(this);

      opaque.dispose();
    }
  }
}
