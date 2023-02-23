// ignore_for_file: implementation_imports

import 'package:medea_flutter_webrtc/src/platform/web/video_renderer.dart'
    as video_renderer;

import 'package:js/js.dart';

import '../interface/media_device_info.dart';
import '../interface/media_display_info.dart';
import '../interface/media_manager.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;
import 'local_media_track.dart';
import 'media_device_info.dart';
import 'media_stream_settings.dart';

class WebMediaManagerHandle implements MediaManagerHandle {
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

  @override
  Future<List<MediaDisplayInfo>> enumerateDisplays() async {
    throw UnsupportedError('enumerateDisplays() is not implemented for Web');
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

  @override
  Future<bool> microphoneVolumeIsAvailable() async {
    return false;
  }

  @override
  Future<int> microphoneVolume() {
    // TODO(logist322): implement microphoneVolume
    throw UnimplementedError();
  }

  @override
  Future<void> setMicrophoneVolume(int level) {
    // TODO(logist322): implement setMicrophoneVolume
    throw UnimplementedError();
  }
}
