import '../interface/audio_track_constraints.dart' as base_audio;
import '../interface/device_video_track_constraints.dart' as base_device_video;
import '../interface/media_stream_settings.dart' as base;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'audio_track_constraints.dart';
import 'device_video_track_constraints.dart';
import 'display_video_track_constraints.dart';
import 'ffi/api_api.g.dart' as frb;
import 'jason.dart';

import '../interface/display_video_track_constraints.dart'
    as base_display_video;

class MediaStreamSettings extends base.MediaStreamSettings {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final frb.RefCellMediaStreamSettings opaque = api.mediaStreamSettingsNew();

  /// Constructs a new [MediaStreamSettings] backed by the Rust struct behind the
  /// provided [frb.RefCellMediaStreamSettings].
  MediaStreamSettings() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void audio(@moveSemantics base_audio.AudioTrackConstraints constraints) {
    api.mediaStreamSettingsAudio(
        mediaStreamSettings: opaque,
        constraints: (constraints as AudioTrackConstraints).opaque);
    constraints.opaque.dispose();
  }

  @override
  void deviceVideo(
      @moveSemantics
          base_device_video.DeviceVideoTrackConstraints constraints) {
    api.mediaStreamSettingsDeviceVideo(
        mediaStreamSettings: opaque,
        constraints: (constraints as DeviceVideoTrackConstraints).opaque);
    constraints.opaque.dispose();
  }

  @override
  void displayVideo(
      @moveSemantics
          base_display_video.DisplayVideoTrackConstraints constraints) {
    api.mediaStreamSettingsDisplayVideo(
        mediaStreamSettings: opaque,
        constraints: (constraints as DisplayVideoTrackConstraints).opaque);
    constraints.opaque.dispose();
  }

  @moveSemantics
  @override
  void free() {
    if (!opaque.isStale()) {
      RustHandlesStorage().removeHandle(this);

      opaque.dispose();
    }
  }
}
