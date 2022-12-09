import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import '../interface/media_device_info.dart';
import '../interface/media_display_info.dart';
import '../interface/media_manager.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as frb;
import 'jason.dart';
import 'local_media_track.dart';
import 'media_device_info.dart';
import 'media_display_info.dart';
import 'media_stream_settings.dart';

class NativeMediaManagerHandle extends MediaManagerHandle {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late frb.MediaManagerHandle opaque;

  /// Creates a new [MediaManagerHandle] backed by the Rust struct behind the
  /// provided [frb.MediaManagerHandle].
  NativeMediaManagerHandle(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<List<LocalMediaTrack>> initLocalTracks(
      base_settings.MediaStreamSettings caps) async {
    var tracks = await (api.mediaManagerHandleInitLocalTracks(
        manager: opaque,
        caps: (caps as MediaStreamSettings).opaque) as Future<Pointer>);
    var vec = api.vecLocalTracksFromPtr(ptr: tracks.address);

    var res = List<LocalMediaTrack>.empty(growable: true);

    var track = api.vecLocalTracksPop(vec: vec);
    while (track != null) {
      res.add(NativeLocalMediaTrack(track));
      track = api.vecLocalTracksPop(vec: vec);
    }
    vec.dispose();
    return res;
  }

  @override
  Future<List<MediaDeviceInfo>> enumerateDevices() async {
    var devices = await (api.mediaManagerHandleEnumerateDevices(manager: opaque)
        as Future<Pointer>);
    var vec = api.vecMediaDeviceInfoFromPtr(ptr: devices.address);

    var res = List<MediaDeviceInfo>.empty(growable: true);

    var device = api.vecMediaDeviceInfoPop(vec: vec);
    while (device != null) {
      res.add(NativeMediaDeviceInfo(device));
      device = api.vecMediaDeviceInfoPop(vec: vec);
    }
    vec.dispose();
    return res;
  }

  @override
  Future<List<MediaDisplayInfo>> enumerateDisplays() async {
    if (!(Platform.isLinux || Platform.isWindows || Platform.isMacOS)) {
      throw UnsupportedError(
          'enumerateDisplays() is not supported on ${Platform.operatingSystem}');
    }

    var devices = await (api.mediaManagerHandleEnumerateDisplays(
        manager: opaque) as Future<Pointer>);
    var vec = api.vecMediaDisplayInfoFromPtr(ptr: devices.address);

    var res = List<MediaDisplayInfo>.empty(growable: true);

    var device = api.vecMediaDisplayInfoPop(vec: vec);
    while (device != null) {
      res.add(NativeMediaDisplayInfo(device));
      device = api.vecMediaDisplayInfoPop(vec: vec);
    }
    vec.dispose();
    return res;
  }

  @override
  Future<void> setOutputAudioId(String deviceId) async {
    await (api.mediaManagerHandleSetOutputAudioId(
        manager: opaque, deviceId: deviceId) as Future<void>);
  }

  @override
  Future<void> setMicrophoneVolume(int level) async {
    await (api.mediaManagerHandleSetMicrophoneVolume(
        manager: opaque, level: level) as Future<void>);
  }

  @override
  Future<int> microphoneVolume() async {
    return await (api.mediaManagerHandleMicrophoneVolume(manager: opaque)
        as Future<int>);
  }

  @override
  Future<bool> microphoneVolumeIsAvailable() async {
    return await (api.mediaManagerHandleMicrophoneVolumeIsAvailable(
        manager: opaque) as Future<bool>);
  }

  @override
  void onDeviceChange(void Function() cb) {
    try {
      api.mediaManagerHandleOnDeviceChange(manager: opaque, cb: cb);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow.message);
    }
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
