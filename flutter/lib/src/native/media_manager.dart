import 'dart:ffi';

import 'package:ffi/ffi.dart';

import '../interface/media_device_info.dart';
import '../interface/local_media_track.dart';
import '../interface/media_manager.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/nullable_pointer.dart';
import 'ffi/ptrarray.dart';
import 'ffi/result.dart';
import 'media_device_info.dart';
import 'jason.dart';
import 'local_media_track.dart';
import 'media_stream_settings.dart';

typedef _initLocalTracks_C = Handle Function(Pointer, Pointer);
typedef _initLocalTracks_Dart = Object Function(Pointer, Pointer);

typedef _enumerateDevices_C = Handle Function(Pointer);
typedef _enumerateDevices_Dart = Object Function(Pointer);

typedef _setOutputAudioId_C = Handle Function(Pointer, Pointer<Utf8>);
typedef _setOutputAudioId_Dart = Object Function(Pointer, Pointer<Utf8>);

typedef _setMicrophoneVolume_C = Handle Function(Pointer, Int64);
typedef _setMicrophoneVolume_Dart = Object Function(Pointer, int);

typedef _microphoneVolumeIsAvailable_C = Handle Function(Pointer);
typedef _microphoneVolumeIsAvailable_Dart = Object Function(Pointer);

typedef _microphoneVolume_C = Handle Function(Pointer);
typedef _microphoneVolume_Dart = Object Function(Pointer);

typedef _onDeviceChange_C = Result Function(Pointer, Handle);
typedef _onDeviceChange_Dart = Result Function(Pointer, Object);

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

final _initLocalTracks =
    dl.lookupFunction<_initLocalTracks_C, _initLocalTracks_Dart>(
        'MediaManagerHandle__init_local_tracks');

final _enumerateDevices =
    dl.lookupFunction<_enumerateDevices_C, _enumerateDevices_Dart>(
        'MediaManagerHandle__enumerate_devices');

final _setOutputAudioId =
    dl.lookupFunction<_setOutputAudioId_C, _setOutputAudioId_Dart>(
        'MediaManagerHandle__set_output_audio_id');

final _setMicrophoneVolume =
    dl.lookupFunction<_setMicrophoneVolume_C, _setMicrophoneVolume_Dart>(
        'MediaManagerHandle__set_microphone_volume');

final _microphoneVolumeIsAvailable = dl.lookupFunction<
        _microphoneVolumeIsAvailable_C, _microphoneVolumeIsAvailable_Dart>(
    'MediaManagerHandle__microphone_volume_is_available');

final _microphoneVolume =
    dl.lookupFunction<_microphoneVolume_C, _microphoneVolume_Dart>(
        'MediaManagerHandle__microphone_volume');

final _onDeviceChange =
    dl.lookupFunction<_onDeviceChange_C, _onDeviceChange_Dart>(
        'MediaManagerHandle__on_device_change');

final _free =
    dl.lookupFunction<_free_C, _free_Dart>('MediaManagerHandle__free');

class NativeMediaManagerHandle extends MediaManagerHandle {
  /// [Pointer] to the Rust struct backing this object.
  late NullablePointer ptr;

  /// Creates a new [MediaManagerHandle] backed by the Rust struct behind the
  /// provided [Pointer].
  NativeMediaManagerHandle(this.ptr) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<List<LocalMediaTrack>> initLocalTracks(
      base_settings.MediaStreamSettings caps) async {
    Pointer tracks = await (_initLocalTracks(
            ptr.getInnerPtr(), (caps as MediaStreamSettings).ptr.getInnerPtr())
        as Future);
    return tracks
        .cast<PtrArray>()
        .intoPointerList()
        .map((e) => NativeLocalMediaTrack(NullablePointer(e)))
        .toList();
  }

  @override
  Future<List<MediaDeviceInfo>> enumerateDevices() async {
    Pointer pointer = await (_enumerateDevices(ptr.getInnerPtr()) as Future);
    return pointer
        .cast<PtrArray>()
        .intoPointerList()
        .map((e) => NativeMediaDeviceInfo(NullablePointer(e)))
        .toList();
  }

  @override
  Future<void> setOutputAudioId(String deviceId) async {
    await (_setOutputAudioId(ptr.getInnerPtr(), deviceId.toNativeUtf8())
        as Future);
  }

  @override
  Future<void> setMicrophoneVolume(int level) async {
    await (_setMicrophoneVolume(ptr.getInnerPtr(), level) as Future);
  }

  @override
  Future<int> microphoneVolume() async {
    return await (_microphoneVolume(ptr.getInnerPtr()) as Future);
  }

  @override
  Future<bool> microphoneVolumeIsAvailable() async {
    return (await (_microphoneVolumeIsAvailable(ptr.getInnerPtr())
            as Future)) !=
        0;
  }

  @override
  void onDeviceChange(void Function() cb) {
    _onDeviceChange(ptr.getInnerPtr(), cb).unwrap();
  }

  @moveSemantics
  @override
  void free() {
    if (!ptr.isFreed()) {
      RustHandlesStorage().removeHandle(this);
      _free(ptr.getInnerPtr());
      ptr.free();
    }
  }
}
