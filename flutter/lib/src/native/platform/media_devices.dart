import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';
import 'media_devices.g.dart' as bridge;
import 'package:flutter_webrtc/src/model/constraints.dart';

/// Registers functions allowing Rust to operate Dart [MediaDevices].
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    enumerateDevices: Pointer.fromFunction(_enumerateDevices),
    getUserMedia: Pointer.fromFunction(_getUserMedia),
    getDisplayMedia: Pointer.fromFunction(_getDisplayMedia),
  );
}

/// Requests media input access and returns the created [MediaStreamTrack]s.
Object _getUserMedia(Constraints constraints) {
  return () async {
    // var videoConstraints = {};
    // if (constraints.video != null && constraints.video['video'] != null) {
    //   videoConstraints = constraints.video['video'];
    // }
    var res = await getUserMedia(constraints);
    return res;
  };
}

/// Returns all the available media devices.
Object _enumerateDevices() {
  return () async {
    print("enum 1");
    var res = await enumerateDevices();
    print("enum 2");
    return res;
  };
}

/// Starts capturing the contents of a display and returns the created
/// [MediaStreamTrack]s.
Object _getDisplayMedia(Map<String, dynamic> constraints) {
  throw UnimplementedError(
      "getDisplayMedia currently isn't supported by flutter_webrtc");
  // return () => navigator.mediaDevices.getDisplayMedia(constraints);
}
