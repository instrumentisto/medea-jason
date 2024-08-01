import 'dart:ffi';
import 'dart:io';

import '../../medea_jason.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/frb//api/dart/api.dart' as frb;
import 'local_media_track.dart';
import 'media_device_details.dart';
import 'media_display_details.dart';

class NativeMediaManagerHandle implements MediaManagerHandle {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final frb.MediaManagerHandle opaque;

  /// Creates a new [MediaManagerHandle] backed by the Rust struct behind the
  /// provided [frb.MediaManagerHandle].
  NativeMediaManagerHandle(frb.MediaManagerHandle mediaManager)
      : opaque = mediaManager {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<List<LocalMediaTrack>> initLocalTracks(
      base_settings.MediaStreamSettings caps) async {
    Pointer tracks;
    tracks = await (frb.mediaManagerHandleInitLocalTracks(
        manager: opaque,
        caps: (caps as MediaStreamSettings).setting) as Future) as Pointer;

    return frb
        .vecLocalTracksFromPtr(ptr: BigInt.from(tracks.address))
        .map((track) => NativeLocalMediaTrack(track))
        .toList();
  }

  @override
  Future<List<MediaDeviceDetails>> enumerateDevices() async {
    Pointer devices;
    devices = await (frb.mediaManagerHandleEnumerateDevices(manager: opaque)
        as Future);
    return frb
        .vecMediaDeviceDetailsFromPtr(ptr: BigInt.from(devices.address))
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
    devices = await (frb.mediaManagerHandleEnumerateDisplays(manager: opaque)
        as Future) as Pointer;

    return frb
        .vecMediaDisplayDetailsFromPtr(ptr: BigInt.from(devices.address))
        .map((info) => NativeMediaDisplayDetails(info))
        .toList();
  }

  @override
  Future<void> setOutputAudioId(String deviceId) async {
    await (frb.mediaManagerHandleSetOutputAudioId(
        manager: opaque, deviceId: deviceId) as Future);
  }

  @override
  Future<void> setMicrophoneVolume(int level) async {
    await (frb.mediaManagerHandleSetMicrophoneVolume(
        manager: opaque, level: level) as Future);
  }

  @override
  Future<int> microphoneVolume() async {
    return await (frb.mediaManagerHandleMicrophoneVolume(manager: opaque)
        as Future) as int;
  }

  @override
  Future<bool> microphoneVolumeIsAvailable() async {
    return await (frb.mediaManagerHandleMicrophoneVolumeIsAvailable(
        manager: opaque) as Future) as bool;
  }

  @override
  void onDeviceChange(void Function() cb) {
    frb.mediaManagerHandleOnDeviceChange(manager: opaque, cb: cb);
  }

  @moveSemantics
  @override
  void free() {
    if (!opaque.isDisposed) {
      RustHandlesStorage().removeHandle(this);

      opaque.dispose();
    }
  }
}
