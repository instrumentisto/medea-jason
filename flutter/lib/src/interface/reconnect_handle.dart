import '../util/move_semantic.dart';

/// External handle used to reconnect to a media server when connection is lost.
///
/// This handle is passed to the `RoomHandle.onConnectionLoss()` callback.
abstract class ReconnectHandle {
  /// Tries to reconnect a `Room` after the provided delay in milliseconds.
  ///
  /// If the `Room` is already reconnecting then new reconnection attempt won't
  /// be performed. Instead, it will wait for the first reconnection attempt
  /// result and use it here.
  ///
  /// Throws `RpcClientException` if reconnect attempt fails.
  ///
  /// Throws [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Converts the provided [delayMs] into an `u32`. Throws an [ArgumentError]
  /// if conversion fails.
  Future<void> reconnectWithDelay(int delayMs) {
    throw UnimplementedError();
  }

  /// Tries to reconnect a `Room` in a loop with a growing backoff delay.
  ///
  /// The first attempt will be performed immediately, and the second attempt
  /// will be performed after [starting_delay_ms].
  ///
  /// Delay between reconnection attempts won't be greater than [max_delay_ms].
  ///
  /// After each reconnection attempt, delay between reconnections will be
  /// multiplied by the given [multiplier] until it reaches [max_delay_ms].
  ///
  /// If [multiplier] is a negative number then it will be considered as `0.0`.
  /// This might cause a busy loop, so it's not recommended.
  ///
  /// Max elapsed time can be limited with an optional [maxElapsedTimeMs]
  /// argument.
  ///
  /// If the `Room` is already reconnecting then new reconnection attempt won't
  /// be performed. Instead, it will wait for the first reconnection attempt
  /// result and use it here.
  ///
  /// Throws `RpcClientException` if reconnect attempt fails.
  ///
  /// Throws [StateError] if the underlying [Pointer] has been freed.
  ///
  /// Converts the provided [startingDelayMs], [maxDelay] and [maxElapsedTimeMs]
  /// into an `u32`s. Throws an [ArgumentError] if any conversion fails.
  Future<void> reconnectWithBackoff(
      int startingDelayMs, double multiplier, int maxDelay,
      [int? maxElapsedTimeMs]) {
    throw UnimplementedError();
  }

  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free() {
    throw UnimplementedError();
  }
}
