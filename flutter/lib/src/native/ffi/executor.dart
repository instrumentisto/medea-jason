import 'dart:ffi';
import 'dart:isolate';

typedef _ExecutorInitC = Void Function(Int64);
typedef _ExecutorInitDart = void Function(int);

typedef _ExecutorPollTaskC = Void Function(Pointer);
typedef _ExecutorPollTaskDart = void Function(Pointer);

/// Executor used to drive Rust futures.
///
/// It must be instantiated before calling any `async` Rust functions.
class Executor {
  /// Pointer to a Rust function used to initialize Rust side of this
  /// [Executor].
  final _ExecutorInitDart _loopInit;

  /// Pointer to a Rust function used to poll Rust futures.
  final _ExecutorPollTaskDart _taskPoll;

  /// [ReceivePort] used to receive commands for polling Rust futures.
  late ReceivePort _wakePort;

  /// Creates a new [Executor].
  ///
  /// Initializes Rust part of the [Executor], creates a [ReceivePort] that
  /// accepts commands to poll Rust futures.
  Executor(DynamicLibrary dylib)
    : _loopInit =
          dylib
              .lookup<NativeFunction<_ExecutorInitC>>('rust_executor_init')
              .asFunction(),
      _taskPoll =
          dylib
              .lookup<NativeFunction<_ExecutorPollTaskC>>(
                'rust_executor_poll_task',
              )
              .asFunction() {
    _wakePort = ReceivePort()..listen(_pollTask);
    _loopInit(_wakePort.sendPort.nativePort);
  }

  /// Polls a Rust future basing on the provided [message].
  void _pollTask(dynamic message) {
    final task = Pointer.fromAddress(message);
    _taskPoll(task);
  }
}
