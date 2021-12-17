import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:web_socket_channel/io.dart';

import 'transport.g.dart' as bridge;

/// Registers functions allowing Rust to manage Dart [IOWebSocketChannel]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    init: Pointer.fromFunction(_init),
    onMessage: Pointer.fromFunction(_onMessage),
    send: Pointer.fromFunction(_send),
    close: Pointer.fromFunction(_close),
  );
}

void _close(IOWebSocketChannel ws, int closeCode, Pointer<Utf8> msg) {
  ws.sink.close(closeCode, msg.toDartString());
}

/// Connects to the provided `addr` and returns [IOWebSocketChannel] for it.
Object _init(Pointer<Utf8> addr) {
  return IOWebSocketChannel.connect(Uri.parse(addr.toDartString()));
}

/// Subscribes on [IOWebSocketChannel.stream] with provided `onMessage` [Function] and `onClose` [Function].
void _onMessage(IOWebSocketChannel ws, Function onMessage, Function onClose) {
  ws.stream.listen((msg) {
    if (msg is String) {
      onMessage(msg);
    }
  }, onDone: () {
    onClose();
  });
}

/// Sends provided `msg` to the provided [IOWebSocketChannel].
void _send(IOWebSocketChannel ws, Pointer<Utf8> msg) {
  var sendMsg = msg.toDartString();
  ws.sink.add(sendMsg);
}
