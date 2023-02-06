import '../interface/audio_track_constraints.dart' as base_audio;
import '../interface/device_video_track_constraints.dart' as base_device_video;
import '../interface/media_stream_settings.dart' as base;
import '../util/move_semantic.dart';
import 'audio_track_constraints.dart';
import 'device_video_track_constraints.dart';
import 'display_video_track_constraints.dart';
import 'ffi/jason_api.g.dart' as frb;

import '../interface/display_video_track_constraints.dart'
    as base_display_video;

class MediaStreamSettings implements base.MediaStreamSettings {
  /// Rust `flutter_rust_bridge` api representation.
  final frb.ApiMediaStreamSettings setting = frb.ApiMediaStreamSettings(
      audio: frb.ApiAudioTrackConstrs(), deviceVideo: null, displayVideo: null);

  @override
  void audio(@moveSemantics base_audio.AudioTrackConstraints audio) {
    audio as AudioTrackConstraints;
    setting.audio = audio.constraints;
  }

  @override
  void deviceVideo(
      @moveSemantics
          base_device_video.DeviceVideoTrackConstraints deviceVideo) {
    deviceVideo as DeviceVideoTrackConstraints;
    setting.deviceVideo = deviceVideo.constraints;
  }

  @override
  void displayVideo(
      @moveSemantics
          base_display_video.DisplayVideoTrackConstraints displayVideo) {
    displayVideo as DisplayVideoTrackConstraints;
    setting.displayVideo = displayVideo.constraints;
  }

  @moveSemantics
  @override
  void free() {}
}
