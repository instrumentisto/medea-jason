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

/// Returns new empty `IceServer`s [List].
Object newIceServers() {
  return List.empty(growable: true);
}

/// Adds `IceServer` with a provided data to the provided [List].
void addIceServer(List servers, Pointer<Utf8> url, ForeignValue username,
    ForeignValue credentials) {
  var iceServer = {'url': url.toDartString()};
  var usernameString = username.toDart();
  if (usernameString is String) {
    iceServer['username'] = usernameString;
  }
  var credentialsString = credentials.toDart();
  if (credentialsString is String) {
    iceServer['credential'] = credentialsString;
  }
  servers.add(iceServer);
}
