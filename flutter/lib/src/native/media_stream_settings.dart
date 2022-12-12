import '../interface/audio_track_constraints.dart' as base_audio;
import '../interface/device_video_track_constraints.dart' as base_device_video;
import '../interface/media_stream_settings.dart' as base;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'audio_track_constraints.dart';
import 'device_video_track_constraints.dart';
import 'display_video_track_constraints.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';

import '../interface/display_video_track_constraints.dart'
    as base_display_video;

class MediaStreamSettings extends base.MediaStreamSettings {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  frb.MediaStreamSettings opaque = api.mediaStreamSettingsNew();

  /// Constructs a new [MediaStreamSettings] backed by the Rust struct behind the
  /// provided [frb.RefCellMediaStreamSettings].
  MediaStreamSettings() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void audio(@moveSemantics base_audio.AudioTrackConstraints constraints) {
    opaque.move = true;
    opaque = api.mediaStreamSettingsAudio(
        mediaStreamSettings: opaque,
        constr: (constraints as AudioTrackConstraints).opaque);
    constraints.opaque.dispose();
  }

  @override
  void deviceVideo(
      @moveSemantics
          base_device_video.DeviceVideoTrackConstraints constraints) {
    constraints as DeviceVideoTrackConstraints;
    constraints.opaque.move = true;
    opaque.move = true;
    opaque = api.mediaStreamSettingsDeviceVideo(
        mediaStreamSettings: opaque, constr: constraints.opaque);
  }

  @override
  void displayVideo(
      @moveSemantics
          base_display_video.DisplayVideoTrackConstraints constraints) {
    constraints as DisplayVideoTrackConstraints;
    constraints.opaque.move = true;
    opaque.move = true;
    opaque = api.mediaStreamSettingsDisplayVideo(
        mediaStreamSettings: opaque, constr: constraints.opaque);
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
