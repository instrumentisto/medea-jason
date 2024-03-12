import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import 'package:medea_jason/src/native/ffi/native_string.dart';
import 'media_devices.g.dart' as bridge;

/// Option to mock `getUserMedia()` request.
const bool mockable = bool.fromEnvironment('MOCKABLE', defaultValue: false);

/// Registers functions allowing Rust to operate Dart media devices.
void registerFunctions(DynamicLibrary dl) {
  if (mockable) {
    bridge.registerFunction(
      dl,
      enumerateDevices: Pointer.fromFunction(_enumerateDevices),
      enumerateDisplays: Pointer.fromFunction(_enumerateDisplays),
      getUserMedia: Pointer.fromFunction(MockMediaDevices.getUserMedia),
      getDisplayMedia: Pointer.fromFunction(_getDisplayMedia),
      setOutputAudioId: Pointer.fromFunction(_setOutputAudioId),
      setMicrophoneVolume: Pointer.fromFunction(_setMicrophoneVolume),
      microphoneVolumeIsAvailable:
          Pointer.fromFunction(_microphoneVolumeIsAvailable),
      microphoneVolume: Pointer.fromFunction(_microphoneVolume),
      onDeviceChange: Pointer.fromFunction(_onDeviceChange),
      getMediaExceptionKind: Pointer.fromFunction(_getMediaExceptionKind, 0),
    );
  } else {
    bridge.registerFunction(
      dl,
      enumerateDevices: Pointer.fromFunction(_enumerateDevices),
      enumerateDisplays: Pointer.fromFunction(_enumerateDisplays),
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
}

/// Provider to mock `getUserMedia()` request.
///
/// [mockable] must be `true`.
class MockMediaDevices {
  /// Default `getUserMedia()` request.
  static const _defaultGUM = webrtc.getUserMedia;

  /// Current `getUserMedia()` request.
  static Function _getUserMedia = _defaultGUM;

  /// Sets `getUserMedia()` request to the provided function.
  static set gum(Function(webrtc.DeviceConstraints) f) {
    _getUserMedia = f;
  }

  /// Requests media input access and returns the created
  /// [webrtc.MediaStreamTrack]s.
  static Object getUserMedia(webrtc.DeviceConstraints constraints) {
    return () => _getUserMedia(constraints);
  }

  /// Sets the current `getUserMedia()` request to default one.
  static void resetGUM() {
    _getUserMedia = _defaultGUM;
  }
}

/// Requests media input access and returns the created [webrtc.MediaStreamTrack]s.
Object _getUserMedia(webrtc.DeviceConstraints constraints) {
  return () => webrtc.getUserMedia(constraints);
}

/// Returns all the available media devices.
Object _enumerateDevices() {
  return () => webrtc.enumerateDevices();
}

/// Returns all the available media displays.
Object _enumerateDisplays() {
  return () => webrtc.enumerateDisplays();
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
