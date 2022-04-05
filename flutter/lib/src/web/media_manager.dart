import 'package:flutter_webrtc/src/platform/web/video_renderer.dart'
    as video_renderer;
import 'package:js/js.dart';

import '../interface/media_device_info.dart';
import '../interface/local_media_track.dart';
import '../interface/media_manager.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'media_device_info.dart';
import 'jason_wasm.dart' as wasm;
import 'local_media_track.dart';
import 'media_stream_settings.dart';

class WebMediaManagerHandle extends MediaManagerHandle {
  late wasm.MediaManagerHandle obj;

  WebMediaManagerHandle(this.obj);

  @override
  Future<List<LocalMediaTrack>> initLocalTracks(
      base_settings.MediaStreamSettings caps) async {
    var tracks = await fallibleFuture(
        obj.init_local_tracks((caps as MediaStreamSettings).obj));
    return tracks.map((t) => WebLocalMediaTrack(t)).toList();
  }

  @override
  Future<List<MediaDeviceInfo>> enumerateDevices() async {
    var tracks = await fallibleFuture(obj.enumerate_devices());
    return tracks.map((t) => WebMediaDeviceInfo(t)).toList();
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }

  @override
  Future<void> setOutputAudioId(String deviceId) async {
    video_renderer.setOutputAudioSinkId(deviceId);
  }

  @override
  void onDeviceChange(Function cb) {
    obj.on_device_change(allowInterop(cb));
  }
}
