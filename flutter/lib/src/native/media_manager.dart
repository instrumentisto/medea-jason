import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import '../interface/media_device_info.dart';
import '../interface/media_display_info.dart';
import '../interface/media_manager.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';
import 'local_media_track.dart';
import 'media_device_info.dart';
import 'media_display_info.dart';
import 'media_stream_settings.dart';

class NativeMediaManagerHandle extends MediaManagerHandle {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late RustOpaque<frb.MediaManagerHandle> opaque;

  /// Creates a new [MediaManagerHandle] backed by the Rust struct behind the
  /// provided [frb.MediaManagerHandle].
  NativeMediaManagerHandle(frb.MediaManagerHandle mediaManager)
      : opaque = RustOpaque(mediaManager) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<List<LocalMediaTrack>> initLocalTracks(
      base_settings.MediaStreamSettings caps) async {
    var tracks;
    try {
      tracks = await (api.mediaManagerHandleInitLocalTracks(
              manager: opaque.innerOpaque,
              caps: (caps as MediaStreamSettings).opaque.innerOpaque) as Future)
          as Pointer;
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }

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
    var devices;
    try {
      devices = await (api.mediaManagerHandleEnumerateDevices(
          manager: opaque.innerOpaque) as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }

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

    var devices;
    try {
      devices = await (api.mediaManagerHandleEnumerateDisplays(
          manager: opaque.innerOpaque) as Future) as Pointer;
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }

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
    try {
      await (api.mediaManagerHandleSetOutputAudioId(
          manager: opaque.innerOpaque, deviceId: deviceId) as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> setMicrophoneVolume(int level) async {
    try {
      await (api.mediaManagerHandleSetMicrophoneVolume(
          manager: opaque.innerOpaque, level: level) as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<int> microphoneVolume() async {
    try {
      return await (api.mediaManagerHandleMicrophoneVolume(
          manager: opaque.innerOpaque) as Future) as int;
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<bool> microphoneVolumeIsAvailable() async {
    try {
      return await (api.mediaManagerHandleMicrophoneVolumeIsAvailable(
              manager: opaque.innerOpaque) as Future) as int ==
          1;
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void onDeviceChange(void Function() cb) {
    try {
      api.mediaManagerHandleOnDeviceChange(manager: opaque.innerOpaque, cb: cb);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
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
