import '../util/move_semantic.dart';

/// The kind of the represented media device.
enum MediaDeviceKind {
  /// Represents an audio input device; for example a microphone.
  audioinput,
  /// Represents a video input device; for example a webcam.
  videoinput,
  /// Represents an audio output device; for example a pair of headphones.
  audiooutput,
}

/// [`MediaDeviceInfo`][1] interface.
///
/// [1]: https://w3.org/TR/mediacapture-streams/#device-info
abstract class MediaDeviceInfo {
  /// Returns an unique identifier of the represented device.
  String deviceId();

  /// Returns label describing the represented device (for example "External USB
  /// Webcam").
  ///
  /// If the device has no associated label, then returns an empty string.
  String label();

  /// Returns kind of the represented device.
  ///
  /// This representation of a [`MediaDeviceInfo`][1] is ONLY for input devices.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams/#device-info
  MediaDeviceKind kind();

  /// Returns a group identifier of the represented device.
  ///
  /// Two devices have the same group identifier if they belong to the same
  /// physical device. For example, the audio input and output devices
  /// representing the speaker and microphone of the same headset have the
  /// same [`groupId`][1].
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediadeviceinfo-groupid
  String? groupId();

  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free();
}
