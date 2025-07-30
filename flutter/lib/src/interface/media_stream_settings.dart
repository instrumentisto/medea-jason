import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'audio_track_constraints.dart';
import 'device_video_track_constraints.dart';
import 'display_video_track_constraints.dart';

/// Representation of [`MediaStreamConstraints`][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
abstract class MediaStreamSettings implements SyncPlatformHandle {
  /// Specifies a nature and settings of the device audio `LocalMediaTrack`.
  void deviceAudio(@moveSemantics AudioTrackConstraints constraints);

  /// Specifies a nature and settings of the display audio `LocalMediaTrack`.
  /// NOTE: Not implemented on native platforms.
  void displayAudio(@moveSemantics AudioTrackConstraints constraints);

  /// Sets constraints for obtaining a local video, sourced from a media device.
  void deviceVideo(@moveSemantics DeviceVideoTrackConstraints constraints);

  /// Set constraints for capturing a local video from user's display.
  void displayVideo(@moveSemantics DisplayVideoTrackConstraints constraints);
}
