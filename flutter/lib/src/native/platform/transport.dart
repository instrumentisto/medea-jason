import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';

import 'transport.g.dart' as bridge;

/// Registers functions allowing Rust to operate Dart [IOWebSocketChannel]s.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    connect: Pointer.fromFunction(_connect),
    listen: Pointer.fromFunction(_listen),
    send: Pointer.fromFunction(_send),
    close: Pointer.fromFunction(_close),
  );
}

/// Wrapper around [WebSocket] which stores WS messages if no one
/// currently listens to them and provides it to the first subscriber.
class BufferedWebSocket {
  /// Native WebSocket client without buffering support.
  late WebSocket _ws;

  /// Storage for the all messages which are can't be sent to the subscriber
  /// (because it null).
  final List<String> _wsBuffer = [];

  /// Callback into which all [WebSocket] messages will be provided.
  Function? _onMessage;

  /// Callback which will be closed when underlying [WebSocket] will be closed.
  Function? _onClose;

  /// Constructs new [BufferedWebSocket] based on the provided [WebSocket].
  BufferedWebSocket(WebSocket ws) {
    ws.listen(
      (msg) {
        if (_onMessage == null) {
          _wsBuffer.add(msg);
        } else {
          _onMessage!(msg);
        }
      },
      onDone: () {
        if (_onClose != null) {
          _onClose!();
        }
      },
      cancelOnError: true,
    );
    _ws = ws;
  }

  /// Subscribes on [WebSocket] messages.
  ///
  /// If [BufferedWebSocket] doesn't have previous [onMessage] listener
  /// then all messages received before, will be passed to the provided
  /// [onMessage] listener.
  ///
  /// Subscribes on [WebSocket] closing event.
  ///
  /// If underlying [WebSocket] is already closed then [onClose] callback
  /// will be called instantly.
  void listen(Function onMessage, Function onClose) {
    for (var bufferedMsg in _wsBuffer) {
      onMessage(bufferedMsg);
    }
    _wsBuffer.clear();

    if (_ws.readyState == WebSocket.closed) {
      onClose();
    }

    _onMessage = onMessage;
    _onClose = onClose;
  }

  /// Sends provided [message] to the server using underlying [WebSocket] client.
  void send(String message) {
    _ws.add(message);
  }

  /// Closes underlying [WebSocket] client with a provided [closeCode] and [closeMsg].
  void close(int closeCode, String closeMsg) {
    _ws.close(closeCode, closeMsg);
  }
}

/// Connects to the provided [addr] and returns [BufferedWebSocket] for it.
Object _connect(Pointer<Utf8> addr) {
  var url = addr.toDartString();
  return () async {
    var ws = await WebSocket.connect(url);
    return BufferedWebSocket(ws);
  };
}

/// Subscribes on WebSocket messages with provided [BufferedWebSocket] and
/// [onClose] callbacks.
void _listen(BufferedWebSocket ws, Function onMessage, Function onClose) {
  ws.listen(onMessage, onClose);
}

/// Sends the provided [message] to the provided [BufferedWebSocket].
void _send(BufferedWebSocket ws, Pointer<Utf8> message) {
  ws.send(message.toDartString());
}

/// Closes the provided [BufferedWebSocket] connection with the provided
/// [closeCode] and [closeMsg].
void _close(BufferedWebSocket ws, int closeCode, Pointer<Utf8> closeMsg) {
  ws.close(closeCode, closeMsg.toDartString());
}
