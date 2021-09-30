import 'dart:ffi';

import '../ffi/ptrarray.dart';
import 'input_device_info.dart';
import '../interface/input_device_info.dart';
import '../interface/local_media_track.dart';
import '../interface/media_manager.dart';
import '../interface/media_stream_settings.dart';
import 'jason.dart';
import 'local_media_track.dart';
import 'media_stream_settings.dart';
import '../util/move_semantic.dart';
import '../util/nullable_pointer.dart';

typedef _initLocalTracks_C = Handle Function(Pointer, Pointer);
typedef _initLocalTracks_Dart = Object Function(Pointer, Pointer);

typedef _enumerateDevices_C = Handle Function(Pointer);
typedef _enumerateDevices_Dart = Object Function(Pointer);

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

final _initLocalTracks =
    dl.lookupFunction<_initLocalTracks_C, _initLocalTracks_Dart>(
        'MediaManagerHandle__init_local_tracks');

final _enumerateDevices =
    dl.lookupFunction<_enumerateDevices_C, _enumerateDevices_Dart>(
        'MediaManagerHandle__enumerate_devices');

final _free =
    dl.lookupFunction<_free_C, _free_Dart>('MediaManagerHandle__free');

class NativeMediaManagerHandle extends MediaManagerHandle {
  /// [Pointer] to the Rust struct backing this object.
  late NullablePointer ptr;

  /// Creates a new [MediaManagerHandle] backed by the Rust struct behind the
  /// provided [Pointer].
  NativeMediaManagerHandle(this.ptr);

  @override
  Future<List<LocalMediaTrack>> initLocalTracks(
      IMediaStreamSettings caps) async {
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
  Future<List<InputDeviceInfo>> enumerateDevices() async {
    Pointer pointer = await (_enumerateDevices(ptr.getInnerPtr()) as Future);
    return pointer
        .cast<PtrArray>()
        .intoPointerList()
        .map((e) => NativeInputDeviceInfo(NullablePointer(e)))
        .toList();
  }

  @moveSemantics
  @override
  void free() {
    _free(ptr.getInnerPtr());
    ptr.free();
  }
}
