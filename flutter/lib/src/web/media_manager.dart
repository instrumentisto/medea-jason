// ignore_for_file: implementation_imports

import 'package:medea_flutter_webrtc/src/platform/web/audio_renderer.dart'
    as audio_renderer;
import 'package:medea_flutter_webrtc/src/platform/web/video_renderer.dart'
    as video_renderer;

import 'dart:js_interop';

import '../interface/media_device_details.dart';
import '../interface/media_display_details.dart';
import '../interface/media_manager.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;
import 'local_media_track.dart';
import 'media_device_details.dart';
import 'media_stream_settings.dart';

class WebMediaManagerHandle implements MediaManagerHandle {
  late wasm.MediaManagerHandle obj;

  WebMediaManagerHandle(this.obj);

  @override
  Future<List<LocalMediaTrack>> initLocalTracks(
    base_settings.MediaStreamSettings caps,
  ) async {
    final tracks = await fallibleFuture(
      obj.init_local_tracks((caps as MediaStreamSettings).obj).toDart,
    );

    return tracks.toDart
        .map((t) => WebLocalMediaTrack(t as wasm.LocalMediaTrack))
        .toList();
  }

  @override
  Future<List<MediaDeviceDetails>> enumerateDevices() async {
    final tracks = await fallibleFuture(obj.enumerate_devices().toDart);
    return tracks.toDart
        .map((t) => WebMediaDeviceDetails(t as wasm.MediaDeviceDetails))
        .toList();
  }

  @override
  Future<List<MediaDisplayDetails>> enumerateDisplays() async {
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
    audio_renderer.setOutputAudioSinkId(deviceId);
  }

  @override
  void onDeviceChange(void Function() cb) {
    obj.on_device_change(cb.toJS);
  }

  @override
  Future<bool> microphoneVolumeIsAvailable() async {
    return false;
  }

  @override
  Future<int> microphoneVolume() {
    // TODO: implement microphoneVolume
    throw UnimplementedError();
  }

  @override
  Future<void> setMicrophoneVolume(int level) {
    // TODO: implement setMicrophoneVolume
    throw UnimplementedError();
  }
}
