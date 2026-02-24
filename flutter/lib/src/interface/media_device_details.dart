import '/src/util/rust_handles_storage.dart';
import 'enums.dart' show MediaDeviceKind, AudioDeviceKind;

export 'enums.dart' show MediaDeviceKind, AudioDeviceKind;

/// [`MediaDeviceInfo`][1] interface.
///
/// [1]: https://w3.org/TR/mediacapture-streams#device-info
abstract class MediaDeviceDetails implements SyncPlatformHandle {
  /// Returns an unique identifier of the represented device.
  String deviceId();

  /// Returns label describing the represented device (for example "External USB
  /// Webcam").
  ///
  /// If the device has no associated label, then returns an empty string.
  String label();

  /// Returns kind of the represented device.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#device-info
  MediaDeviceKind kind();

  /// Returns an [AudioDeviceKind] of these [MediaDeviceDetails], if applicable.
  ///
  /// Only implemented on mobile platforms at the moment.
  AudioDeviceKind? audioDeviceKind();

  /// Returns a group identifier of the represented device.
  ///
  /// Two devices have the same group identifier if they belong to the same
  /// physical device. For example, the audio input and output devices
  /// representing the speaker and microphone of the same headset have the
  /// same [`groupId`][1].
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-groupid
  String? groupId();

  /// Audio device sample rate in `Hz`.
  ///
  /// For audio devices only. `null` for video or if is unavailable.
  int? sampleRate();

  /// Audio device number of channels.
  ///
  /// For audio devices only. `null` for video or if is unavailable.
  int? numChannels();

  /// Indicates whether the last attempt to use this device failed.
  bool isFailed();
}
