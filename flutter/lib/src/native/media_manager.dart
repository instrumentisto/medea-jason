import 'dart:ffi';
import 'dart:io';

import '../../medea_jason.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/frb/frb.dart' as frb;
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
    base_settings.MediaStreamSettings caps,
  ) async {
    Pointer tracks;
    tracks =
        await (opaque.inner.initLocalTracks(
                  caps: (caps as MediaStreamSettings).setting,
                )
                as Future)
            as Pointer;

    return frb
        .vecLocalTracksFromRaw(ptr: tracks.address)
        .map((track) => NativeLocalMediaTrack(track))
        .toList();
  }

  @override
  Future<List<MediaDeviceDetails>> enumerateDevices() async {
    Pointer devices;
    devices = await (opaque.inner.enumerateDevices() as Future);
    return frb
        .vecMediaDeviceDetailsFromRaw(ptr: devices.address)
        .map((info) => NativeMediaDeviceDetails(info))
        .toList();
  }

  @override
  Future<List<MediaDisplayDetails>> enumerateDisplays() async {
    if (!(Platform.isLinux || Platform.isWindows || Platform.isMacOS)) {
      throw UnsupportedError(
        'enumerateDisplays() is not supported on ${Platform.operatingSystem}',
      );
    }

    Pointer devices;
    devices = await (opaque.inner.enumerateDisplays() as Future) as Pointer;

    return frb
        .vecMediaDisplayDetailsFromRaw(ptr: devices.address)
        .map((info) => NativeMediaDisplayDetails(info))
        .toList();
  }

  @override
  Future<void> setOutputAudioId(String deviceId) async {
    await (opaque.inner.setOutputAudioId(deviceId: deviceId) as Future);
  }

  @override
  Future<void> setMicrophoneVolume(int level) async {
    await (opaque.inner.setMicrophoneVolume(level: level) as Future);
  }

  @override
  Future<int> microphoneVolume() async {
    return await (opaque.inner.microphoneVolume() as Future) as int;
  }

  @override
  Future<bool> microphoneVolumeIsAvailable() async {
    return await (opaque.inner.microphoneVolumeIsAvailable() as Future) as bool;
  }

  @override
  void onDeviceChange(void Function() cb) {
    opaque.inner.onDeviceChange(cb: cb);
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
