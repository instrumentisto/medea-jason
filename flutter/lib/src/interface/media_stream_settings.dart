import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'audio_track_constraints.dart';
import 'device_video_track_constraints.dart';
import 'display_video_track_constraints.dart';

/// Representation of [`MediaStreamConstraints`][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
abstract class MediaStreamSettings implements SyncPlatformHandle {
  /// Sets constraints for obtaining a local audio from a system recording
  /// device.
  void deviceAudio(@moveSemantics DeviceAudioTrackConstraints constraints);

  /// Sets constraints for obtaining a system audio capture.
  ///
  /// __NOTE__: Behaviour is platform dependent and there is no propper feature
  /// check. It is known to only work in Chrome and Chrome-based browsers. It
  /// must always be coupled with a `DisplayVideoTrackConstraints`, meaning
  /// that system audio capture prompt is a part of the screen-sharing prompt,
  /// so if you try to request `displayAudio` without `deviceVideo` the UA will
  /// ask user for screen capture track anyway.
  ///
  /// It is also OS dependent:
  /// 1. Only `Chrome-tab` audio can be captured on macOS and Linux.
  /// 2. Both `Chrome-tab` and `Entire screen` audio can be captured on Windows.
  ///
  /// As of desktop platforms only full system audio capture is supported
  /// and it does not depend on screen-sharing.
  void displayAudio(@moveSemantics DisplayAudioTrackConstraints constraints);

  /// Sets constraints for obtaining a local video, sourced from a media device.
  void deviceVideo(@moveSemantics DeviceVideoTrackConstraints constraints);

  /// Set constraints for capturing a local video from user's display.
  void displayVideo(@moveSemantics DisplayVideoTrackConstraints constraints);
}
