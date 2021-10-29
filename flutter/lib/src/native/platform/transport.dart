import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:web_socket_channel/io.dart';

import '../jason.dart';

void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register_WebSocketRpcTransport__new')(
      Pointer.fromFunction<Handle Function(Pointer<Utf8>)>(newWs));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register_WebSocketRpcTransport__on_message')(
      Pointer.fromFunction<Void Function(Handle, Handle)>(listenWs));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register_WebSocketRpcTransport__send')(
      Pointer.fromFunction<Void Function(Handle, Pointer<Utf8>)>(sendWsMsg));

  // dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
  //         "register_WebSocketRpcTransport__on_close")(
  //     Pointer.fromFunction<Void Function(Handle, Pointer)>(listenClose));
}

Object newWs(Pointer<Utf8> addr) {
  return IOWebSocketChannel.connect(Uri.parse(addr.toDartString()));
}

final _callMessageListenerDart _callMessageListener =
dl.lookupFunction<_callMessageListenerC, _callMessageListenerDart>(
    'StringCallback__call');
typedef _callMessageListenerC = Pointer<Utf8> Function(Pointer, Pointer<Utf8>);
typedef _callMessageListenerDart = Pointer<Utf8> Function(
    Pointer, Pointer<Utf8>);

void listenWs(IOWebSocketChannel ws, Function callback) {
  ws.stream.listen((msg) {
    if (msg is String) {
      print('onMessage: $msg');
      callback(msg);
    }
  });
}

void listenClose(IOWebSocketChannel ws, Pointer listener) {
  ws.stream.listen((msg) {
    if (msg is String) {
      _callMessageListener(listener, msg.toNativeUtf8());
    }
  });
}

void sendWsMsg(IOWebSocketChannel ws, Pointer<Utf8> msg) {
  var sendMsg = msg.toDartString();
  print('sendind message: $sendMsg');
  ws.sink.add(sendMsg);
}
