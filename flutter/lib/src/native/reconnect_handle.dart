import 'dart:ffi';

import '../interface/reconnect_handle.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/foreign_value.dart';
import 'ffi/nullable_pointer.dart';
import 'jason.dart';

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

typedef _reconnect_with_delay_C = Handle Function(Pointer, Int64);
typedef _reconnect_with_delay_Dart = Object Function(Pointer, int);

typedef _reconnect_with_backoff_C = Handle Function(
    Pointer, Int64, Double, Int64, ForeignValue);
typedef _reconnect_with_backoff_Dart = Object Function(
    Pointer, int, double, int, ForeignValue);

final _free = dl.lookupFunction<_free_C, _free_Dart>('ReconnectHandle__free');

final _reconnect_with_delay =
    dl.lookupFunction<_reconnect_with_delay_C, _reconnect_with_delay_Dart>(
        'ReconnectHandle__reconnect_with_delay');

final _reconnect_with_backoff =
    dl.lookupFunction<_reconnect_with_backoff_C, _reconnect_with_backoff_Dart>(
        'ReconnectHandle__reconnect_with_backoff');

class NativeReconnectHandle extends ReconnectHandle {
  /// [Pointer] to the Rust struct backing this object.
  late NullablePointer ptr;

  /// Constructs a new [ReconnectHandle] backed by the Rust struct behind the
  /// provided [Pointer].
  NativeReconnectHandle(this.ptr) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<void> reconnectWithDelay(int delayMs) async {
    await (_reconnect_with_delay(ptr.getInnerPtr(), delayMs) as Future);
  }

  @override
  Future<void> reconnectWithBackoff(
      int startingDelayMs, double multiplier, int maxDelay,
      [int? maxElapsedTimeMs]) async {
    var maxElapsedTimeMs_arg = maxElapsedTimeMs == null
        ? ForeignValue.none()
        : ForeignValue.fromInt(maxElapsedTimeMs);

    try {
      await (_reconnect_with_backoff(ptr.getInnerPtr(), startingDelayMs,
          multiplier, maxDelay, maxElapsedTimeMs_arg.ref) as Future);
    } finally {
      maxElapsedTimeMs_arg.free();
    }
  }

  @moveSemantics
  @override
  void free() {
    if (!ptr.isFreed()) {
      RustHandlesStorage().removeHandle(this);
      _free(ptr.getInnerPtr());
      ptr.free();
    }
  }
}
