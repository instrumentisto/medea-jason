import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';

import 'transport.g.dart' as bridge;

/// Registers functions allowing Rust to operate Dart [WebSocket]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    connect: Pointer.fromFunction(_connect),
    send: Pointer.fromFunction(_send),
    close: Pointer.fromFunction(_close),
  );
}

/// Connects to the provided [addr] and returns [WebSocket] for it.
///
/// Subscribes to the created [WebSocket] messages with the given [onMessage]
/// and [onClose] callbacks.
Object _connect(Pointer<Utf8> addr, Function onMessage, Function onClose) {
  return () async {
    var ws = await WebSocket.connect(addr.toDartString());
    ws.listen(
      (msg) {
        if (msg is String) {
          onMessage(msg);
        }
      },
      onDone: () {
        onClose(ws.closeCode);
      },
      cancelOnError: true,
    );
    return ws;
  };
}

/// Sends the provided [message] to the provided [WebSocket].
void _send(WebSocket ws, Pointer<Utf8> message) {
  ws.add(message.toDartString());
}

/// Closes the provided [WebSocket] connection with the provided
/// [closeCode] and [closeMsg].
void _close(WebSocket ws, int closeCode, Pointer<Utf8> closeMsg) {
  ws.close(closeCode, closeMsg.toDartString());
}
