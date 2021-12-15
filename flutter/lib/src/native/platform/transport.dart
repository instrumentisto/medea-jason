import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:web_socket_channel/io.dart';

import 'transport.g.dart' as bridge;

/// Registers functions allowing Rust to manage Dart [IOWebSocketChannel]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    init: Pointer.fromFunction(newWs),
    onMessage: Pointer.fromFunction(listenWs),
    send: Pointer.fromFunction(sendWsMsg),
    close: Pointer.fromFunction(close),
  );
}

void close(IOWebSocketChannel ws, int closeCode, Pointer<Utf8> msg) {
  ws.sink.close(closeCode, msg.toDartString());
}

/// Connects to the provided `addr` and returns [IOWebSocketChannel] for it.
Object newWs(Pointer<Utf8> addr) {
  return IOWebSocketChannel.connect(Uri.parse(addr.toDartString()));
}

/// Subscribes on [IOWebSocketChannel.stream] with provided `onMessage` [Function] and `onClose` [Function].
void listenWs(IOWebSocketChannel ws, Function onMessage, Function onClose) {
  ws.stream.listen((msg) {
    if (msg is String) {
      onMessage(msg);
    }
  }, onDone: () {
    //onClose();
  });
}

/// Sends provided `msg` to the provided [IOWebSocketChannel].
void sendWsMsg(IOWebSocketChannel ws, Pointer<Utf8> msg) {
  var sendMsg = msg.toDartString();
  ws.sink.add(sendMsg);
}
