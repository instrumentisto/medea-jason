import 'dart:ffi';

import '../interface/connection_handle.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/media_track.dart';
import '../interface/reconnect_handle.dart';
import '../interface/room_close_reason.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'connection_handle.dart';
import 'ffi/api_api.g.dart' as api;
import 'jason.dart';
import 'local_media_track.dart';
import 'media_stream_settings.dart';
import 'reconnect_handle.dart';
import 'room_close_reason.dart';

class NativeRoomHandle extends RoomHandle {
  /// [Pointer] to the Rust struct that backing this object.
  late api.RoomHandle opaque;

  /// Constructs a new [RoomHandle] backed by the Rust struct behind the
  /// provided [Pointer].
  NativeRoomHandle.opaque(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<void> join(String token) async {
    await rust2dart(impl_api.roomHandleJoin(roomHandle: opaque, token: token));
  }

  @override
  Future<void> setLocalMediaSettings(base_settings.MediaStreamSettings settings,
      bool stopFirst, bool rollbackOnFail) async {
    await rust2dart(impl_api.roomHandleSetLocalMediaSettings(
        roomHandle: opaque,
        settings: (settings as MediaStreamSettings).opaque,
        stopFirst: stopFirst,
        rollbackOnFail: rollbackOnFail));
  }

  @override
  Future<void> muteAudio() async {
    await rust2dart(impl_api.roomHandleMuteAudio(roomHandle: opaque));
  }

  @override
  Future<void> unmuteAudio() async {
    await rust2dart(impl_api.roomHandleUnmuteAudio(roomHandle: opaque));
  }

  @override
  Future<void> enableAudio() async {
    await rust2dart(impl_api.roomHandleEnableAudio(roomHandle: opaque));
  }

  @override
  Future<void> disableAudio() async {
    await rust2dart(impl_api.roomHandleDisableAudio(roomHandle: opaque));
  }

  @override
  Future<void> muteVideo([MediaSourceKind? kind]) async {
    await rust2dart(impl_api.roomHandleMuteVideo(
        roomHandle: opaque, sourceKind: kind?.index));
  }

  @override
  Future<void> unmuteVideo([MediaSourceKind? kind]) async {
    await rust2dart(impl_api.roomHandleUnmuteVideo(
        roomHandle: opaque, sourceKind: kind?.index));
  }

  @override
  Future<void> enableVideo([MediaSourceKind? kind]) async {
    await rust2dart(impl_api.roomHandleEnableVideo(
        roomHandle: opaque, sourceKind: kind?.index));
  }

  @override
  Future<void> disableVideo([MediaSourceKind? kind]) async {
    await rust2dart(impl_api.roomHandleDisableVideo(
        roomHandle: opaque, sourceKind: kind?.index));
  }

  @override
  Future<void> enableRemoteAudio() async {
    await rust2dart(impl_api.roomHandleEnableRemoteAudio(roomHandle: opaque));
  }

  @override
  Future<void> disableRemoteAudio() async {
    await rust2dart(impl_api.roomHandleDisableRemoteAudio(roomHandle: opaque));
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await rust2dart(impl_api.roomHandleEnableRemoteVideo(
        roomHandle: opaque, sourceKind: kind?.index));
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await rust2dart(impl_api.roomHandleDisableRemoteVideo(
        roomHandle: opaque, sourceKind: kind?.index));
  }

  @override
  void onNewConnection(void Function(ConnectionHandle) f) {
    var _f = handle2rust((t) {
      f(NativeConnectionHandle.opaque(impl_api.connectionHandleFromPtr(ptr: t.address)));
    });
    impl_api.roomHandleOnNewConnection(roomHandle: opaque, cb: _f);
  }

  @override
  void onClose(void Function(RoomCloseReason) f) {
    var _f = handle2rust((t) {
      f(NativeRoomCloseReason.opaque(impl_api.roomCloseReasonFromPtr(ptr: t.address)));
    });
    impl_api.roomHandleOnClose(roomHandle: opaque, cb: _f);
  }

  @override
  void onLocalTrack(void Function(LocalMediaTrack) f) {
    var _f = handle2rust((t) {
      f(NativeLocalMediaTrack.opaque(impl_api.localMediaTrackFromPtr(ptr: t.address)));
    });
    impl_api.roomHandleOnLocalTrack(roomHandle: opaque, cb: _f);
  }

  @override
  void onConnectionLoss(void Function(ReconnectHandle) f) {
    var _f = handle2rust((t) {
      f(NativeReconnectHandle.opaque(impl_api.reconnectHandleFromPtr(ptr: t.address)));
    });
    impl_api.roomHandleOnConnectionLoss(roomHandle: opaque, cb: _f);
  }

  @override
  void onFailedLocalMedia(void Function(Object) f) {
    var _f = handle2rust((err) {
      f(err);
    });
    impl_api.roomHandleOnFailedLocalMedia(roomHandle: opaque, cb: _f);
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
