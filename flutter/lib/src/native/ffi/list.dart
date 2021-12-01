import 'dart:ffi';

import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:medea_jason/src/native/ffi/box_handle.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

/// Registers functions allowing Rust to create Dart [List]s.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Array__get')(
      Pointer.fromFunction<Handle Function(Handle, Int32)>(get));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Array__length')(
      Pointer.fromFunction<Int32 Function(Handle)>(len, 0));
}

var elements = [];

/// Returns [Pointer] to an object with a provided index.
Object get(List arr, int i) {
  List foobar = [...arr];
  var el = foobar[i];
  if (el == null) {
    print("Nothing found in List");
    throw Exception();
    // return ForeignValue.none().intoRustOwned();
  } else {
    print("Get Handle with type: ${el.runtimeType}");
    if (el is MediaDeviceInfo) {
      print("MediaDeviceInfo: ${el.kind}");
      return MediaDeviceInfo(kind: el.kind, deviceId: el.deviceId, label: el.label, groupId: el.groupId);
    } else {
      return el;
    }
    // return el;
    // return ForeignValue.fromHandle(el).intoRustOwned();
  }
}

/// Returns length of the provided [List].
int len(List arr) {
  return arr.length;
}
