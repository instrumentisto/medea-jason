import '../interface/reconnect_handle.dart';
import '../web/jason_wasm.dart' as wasm;
import '../util/move_semantic.dart';

class WebReconnectHandle extends ReconnectHandle {
  late wasm.ReconnectHandle obj;

  WebReconnectHandle(this.obj);

  @override
  Future<void> reconnectWithDelay(int delayMs) async {
    await obj.reconnect_with_delay(delayMs);
  }

  @override
  Future<void> reconnectWithBackoff(
      int startingDelayMs, double multiplier, int maxDelay,
      [int? maxElapsedTimeMs]) async {
    await obj.reconnect_with_backoff(
        startingDelayMs, multiplier, maxDelay, maxElapsedTimeMs);
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
