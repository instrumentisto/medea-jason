import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:web_socket_channel/io.dart';

/// Registers functions allowing Rust to manage Dart [IOWebSocketChannel]s.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_WebSocketRpcTransport__new')(
      Pointer.fromFunction<Handle Function(Pointer<Utf8>)>(newWs));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_WebSocketRpcTransport__listen_ws')(
      Pointer.fromFunction<Void Function(Handle, Handle, Handle)>(listenWs));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_WebSocketRpcTransport__send')(
      Pointer.fromFunction<Void Function(Handle, Pointer<Utf8>)>(sendWsMsg));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register_WebSocketRpcTransport__close')(
      Pointer.fromFunction<Void Function(Handle, Int32, Pointer<Utf8>)>(close));
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
