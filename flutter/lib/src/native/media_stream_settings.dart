import '../interface/audio_track_constraints.dart' as base_audio;
import '../interface/device_video_track_constraints.dart' as base_device_video;
import '../interface/media_stream_settings.dart' as base;
import 'audio_track_constraints.dart';
import 'device_video_track_constraints.dart';
import 'display_video_track_constraints.dart';
import 'ffi/jason_api.g.dart' as frb;

import '../interface/display_video_track_constraints.dart'
    as base_display_video;

class MediaStreamSettings implements base.MediaStreamSettings {
  /// Rust `flutter_rust_bridge` API representation.
  final frb.ApiMediaStreamSettings setting = frb.ApiMediaStreamSettings(
      audio: frb.ApiAudioConstraints(), deviceVideo: null, displayVideo: null);

  @override
  void audio(base_audio.AudioTrackConstraints audio) {
    audio as AudioTrackConstraints;
    setting.audio = audio.constraints;
  }

  @override
  void deviceVideo(base_device_video.DeviceVideoTrackConstraints deviceVideo) {
    deviceVideo as DeviceVideoTrackConstraints;
    setting.deviceVideo = deviceVideo.constraints;
  }

  @override
  void displayVideo(
      base_display_video.DisplayVideoTrackConstraints displayVideo) {
    displayVideo as DisplayVideoTrackConstraints;
    setting.displayVideo = displayVideo.constraints;
  }

  @override
  void free() {}
}
