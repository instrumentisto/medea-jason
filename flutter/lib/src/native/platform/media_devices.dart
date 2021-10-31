import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'dart:ffi';

void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaDevices__get_user_media')(
      Pointer.fromFunction<Handle Function(Handle)>(getUserMedia));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaDevices__enumerate_devices')(
      Pointer.fromFunction<Handle Function()>(enumerateDevices));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_MediaDevices__enumerate_devices')(
      Pointer.fromFunction<Handle Function(Handle)>(getDisplayMedia));
}

Object gUM(Map<String, dynamic> mediaConstraints) async {
  var res = await navigator.mediaDevices.getUserMedia(mediaConstraints);
  // ignore: deprecated_member_use
  await res.getMediaTracks();
  return res.getTracks();
}

Object getUserMedia(MediaStreamConstraints constraints) {
  return () => gUM(
        {
          'audio': constraints.audio,
          'video': constraints.video,
        },
      );
}

Object enumerateDevices() {
  return () => navigator.mediaDevices.enumerateDevices();
}

Object getDisplayMedia(Map<String, dynamic> constraints) {
  return () => navigator.mediaDevices.getDisplayMedia(constraints);
}
