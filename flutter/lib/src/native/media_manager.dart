import 'dart:ffi';
import 'dart:io';

import '../interface/media_device_info.dart';
import '../interface/media_display_info.dart';
import '../interface/media_manager.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as api;
import 'jason.dart';
import 'local_media_track.dart';
import 'media_device_info.dart';
import 'media_display_info.dart';
import 'media_stream_settings.dart';

class NativeMediaManagerHandle extends MediaManagerHandle {
  /// [Pointer] to the Rust struct backing this object.
  late api.MediaManagerHandle opaque;

  /// Creates a new [MediaManagerHandle] backed by the Rust struct behind the
  /// provided [Pointer].

  NativeMediaManagerHandle.opaque(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<List<LocalMediaTrack>> initLocalTracks(
      base_settings.MediaStreamSettings caps) async {
    Pointer tracks = await rust2dart(impl_api.mediaManagerHandleInitLocalTracks(
        manager: opaque, caps: (caps as MediaStreamSettings).opaque));
    var vec = impl_api.vecLocalTracksFromPtr(ptr: tracks.address);

    var res = List<LocalMediaTrack>.empty(growable: true);

    var track = impl_api.vecLocalTracksPop(vec: vec);
    while (track != null) {
      res.add(NativeLocalMediaTrack.opaque(track));
      track = impl_api.vecLocalTracksPop(vec: vec);
    }
    vec.dispose();
    return res;
  }

  @override
  Future<List<MediaDeviceInfo>> enumerateDevices() async {
    Pointer devices = await rust2dart(
        impl_api.mediaManagerHandleEnumerateDevices(manager: opaque));
    var vec = impl_api.vecMediaDeviceInfoFromPtr(ptr: devices.address);

    var res = List<MediaDeviceInfo>.empty(growable: true);

    var device = impl_api.vecMediaDeviceInfoPop(vec: vec);
    while (device != null) {
      res.add(NativeMediaDeviceInfo.opaque(device));
      device = impl_api.vecMediaDeviceInfoPop(vec: vec);
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

    Pointer devices = await rust2dart(
        impl_api.mediaManagerHandleEnumerateDisplays(manager: opaque));
    var vec = impl_api.vecMediaDisplayInfoFromPtr(ptr: devices.address);

    var res = List<MediaDisplayInfo>.empty(growable: true);

    var device = impl_api.vecMediaDisplayInfoPop(vec: vec);
    while (device != null) {
      res.add(NativeMediaDisplayInfo.opaque(device));
      device = impl_api.vecMediaDisplayInfoPop(vec: vec);
    }
    vec.dispose();
    return res;
  }

  @override
  Future<void> setOutputAudioId(String deviceId) async {
    await rust2dart(impl_api.mediaManagerHandleSetOutputAudioId(
        manager: opaque, deviceId: deviceId));
  }

  @override
  Future<void> setMicrophoneVolume(int level) async {
    await rust2dart(impl_api.mediaManagerHandleSetMicrophoneVolume(
        manager: opaque, level: level));
  }

  @override
  Future<int> microphoneVolume() async {
    return await rust2dart(
        impl_api.mediaManagerHandleMicrophoneVolume(manager: opaque));
  }

  @override
  Future<bool> microphoneVolumeIsAvailable() async {
    return await rust2dart(impl_api
        .mediaManagerHandleMicrophoneVolumeIsAvailable(manager: opaque));
  }

  @override
  void onDeviceChange(void Function() cb) {
    impl_api.mediaManagerHandleOnDeviceChange(
        manager: opaque, cb: handle2rust(cb));
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
