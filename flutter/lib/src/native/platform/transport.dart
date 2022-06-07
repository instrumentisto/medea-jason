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
    closeCode: Pointer.fromFunction(_closeCode, 0),
    closeReason: Pointer.fromFunction(_closeReason),
  );
}

class LastFrame {
  /// A [WebSocket]'s `close code`.
  int? code;

  /// A [WebSocket]'s `close reason`.
  String? reason;

  LastFrame(this.code, this.reason);
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
        onClose(LastFrame(ws.closeCode, ws.closeReason));
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

/// Return [LastFrame.code].
int _closeCode(LastFrame lastFrame) {
  return lastFrame.code ?? 1000;
}

/// Return [LastFrame.reason].
Pointer<Utf8> _closeReason(LastFrame lastFrame) {
  return (lastFrame.reason ?? '').toNativeUtf8();
}
