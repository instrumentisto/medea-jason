import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'package:medea_jason/src/native/ffi/native_string.dart';
import 'ice_servers.g.dart' as bridge;

/// Registers `PeerConnection` ICE servers related functions in Rust.
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

/// Adds an [IceServer] with the provided data to the provided [List].
void _add(List servers, Pointer<Utf8> url, ForeignValue username,
    ForeignValue credentials) {
  var iceServer = webrtc.IceServer([url.nativeStringToDartString()],
      username.toDart(), credentials.toDart());
  servers.add(iceServer);
}
