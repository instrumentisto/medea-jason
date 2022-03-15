import 'package:medea_jason/src/util/rust_handles_storage.dart';

import '../util/move_semantic.dart';
import 'track_kinds.dart';

/// [`MediaDeviceInfo`][1] interface.
///
/// [1]: https://w3.org/TR/mediacapture-streams/#device-info
abstract class InputDeviceInfo implements FreeableHandle {
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
  MediaKind kind();

  /// Returns a group identifier of the represented device.
  ///
  /// Two devices have the same group identifier if they belong to the same
  /// physical device. For example, the audio input and output devices
  /// representing the speaker and microphone of the same headset have the
  /// same [`groupId`][1].
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediadeviceinfo-groupid
  String? groupId();
}
