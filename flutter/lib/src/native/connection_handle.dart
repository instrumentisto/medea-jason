import 'dart:ffi';

import '../interface/connection_handle.dart';
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/foreign_value.dart';
import 'ffi/nullable_pointer.dart';
import 'ffi/result.dart';
import 'jason.dart';
import 'remote_media_track.dart';
import 'ffi/api_api.g.dart' as api;

typedef _getRemoteMemberId_C = Result Function(Pointer);
typedef _getRemoteMemberId_Dart = Result Function(Pointer);

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

typedef _onClose_C = Result Function(Pointer, Handle);
typedef _onClose_Dart = Result Function(Pointer, void Function());

typedef _onRemoteTrackAdded_C = Result Function(Pointer, Handle);
typedef _onRemoteTrackAdded_Dart = Result Function(
    Pointer, void Function(Pointer));

typedef _onQualityScoreUpdate_C = Result Function(Pointer, Handle);
typedef _onQualityScoreUpdate_Dart = Result Function(
    Pointer, void Function(int));

typedef _disableRemoteAudio_C = Handle Function(Pointer);
typedef _disableRemoteAudio_Dart = Object Function(Pointer);

typedef _enableRemoteAudio_C = Handle Function(Pointer);
typedef _enableRemoteAudio_Dart = Object Function(Pointer);

typedef _disableRemoteVideo_C = Handle Function(Pointer, ForeignValue);
typedef _disableRemoteVideo_Dart = Object Function(Pointer, ForeignValue);

typedef _enableRemoteVideo_C = Handle Function(Pointer, ForeignValue);
typedef _enableRemoteVideo_Dart = Object Function(Pointer, ForeignValue);

final _getRemoteMemberId =
    dl.lookupFunction<_getRemoteMemberId_C, _getRemoteMemberId_Dart>(
        'ConnectionHandle__get_remote_member_id');

final _free = dl.lookupFunction<_free_C, _free_Dart>('ConnectionHandle__free');

final _onClose =
    dl.lookupFunction<_onClose_C, _onClose_Dart>('ConnectionHandle__on_close');

final _onRemoteTrackAdded =
    dl.lookupFunction<_onRemoteTrackAdded_C, _onRemoteTrackAdded_Dart>(
        'ConnectionHandle__on_remote_track_added');

final _onQualityScoreUpdate =
    dl.lookupFunction<_onQualityScoreUpdate_C, _onQualityScoreUpdate_Dart>(
        'ConnectionHandle__on_quality_score_update');

final _disableRemoteAudio =
    dl.lookupFunction<_disableRemoteAudio_C, _disableRemoteAudio_Dart>(
        'ConnectionHandle__disable_remote_audio');

final _enableRemoteAudio =
    dl.lookupFunction<_enableRemoteAudio_C, _enableRemoteAudio_Dart>(
        'ConnectionHandle__enable_remote_audio');

final _disableRemoteVideo =
    dl.lookupFunction<_disableRemoteVideo_C, _disableRemoteVideo_Dart>(
        'ConnectionHandle__disable_remote_video');

final _enableRemoteVideo =
    dl.lookupFunction<_enableRemoteVideo_C, _enableRemoteVideo_Dart>(
        'ConnectionHandle__enable_remote_video');

class NativeConnectionHandle extends ConnectionHandle {
  /// [Pointer] to the Rust struct backing this object.
  late NullablePointer ptr;
  late api.ConnectionHandle opaque;

  /// Constructs a new [ConnectionHandle] backed by a Rust struct behind the
  /// provided [Pointer].
  NativeConnectionHandle(this.ptr) {
    RustHandlesStorage().insertHandle(this);
  }

  NativeConnectionHandle.opaque(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String getRemoteMemberId() {
    impl_api.connectionHandleGetRemoteMemberId(connection: opaque);

    return _getRemoteMemberId(ptr.getInnerPtr()).unwrap();
  }

  @override
  void onClose(void Function() f) {
    impl_api.connectionHandleOnClose(connection: opaque, f: handle2rust(f));

    _onClose(ptr.getInnerPtr(), f).unwrap();
  }

  @override
  void onRemoteTrackAdded(void Function(RemoteMediaTrack) f) {
    impl_api.connectionHandleOnRemoteTrackAdded(
        connection: opaque, f: handle2rust(f));

    _onRemoteTrackAdded(ptr.getInnerPtr(), (t) {
      f(NativeRemoteMediaTrack(NullablePointer(t)));
    }).unwrap();
  }

  @override
  void onQualityScoreUpdate(void Function(int) f) {
    impl_api.connectionHandleOnQualityScoreUpdate(
        connection: opaque, f: handle2rust(f));

    _onQualityScoreUpdate(ptr.getInnerPtr(), f).unwrap();
  }

  @moveSemantics
  @override
  void free() {
    if (!ptr.isFreed()) {
      RustHandlesStorage().removeHandle(this);
      _free(ptr.getInnerPtr());
      ptr.free();

      opaque.dispose();
    }
  }

  @override
  Future<void> enableRemoteAudio() async {
    await rust2dart(
        impl_api.connectionHandleEnableRemoteAudio(connection: opaque));

    await (_enableRemoteAudio(ptr.getInnerPtr()) as Future);
  }

  @override
  Future<void> disableRemoteAudio() async {
    await rust2dart(
        impl_api.connectionHandleDisableRemoteAudio(connection: opaque));

    await (_disableRemoteAudio(ptr.getInnerPtr()) as Future);
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await rust2dart(impl_api.connectionHandleEnableRemoteVideo(
        connection: opaque, sourceKind: kind?.index));

    var kind_arg =
        kind == null ? ForeignValue.none() : ForeignValue.fromInt(kind.index);
    try {
      await (_enableRemoteVideo(ptr.getInnerPtr(), kind_arg.ref) as Future);
    } finally {
      kind_arg.free();
    }
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await rust2dart(impl_api.connectionHandleDisableRemoteVideo(
        connection: opaque, sourceKind: kind?.index));

    var kind_arg =
        kind == null ? ForeignValue.none() : ForeignValue.fromInt(kind.index);
    try {
      await (_disableRemoteVideo(ptr.getInnerPtr(), kind_arg.ref) as Future);
    } finally {
      kind_arg.free();
    }
  }
}
