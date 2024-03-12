import 'dart:ffi';
import 'dart:io';

import '../../medea_jason.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'local_media_track.dart';
import 'media_device_details.dart';
import 'media_display_details.dart';

class NativeMediaManagerHandle implements MediaManagerHandle {
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
    Pointer tracks;
    tracks = await (api.mediaManagerHandleInitLocalTracks(
        manager: opaque.innerOpaque,
        caps: (caps as MediaStreamSettings).setting) as Future) as Pointer;

    return api
        .vecLocalTracksFromPtr(ptr: tracks.address)
        .map((track) => NativeLocalMediaTrack(track))
        .toList();
  }

  @override
  Future<List<MediaDeviceDetails>> enumerateDevices() async {
    Pointer devices;
    devices = await (api.mediaManagerHandleEnumerateDevices(
        manager: opaque.innerOpaque) as Future);
    return api
        .vecMediaDeviceDetailsFromPtr(ptr: devices.address)
        .map((info) => NativeMediaDeviceDetails(info))
        .toList();
  }

  @override
  Future<List<MediaDisplayDetails>> enumerateDisplays() async {
    if (!(Platform.isLinux || Platform.isWindows || Platform.isMacOS)) {
      throw UnsupportedError(
          'enumerateDisplays() is not supported on ${Platform.operatingSystem}');
    }

    Pointer devices;
    devices = await (api.mediaManagerHandleEnumerateDisplays(
        manager: opaque.innerOpaque) as Future) as Pointer;

    return api
        .vecMediaDisplayDetailsFromPtr(ptr: devices.address)
        .map((info) => NativeMediaDisplayDetails(info))
        .toList();
  }

  @override
  Future<void> setOutputAudioId(String deviceId) async {
    await (api.mediaManagerHandleSetOutputAudioId(
        manager: opaque.innerOpaque, deviceId: deviceId) as Future);
  }

  @override
  Future<void> setMicrophoneVolume(int level) async {
    await (api.mediaManagerHandleSetMicrophoneVolume(
        manager: opaque.innerOpaque, level: level) as Future);
  }

  @override
  Future<int> microphoneVolume() async {
    return await (api.mediaManagerHandleMicrophoneVolume(
        manager: opaque.innerOpaque) as Future) as int;
  }

  @override
  Future<bool> microphoneVolumeIsAvailable() async {
    return await (api.mediaManagerHandleMicrophoneVolumeIsAvailable(
        manager: opaque.innerOpaque) as Future) as bool;
  }

  @override
  void onDeviceChange(void Function() cb) {
    api.mediaManagerHandleOnDeviceChange(manager: opaque.innerOpaque, cb: cb);
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
