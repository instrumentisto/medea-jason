import '../util/move_semantic.dart';
import 'audio_track_constraints.dart';
import 'display_video_track_constraints.dart';
import 'device_video_track_constraints.dart';

/// Representation of [`MediaStreamConstraints`][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
abstract class IMediaStreamSettings {
  /// Specifies a nature and settings of the audio `LocalMediaTrack`.
  void audio(@moveSemantics IAudioTrackConstraints constraints) {
    throw UnimplementedError();
  }

  /// Sets constraints for obtaining a local video, sourced from a media device.
  void deviceVideo(@moveSemantics IDeviceVideoTrackConstraints constraints) {
    throw UnimplementedError();
  }

  /// Set constraints for capturing a local video from user's display.
  void displayVideo(@moveSemantics IDisplayVideoTrackConstraints constraints) {
    throw UnimplementedError();
  }

  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  void free() {
    throw UnimplementedError();
  }
}
