// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.6.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import '../../../frb_generated.dart';
import '../../../media/constraints.dart';
import '../api.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `from`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner< MediaManagerHandle>>
abstract class MediaManagerHandle implements RustOpaqueInterface {
  /// Returns a list of [`ApiMediaDeviceDetails`] objects representing
  /// available media input and devices, such as microphones, cameras, and
  /// so forth.
  Object enumerateDevices();

  /// Returns a list of [`ApiMediaDisplayDetails`] objects representing
  /// available sources that can be used for screen capturing.
  Object enumerateDisplays();

  /// Returns [`LocalMediaTrack`]s objects, built from the provided
  /// [`ApiMediaStreamSettings`].
  Object initLocalTracks({required ApiMediaStreamSettings caps});

  /// Returns the current microphone volume level in percents.
  Object microphoneVolume();

  /// Indicates whether it's possible to access microphone volume settings.
  Object microphoneVolumeIsAvailable();

  /// Subscribes onto the [`MediaManagerHandle`]'s `devicechange` event.
  ///
  /// Sets an ideal [frameRate][1] constraint.
  ///
  /// # Errors
  ///
  /// If [`MediaManagerHandle::on_device_change()`] errors.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
  void onDeviceChange({required Object cb});

  /// Sets the microphone volume level in percents.
  Object setMicrophoneVolume({required PlatformInt64 level});

  /// Switches the current output audio device to the device with the provided
  /// `device_id`.
  Object setOutputAudioId({required String deviceId});
}
