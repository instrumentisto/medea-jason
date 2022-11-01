import 'dart:ffi';

import '../interface/audio_track_constraints.dart' as base_audio;
import '../interface/device_video_track_constraints.dart' as base_device_video;
import '../interface/media_stream_settings.dart' as base;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'audio_track_constraints.dart';
import 'device_video_track_constraints.dart';
import 'display_video_track_constraints.dart';
import 'ffi/api_api.g.dart' as api;
import 'jason.dart';

import '../interface/display_video_track_constraints.dart'
    as base_display_video;

class MediaStreamSettings extends base.MediaStreamSettings {
  /// [Pointer] to the Rust struct backing this object.
  final api.RefCellMediaStreamSettings opaque =
      impl_api.mediaStreamSettingsNew();

  MediaStreamSettings() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void audio(@moveSemantics base_audio.AudioTrackConstraints constraints) {
    impl_api.mediaStreamSettingsAudio(
        mediaStreamSettings: opaque,
        constraints: (constraints as AudioTrackConstraints).opaque);
    constraints.opaque.dispose();
  }

  @override
  void deviceVideo(
      @moveSemantics
          base_device_video.DeviceVideoTrackConstraints constraints) {
    impl_api.mediaStreamSettingsDeviceVideo(
        mediaStreamSettings: opaque,
        constraints: (constraints as DeviceVideoTrackConstraints).opaque);
    constraints.opaque.dispose();
  }

  @override
  void displayVideo(
      @moveSemantics
          base_display_video.DisplayVideoTrackConstraints constraints) {
    impl_api.mediaStreamSettingsDisplayVideo(
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
