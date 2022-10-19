import 'dart:ffi';

import 'package:ffi/ffi.dart';

import '../interface/connection_handle.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/media_track.dart';
import '../interface/reconnect_handle.dart';
import '../interface/room_close_reason.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'connection_handle.dart';
import 'ffi/foreign_value.dart';
import 'ffi/nullable_pointer.dart';
import 'ffi/result.dart';
import 'jason.dart';
import 'local_media_track.dart';
import 'media_stream_settings.dart';
import 'reconnect_handle.dart';
import 'room_close_reason.dart';

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

typedef _onNewConnection_C = Result Function(Pointer, Handle);
typedef _onNewConnection_Dart = Result Function(
    Pointer, void Function(Pointer));

typedef _onClose_C = Result Function(Pointer, Handle);
typedef _onClose_Dart = Result Function(Pointer, void Function(Pointer));

typedef _onLocalTrack_C = Result Function(Pointer, Handle);
typedef _onLocalTrack_Dart = Result Function(Pointer, void Function(Pointer));

typedef _onConnectionLoss_C = Result Function(Pointer, Handle);
typedef _onConnectionLoss_Dart = Result Function(
    Pointer, void Function(Pointer));

typedef _onFailedLocalMedia_C = Result Function(Pointer, Handle);
typedef _onFailedLocalMedia_Dart = Result Function(
    Pointer, void Function(Pointer<Handle>));

typedef _join_C = Handle Function(Pointer, Pointer<Utf8>);
typedef _join_Dart = Object Function(Pointer, Pointer<Utf8>);

typedef _setLocalMediaSettings_C = Handle Function(
    Pointer, Pointer, Uint8, Uint8);
typedef _setLocalMediaSettings_Dart = Object Function(
    Pointer, Pointer, int, int);

typedef _muteAudio_C = Handle Function(Pointer);
typedef _muteAudio_Dart = Object Function(Pointer);

typedef _unmuteAudio_C = Handle Function(Pointer);
typedef _unmuteAudio_Dart = Object Function(Pointer);

typedef _muteVideo_C = Handle Function(Pointer, ForeignValue);
typedef _muteVideo_Dart = Object Function(Pointer, ForeignValue);

typedef _unmuteVideo_C = Handle Function(Pointer, ForeignValue);
typedef _unmuteVideo_Dart = Object Function(Pointer, ForeignValue);

typedef _disableVideo_C = Handle Function(Pointer, ForeignValue);
typedef _disableVideo_Dart = Object Function(Pointer, ForeignValue);

typedef _enableVideo_C = Handle Function(Pointer, ForeignValue);
typedef _enableVideo_Dart = Object Function(Pointer, ForeignValue);

typedef _disableAudio_C = Handle Function(Pointer);
typedef _disableAudio_Dart = Object Function(Pointer);

typedef _enableAudio_C = Handle Function(Pointer);
typedef _enableAudio_Dart = Object Function(Pointer);

typedef _disableRemoteAudio_C = Handle Function(Pointer);
typedef _disableRemoteAudio_Dart = Object Function(Pointer);

typedef _enableRemoteAudio_C = Handle Function(Pointer);
typedef _enableRemoteAudio_Dart = Object Function(Pointer);

typedef _disableRemoteVideo_C = Handle Function(Pointer, ForeignValue);
typedef _disableRemoteVideo_Dart = Object Function(Pointer, ForeignValue);

typedef _enableRemoteVideo_C = Handle Function(Pointer, ForeignValue);
typedef _enableRemoteVideo_Dart = Object Function(Pointer, ForeignValue);

final _free = dl.lookupFunction<_free_C, _free_Dart>('RoomHandle__free');

final _onNewConnection =
    dl.lookupFunction<_onNewConnection_C, _onNewConnection_Dart>(
        'RoomHandle__on_new_connection');

final _onClose =
    dl.lookupFunction<_onClose_C, _onClose_Dart>('RoomHandle__on_close');

final _onLocalTrack = dl.lookupFunction<_onLocalTrack_C, _onLocalTrack_Dart>(
    'RoomHandle__on_local_track');

final _onConnectionLoss =
    dl.lookupFunction<_onConnectionLoss_C, _onConnectionLoss_Dart>(
        'RoomHandle__on_connection_loss');

final _onFailedLocalMedia =
    dl.lookupFunction<_onFailedLocalMedia_C, _onFailedLocalMedia_Dart>(
        'RoomHandle__on_failed_local_media');

final _join = dl.lookupFunction<_join_C, _join_Dart>('RoomHandle__join');

final _setLocalMediaSettings =
    dl.lookupFunction<_setLocalMediaSettings_C, _setLocalMediaSettings_Dart>(
        'RoomHandle__set_local_media_settings');

final _muteAudio =
    dl.lookupFunction<_muteAudio_C, _muteAudio_Dart>('RoomHandle__mute_audio');

final _unmuteAudio = dl.lookupFunction<_unmuteAudio_C, _unmuteAudio_Dart>(
    'RoomHandle__unmute_audio');

final _muteVideo =
    dl.lookupFunction<_muteVideo_C, _muteVideo_Dart>('RoomHandle__mute_video');

final _unmuteVideo = dl.lookupFunction<_unmuteVideo_C, _unmuteVideo_Dart>(
    'RoomHandle__unmute_video');

final _disableVideo = dl.lookupFunction<_disableVideo_C, _disableVideo_Dart>(
    'RoomHandle__disable_video');

final _enableVideo = dl.lookupFunction<_enableVideo_C, _enableVideo_Dart>(
    'RoomHandle__enable_video');

