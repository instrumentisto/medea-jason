import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

/// Registers [RTCPeerConnection] ICE servers related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_IceServers__new')(
      Pointer.fromFunction<Handle Function()>(newIceServers));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
      'register_IceServers__add')(Pointer.fromFunction<
          Void Function(Handle, Pointer<Utf8>, ForeignValue, ForeignValue)>(
      addIceServer));
}

Object newIceServers() {
  return List.empty(growable: true);
}

void addIceServer(Object servers, Pointer<Utf8> url, ForeignValue username,
    ForeignValue credentials) {
  servers as List;
  var iceServer = {'url': url.toDartString()};
  username = username.toDart();
  if (username is String) {
    iceServer['username'] = username as String;
  }
  credentials = credentials.toDart();
  if (credentials is String) {
    iceServer['credentials'] = credentials as String;
  }
  servers.add(iceServer);
}
