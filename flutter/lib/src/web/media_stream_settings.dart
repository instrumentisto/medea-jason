import '../interface/audio_track_constraints.dart' as base_audio;
import '../interface/device_video_track_constraints.dart' as base_device_video;
import '../interface/media_stream_settings.dart' as base;
import '../util/move_semantic.dart';
import 'audio_track_constraints.dart';
import 'device_video_track_constraints.dart';
import 'display_video_track_constraints.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

import '../interface/display_video_track_constraints.dart'
    as base_display_video;

class MediaStreamSettings implements base.MediaStreamSettings {
  final wasm.MediaStreamSettings obj = wasm.MediaStreamSettings();

  @override
  void deviceAudio(
    @moveSemantics base_audio.DeviceAudioTrackConstraints constraints,
  ) {
    fallibleFunction(
      () => obj.device_audio((constraints as DeviceAudioTrackConstraints).obj),
    );
  }

  @override
  void displayAudio(
    @moveSemantics base_audio.DisplayAudioTrackConstraints constraints,
  ) {
    fallibleFunction(
      () =>
          obj.display_audio((constraints as DisplayAudioTrackConstraints).obj),
    );
  }

  @override
  void deviceVideo(
    @moveSemantics base_device_video.DeviceVideoTrackConstraints constraints,
  ) {
    fallibleFunction(
      () => obj.device_video((constraints as DeviceVideoTrackConstraints).obj),
    );
  }

  @override
  void displayVideo(
    @moveSemantics base_display_video.DisplayVideoTrackConstraints constraints,
  ) {
    fallibleFunction(
      () =>
          obj.display_video((constraints as DisplayVideoTrackConstraints).obj),
    );
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
