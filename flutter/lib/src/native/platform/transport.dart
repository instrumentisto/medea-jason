import 'dart:collection';
import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';

import 'package:medea_jason/src/native/ffi/native_string.dart';
import 'transport.g.dart' as bridge;

/// Option to mock a [WebSocket].
const bool mockable = bool.fromEnvironment('MOCKABLE', defaultValue: false);

/// Registers functions allowing Rust to operate Dart [WebSocket]s.
void registerFunctions(DynamicLibrary dl) {
  if (mockable) {
    bridge.registerFunction(
      dl,
      connect: Pointer.fromFunction(MockWebSocket.connect),
      send: Pointer.fromFunction(_send),
      close: Pointer.fromFunction(_close),
      closeCode: Pointer.fromFunction(_closeCode, 0),
      closeReason: Pointer.fromFunction(_closeReason),
    );
  } else {
    bridge.registerFunction(
      dl,
      connect: Pointer.fromFunction(_connect),
      send: Pointer.fromFunction(_send),
      close: Pointer.fromFunction(_close),
      closeCode: Pointer.fromFunction(_closeCode, 0),
      closeReason: Pointer.fromFunction(_closeReason),
    );
  }
}

/// [Close frame][1], sent to clients when a [WebSocket] connection is closed
/// normally.
///
/// [1]: https://tools.ietf.org/html/rfc6455#section-5.5.1
class CloseFrame {
  /// Close code sent by the server.
  int? code;

  /// Reason why the server closed the connection.
  String? reason;

  CloseFrame(this.code, this.reason);
}

/// Provider to mock a [WebSocket].
///
/// [mockable] must be `true`.
class MockWebSocket {
  /// Safe last created [WebSocket].
  static late WebSocket _lastWebSocket;

  /// Stored [WebSocket]s for outside access.
  static final _allWebSocket = HashMap<String, WebSocket>();

  /// Connects to the provided [addr] and returns a [WebSocket] for it.
  ///
  /// Subscribes to the created [WebSocket] messages with the specified
  /// [onMessage] and [onClose] callbacks.
  static Object connect(
      Pointer<Utf8> addr, Object onMessage, Object onClose) {
    onMessage as Function;
    onClose as Function;
    return () async {
      var ws = await WebSocket.connect(addr.nativeStringToDartString());
      _lastWebSocket = ws;

      ws.listen(
        (msg) {
          if (msg is String) {
            onMessage(msg);
          }
        },
        onDone: () {
          onClose(CloseFrame(ws.closeCode, ws.closeReason));
        },
        cancelOnError: true,
      );
      return ws;
    };
  }

  /// Saves [WebSocket] for the specified [member].
  static void addMember(String member) {
    _allWebSocket.addAll({member: _lastWebSocket});
  }

  /// Returns the [WebSocket] of the provided [member].
  static WebSocket? getSocket(String member) {
    return _allWebSocket[member];
  }
}

/// Connects to the provided [addr] and returns [WebSocket] for it.
///
/// Subscribes to the created [WebSocket] messages with the given [onMessage]
/// and [onClose] callbacks.
Object _connect(Pointer<Utf8> addr, Object onMessage, Object onClose) {
  onMessage as Function;
  onClose as Function;
  return () async {
    var ws = await WebSocket.connect(addr.nativeStringToDartString());
    ws.listen(
      (msg) {
        if (msg is String) {
          onMessage(msg);
        }
      },
      onDone: () {
        onClose(CloseFrame(ws.closeCode, ws.closeReason));
      },
      cancelOnError: true,
    );
    return ws;
  };
}

/// Sends the provided [message] to the provided [WebSocket].
void _send(Object ws, Pointer<Utf8> message) {
  ws as WebSocket;
  ws.add(message.nativeStringToDartString());
}

/// Closes the provided [WebSocket] connection with the provided
/// [closeCode] and [closeMsg].
void _close(Object ws, int closeCode, Pointer<Utf8> closeMsg) {
  ws as WebSocket;
  ws.close(closeCode, closeMsg.nativeStringToDartString());
}

/// Returns [CloseFrame.code] of the provided [CloseFrame].
int _closeCode(Object closeFrame) {
  closeFrame as CloseFrame;
  return closeFrame.code ?? 1005;
}

/// Returns [CloseFrame.reason] of the provided [CloseFrame].
Pointer<Utf8> _closeReason(Object closeFrame) {
  closeFrame as CloseFrame;
  return (closeFrame.reason ?? '').toNativeUtf8();
}
