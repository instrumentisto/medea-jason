import 'dart:ffi';

import 'package:medea_jason/src/util/rust_handles_storage.dart';

import '../interface/audio_track_constraints.dart' as base_audio;
import '../interface/device_video_track_constraints.dart' as base_device_video;
import '../interface/display_video_track_constraints.dart'
    as base_display_video;
import '../interface/media_stream_settings.dart' as base;
import '../util/move_semantic.dart';
import 'ffi/nullable_pointer.dart';
import 'audio_track_constraints.dart';
import 'device_video_track_constraints.dart';
import 'display_video_track_constraints.dart';
import 'jason.dart';

typedef _new_C = Pointer Function();
typedef _new_Dart = Pointer Function();

typedef _audio_C = Void Function(Pointer, Pointer);
typedef _audio_Dart = void Function(Pointer, Pointer);

typedef _deviceVideo_C = Void Function(Pointer, Pointer);
typedef _deviceVideo_Dart = void Function(Pointer, Pointer);

typedef _displayVideo_C = Void Function(Pointer, Pointer);
typedef _displayVideo_Dart = void Function(Pointer, Pointer);

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

final _new = dl.lookupFunction<_new_C, _new_Dart>('MediaStreamSettings__new');

final _audio =
    dl.lookupFunction<_audio_C, _audio_Dart>('MediaStreamSettings__audio');

final _deviceVideo = dl.lookupFunction<_deviceVideo_C, _deviceVideo_Dart>(
    'MediaStreamSettings__device_video');

final _displayVideo = dl.lookupFunction<_displayVideo_C, _displayVideo_Dart>(
    'MediaStreamSettings__display_video');

final _free =
    dl.lookupFunction<_free_C, _free_Dart>('MediaStreamSettings__free');

class MediaStreamSettings extends base.MediaStreamSettings {
  /// [Pointer] to the Rust struct backing this object.
  final NullablePointer ptr = NullablePointer(_new());

  MediaStreamSettings() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void audio(@moveSemantics base_audio.AudioTrackConstraints constraints) {
    _audio(ptr.getInnerPtr(),
        (constraints as AudioTrackConstraints).ptr.getInnerPtr());
    constraints.ptr.free();
  }

  @override
  void deviceVideo(
      @moveSemantics
          base_device_video.DeviceVideoTrackConstraints constraints) {
    _deviceVideo(ptr.getInnerPtr(),
        (constraints as DeviceVideoTrackConstraints).ptr.getInnerPtr());
    constraints.ptr.free();
  }

  @override
  void displayVideo(
      @moveSemantics
          base_display_video.DisplayVideoTrackConstraints constraints) {
    _displayVideo(ptr.getInnerPtr(),
        (constraints as DisplayVideoTrackConstraints).ptr.getInnerPtr());
    constraints.ptr.free();
  }

  @moveSemantics
  @override
  void free() {
    _free(ptr.getInnerPtr());
    ptr.free();
  }
}
