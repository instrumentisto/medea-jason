import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

import 'ice_servers.g.dart' as bridge;

/// Registers [RTCPeerConnection] ICE servers related functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    init: Pointer.fromFunction(_new),
    add: Pointer.fromFunction(_add),
  );
}

/// Returns a new empty `IceServer`s [List].
Object _new() {
  return List.empty(growable: true);
}

/// Adds an `IceServer` with the provided data to the provided [List].
void _add(List servers, Pointer<Utf8> url, ForeignValue username,
    ForeignValue credentials) {
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
