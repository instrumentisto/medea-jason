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
      enumerateDevices: _enumerateDevices,
      enumerateDisplays: _enumerateDisplays,
      getUserMedia: MockMediaDevices.getUserMedia,
      getDisplayMedia: _getDisplayMedia,
      setOutputAudioId: _setOutputAudioId,
      setMicrophoneVolume: _setMicrophoneVolume,
      microphoneVolumeIsAvailable: _microphoneVolumeIsAvailable,
      microphoneVolume: _microphoneVolume,
      onDeviceChange: _onDeviceChange,
      getMediaExceptionKind: _getMediaExceptionKind,
    );
  } else {
    bridge.registerFunction(
      dl,
      enumerateDevices: _enumerateDevices,
      enumerateDisplays: _enumerateDisplays,
      getUserMedia: _getUserMedia,
      getDisplayMedia: _getDisplayMedia,
      setOutputAudioId: _setOutputAudioId,
      setMicrophoneVolume: _setMicrophoneVolume,
      microphoneVolumeIsAvailable: _microphoneVolumeIsAvailable,
      microphoneVolume: _microphoneVolume,
      onDeviceChange: _onDeviceChange,
      getMediaExceptionKind: _getMediaExceptionKind,
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
  static Object getUserMedia(Object constraints) {
    constraints as webrtc.DeviceConstraints;
    return () => _getUserMedia(constraints);
  }

  /// Sets the current `getUserMedia()` request to default one.
  static void resetGUM() {
    _getUserMedia = _defaultGUM;
  }
}

/// Requests media input access and returns the created [webrtc.MediaStreamTrack]s.
Future<List<webrtc.MediaStreamTrack>> Function() _getUserMedia(
  Object constraints,
) {
  print('_getUserMedia 000');
  constraints as webrtc.DeviceConstraints;
  return () async {
    print('_getUserMedia 111');
    var s = await webrtc.getUserMedia(constraints);
    print('_getUserMedia 222');

    return s;
  };
}

/// Returns all the available media devices.
Future<List<webrtc.MediaDeviceInfo>> Function() _enumerateDevices() {
  print('_enumerateDevices 000');
  return () async {
    print('_enumerateDevices 111');
    var s = await webrtc.enumerateDevices();
    print('_enumerateDevices 222');
    return s;
  };
}

/// Returns all the available media displays.
Future<List<webrtc.MediaDisplayInfo>> Function() _enumerateDisplays() {
  return () => webrtc.enumerateDisplays();
}

/// Starts capturing the contents of a display and returns the created
/// [webrtc.MediaStreamTrack]s.
Future<List<webrtc.MediaStreamTrack>> Function() _getDisplayMedia(
  Object constraints,
) {
  constraints as webrtc.DisplayConstraints;
  return () => webrtc.getDisplayMedia(constraints);
}

/// Switches output audio device to the device with the provided [deviceId].
Future<void> Function() _setOutputAudioId(Pointer<Utf8> deviceId) {
  return () => webrtc.setOutputAudioId(deviceId.nativeStringToDartString());
}

/// Sets the microphone volume level in percents.
Future<void> Function() _setMicrophoneVolume(int level) {
  return () => webrtc.setMicrophoneVolume(level);
}

/// Indicates whether it's possible to access microphone volume settings.
Future<bool> Function() _microphoneVolumeIsAvailable() {
  return () async {
    return await webrtc.microphoneVolumeIsAvailable();
  };
}

/// Gets the current microphone volume level in percents.
Future<int> Function() _microphoneVolume() {
  return () => webrtc.microphoneVolume();
}

/// Subscribes onto the `MediaDevices`'s `devicechange` event.
void _onDeviceChange(Object cb) {
  cb as Function;
  webrtc.onDeviceChange(() => cb(null));
}

/// Returns the kind of the `GetMediaException`.
int _getMediaExceptionKind(Object exception) {
  exception as webrtc.GetMediaException;
  return exception.kind().index;
}
