import '../interface/audio_track_constraints.dart';
import '../interface/device_video_track_constraints.dart';
import '../interface/display_video_track_constraints.dart';
import '../interface/media_stream_settings.dart';
import '../web/jason_wasm.dart' as wasm;
import '../util/move_semantic.dart';
import '../web/audio_track_constraints.dart';
import '../web/device_video_track_constraints.dart';
import 'display_video_track_constraints.dart';

class MediaStreamSettings extends IMediaStreamSettings {
  final wasm.MediaStreamSettings obj = wasm.MediaStreamSettings();

  /// Specifies a nature and settings of the audio `LocalMediaTrack`.
  @override
  void audio(@moveSemantics IAudioTrackConstraints constraints) {
    obj.audio((constraints as AudioTrackConstraints).obj);
  }

  /// Sets constraints for obtaining a local video, sourced from a media device.
  @override
  void deviceVideo(@moveSemantics IDeviceVideoTrackConstraints constraints) {
    obj.device_video((constraints as DeviceVideoTrackConstraints).obj);
  }

  /// Set constraints for capturing a local video from user's display.
  @override
  void displayVideo(@moveSemantics IDisplayVideoTrackConstraints constraints) {
    obj.display_video((constraints as DisplayVideoTrackConstraints).obj);
  }

  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