final _disableAudio = dl.lookupFunction<_disableAudio_C, _disableAudio_Dart>(
    'RoomHandle__disable_audio');

final _enableAudio = dl.lookupFunction<_enableAudio_C, _enableAudio_Dart>(
    'RoomHandle__enable_audio');

final _disableRemoteAudio =
    dl.lookupFunction<_disableRemoteAudio_C, _disableRemoteAudio_Dart>(
        'RoomHandle__disable_remote_audio');

final _enableRemoteAudio =
    dl.lookupFunction<_enableRemoteAudio_C, _enableRemoteAudio_Dart>(
        'RoomHandle__enable_remote_audio');

final _disableRemoteVideo =
    dl.lookupFunction<_disableRemoteVideo_C, _disableRemoteVideo_Dart>(
        'RoomHandle__disable_remote_video');

final _enableRemoteVideo =
    dl.lookupFunction<_enableRemoteVideo_C, _enableRemoteVideo_Dart>(
        'RoomHandle__enable_remote_video');

class NativeRoomHandle extends RoomHandle {
  /// [Pointer] to the Rust struct that backing this object.
  late NullablePointer ptr;

  /// Constructs a new [RoomHandle] backed by the Rust struct behind the
  /// provided [Pointer].
  NativeRoomHandle(this.ptr) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<void> join(String token) async {
    var tokenPtr = token.toNativeUtf8();
    try {
      await (_join(ptr.getInnerPtr(), tokenPtr) as Future);
    } finally {
      calloc.free(tokenPtr);
    }
  }

  @override
  Future<void> setLocalMediaSettings(base_settings.MediaStreamSettings settings,
      bool stopFirst, bool rollbackOnFail) async {
    await (_setLocalMediaSettings(
        ptr.getInnerPtr(),
        (settings as MediaStreamSettings).ptr.getInnerPtr(),
        stopFirst ? 1 : 0,
        rollbackOnFail ? 1 : 0) as Future);
  }

  @override
  Future<void> muteAudio() async {
    await (_muteAudio(ptr.getInnerPtr()) as Future);
  }

  @override
  Future<void> unmuteAudio() async {
    await (_unmuteAudio(ptr.getInnerPtr()) as Future);
  }

  @override
  Future<void> enableAudio() async {
    await (_enableAudio(ptr.getInnerPtr()) as Future);
  }

  @override
  Future<void> disableAudio() async {
    await (_disableAudio(ptr.getInnerPtr()) as Future);
  }

  @override
  Future<void> muteVideo([MediaSourceKind? kind]) async {
    var kind_arg =
        kind == null ? ForeignValue.none() : ForeignValue.fromInt(kind.index);
    try {
      await (_muteVideo(ptr.getInnerPtr(), kind_arg.ref) as Future);
    } finally {
      kind_arg.free();
    }
  }

  @override
  Future<void> unmuteVideo([MediaSourceKind? kind]) async {
    var kind_arg =
        kind == null ? ForeignValue.none() : ForeignValue.fromInt(kind.index);
    try {
      await (_unmuteVideo(ptr.getInnerPtr(), kind_arg.ref) as Future);
    } finally {
      kind_arg.free();
    }
  }

  @override
  Future<void> enableVideo([MediaSourceKind? kind]) async {
    var kind_arg =
        kind == null ? ForeignValue.none() : ForeignValue.fromInt(kind.index);
    try {
      await (_enableVideo(ptr.getInnerPtr(), kind_arg.ref) as Future);
    } finally {
      kind_arg.free();
    }
  }

  @override
  Future<void> disableVideo([MediaSourceKind? kind]) async {
    var kind_arg =
        kind == null ? ForeignValue.none() : ForeignValue.fromInt(kind.index);
    try {
      await (_disableVideo(ptr.getInnerPtr(), kind_arg.ref) as Future);
    } finally {
      kind_arg.free();
    }
  }

  @override
  Future<void> enableRemoteAudio() async {
    await (_enableRemoteAudio(ptr.getInnerPtr()) as Future);
  }

  @override
  Future<void> disableRemoteAudio() async {
    await (_disableRemoteAudio(ptr.getInnerPtr()) as Future);
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
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
    var kind_arg =
        kind == null ? ForeignValue.none() : ForeignValue.fromInt(kind.index);
    try {
      await (_disableRemoteVideo(ptr.getInnerPtr(), kind_arg.ref) as Future);
    } finally {
      kind_arg.free();
    }
  }

  @override
  void onNewConnection(void Function(ConnectionHandle) f) {
    _onNewConnection(ptr.getInnerPtr(), (t) {
      f(NativeConnectionHandle(NullablePointer(t)));
    }).unwrap();
  }

  @override
  void onClose(void Function(RoomCloseReason) f) {
    _onClose(ptr.getInnerPtr(), (t) {
      f(NativeRoomCloseReason(NullablePointer(t)));
    }).unwrap();
  }

  @override
  void onLocalTrack(void Function(LocalMediaTrack) f) {
    _onLocalTrack(ptr.getInnerPtr(), (t) {
      f(NativeLocalMediaTrack(NullablePointer(t)));
    }).unwrap();
  }

  @override
  void onConnectionLoss(void Function(ReconnectHandle) f) {
    _onConnectionLoss(ptr.getInnerPtr(), (t) {
      f(NativeReconnectHandle(NullablePointer(t)));
    }).unwrap();
  }

  @override
  void onFailedLocalMedia(void Function(Object) f) {
    _onFailedLocalMedia(ptr.getInnerPtr(), (err) {
      f(err);
    }).unwrap();
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
