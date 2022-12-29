import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import '../../medea_jason.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'local_media_track.dart';
import 'media_device_info.dart';
import 'media_display_info.dart';

class NativeMediaManagerHandle extends MediaManagerHandle {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final RustOpaque<frb.MediaManagerHandle> opaque;

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
          caps: (caps as MediaStreamSettings).setting) as Future) as Pointer;
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
    }

    return api
        .vecLocalTracksFromPtr(ptr: tracks.address)
        .map((track) => NativeLocalMediaTrack(track))
        .toList();
  }

  @override
  Future<List<MediaDeviceInfo>> enumerateDevices() async {
    var devices;
    try {
      devices = await (api.mediaManagerHandleEnumerateDevices(
          manager: opaque.innerOpaque) as Future);
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
    }

    return api
        .vecMediaDeviceInfoFromPtr(ptr: devices.address)
        .map((info) => NativeMediaDeviceInfo(info))
        .toList();
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
      throw anyhow.parse();
    }

    return api
        .vecMediaDisplayInfoFromPtr(ptr: devices.address)
        .map((info) => NativeMediaDisplayInfo(info))
        .toList();
  }

  @override
  Future<void> setOutputAudioId(String deviceId) async {
    try {
      await (api.mediaManagerHandleSetOutputAudioId(
          manager: opaque.innerOpaque, deviceId: deviceId) as Future);
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
    }
  }

  @override
  Future<void> setMicrophoneVolume(int level) async {
    try {
      await (api.mediaManagerHandleSetMicrophoneVolume(
          manager: opaque.innerOpaque, level: level) as Future);
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
    }
  }

  @override
  Future<int> microphoneVolume() async {
    try {
      return await (api.mediaManagerHandleMicrophoneVolume(
          manager: opaque.innerOpaque) as Future) as int;
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
    }
  }

  @override
  Future<bool> microphoneVolumeIsAvailable() async {
    try {
      return await (api.mediaManagerHandleMicrophoneVolumeIsAvailable(
          manager: opaque.innerOpaque) as Future) as bool;
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
    }
  }

  @override
  void onDeviceChange(void Function() cb) {
    try {
      api.mediaManagerHandleOnDeviceChange(manager: opaque.innerOpaque, cb: cb);
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
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
