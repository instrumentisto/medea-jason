import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:web_socket_channel/io.dart';

import 'transport.g.dart' as bridge;

/// Registers functions allowing Rust to manage Dart [IOWebSocketChannel]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    connect: Pointer.fromFunction(_connect),
    listen: Pointer.fromFunction(_listen),
    send: Pointer.fromFunction(_send),
    close: Pointer.fromFunction(_close),
  );
}

/// Connects to the provided [addr] and returns [IOWebSocketChannel] for it.
Object _connect(Pointer<Utf8> addr) {
  return IOWebSocketChannel.connect(Uri.parse(addr.toDartString()));
}

/// Subscribes on [IOWebSocketChannel.stream] with provided [onMessage] and
/// [onClose] callbacks.
void _listen(IOWebSocketChannel ws, Function onMessage, Function onClose) {
  ws.stream.listen((msg) {
    if (msg is String) {
      onMessage(msg);
    }
  }, onDone: () {
    onClose();
  });
}

/// Sends the provided [msg] to the provided [IOWebSocketChannel].
void _send(IOWebSocketChannel ws, Pointer<Utf8> msg) {
  var sendMsg = msg.toDartString();
  ws.sink.add(sendMsg);
}

/// Closes the provided [IOWebSocketChannel] connection with the provided
/// [closeCode] and [closeMsg].
void _close(IOWebSocketChannel ws, int closeCode, Pointer<Utf8> closeMsg) {
  ws.sink.close(closeCode, closeMsg.toDartString());
}
