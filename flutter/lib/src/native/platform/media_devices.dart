import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart' as webrtc;

import 'package:medea_jason/src/native/ffi/native_string.dart';
import 'media_devices.g.dart' as bridge;

/// Registers functions allowing Rust to operate Dart media devices.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    enumerateDevices: Pointer.fromFunction(_enumerateDevices),
    getUserMedia: Pointer.fromFunction(_getUserMedia),
    getDisplayMedia: Pointer.fromFunction(_getDisplayMedia),
    setOutputAudioId: Pointer.fromFunction(_setOutputAudioId),
    setMicrophoneVolume: Pointer.fromFunction(_setMicrophoneVolume),
    microphoneVolumeIsAvailable:
        Pointer.fromFunction(_microphoneVolumeIsAvailable),
    microphoneVolume: Pointer.fromFunction(_microphoneVolume),
    onDeviceChange: Pointer.fromFunction(_onDeviceChange),
    getMediaExceptionKind: Pointer.fromFunction(_getMediaExceptionKind, 0),
  );
}

// todo
var MOCK_GUM = (webrtc.DeviceConstraints constraints) => webrtc.getUserMedia(constraints);

/// Requests media input access and returns the created [webrtc.MediaStreamTrack]s.
Object _getUserMedia(webrtc.DeviceConstraints constraints) {
  return () async {
    return MOCK_GUM(constraints);
  };
}

/// Returns all the available media devices.
Object _enumerateDevices() {
  return () => webrtc.enumerateDevices();
}

/// Starts capturing the contents of a display and returns the created
/// [webrtc.MediaStreamTrack]s.
Object _getDisplayMedia(webrtc.DisplayConstraints constraints) {
  return () => webrtc.getDisplayMedia(constraints);
}

/// Switches output audio device to the device with the provided [deviceId].
Object _setOutputAudioId(Pointer<Utf8> deviceId) {
  return () => webrtc.setOutputAudioId(deviceId.nativeStringToDartString());
}

/// Sets the microphone volume level in percents.
Object _setMicrophoneVolume(int level) {
  return () => webrtc.setMicrophoneVolume(level);
}

/// Indicates whether it's possible to access microphone volume settings.
Object _microphoneVolumeIsAvailable() {
  return () async {
    return await webrtc.microphoneVolumeIsAvailable() ? 1 : 0;
  };
}

/// Gets the current microphone volume level in percents.
Object _microphoneVolume() {
  return () => webrtc.microphoneVolume();
}

/// Subscribes onto the `MediaDevices`'s `devicechange` event.
void _onDeviceChange(Function cb) {
  webrtc.onDeviceChange(() => cb(null));
}

/// Returns the kind of the `GetMediaException`.
int _getMediaExceptionKind(webrtc.GetMediaException exception) {
  return exception.kind().index;
}
