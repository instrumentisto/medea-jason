import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import '../interface/connection_handle.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/media_track.dart';
import '../interface/reconnect_handle.dart';
import '../interface/room_close_reason.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'connection_handle.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';
import 'local_media_track.dart';
import 'media_stream_settings.dart';
import 'reconnect_handle.dart';
import 'room_close_reason.dart';

class NativeRoomHandle extends RoomHandle {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late frb.RoomHandle opaque;

  /// Constructs a new [RoomHandle] backed by the Rust struct behind the
  /// provided [frb.RoomHandle].
  NativeRoomHandle(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<void> join(String token) async {
    await (api.roomHandleJoin(roomHandle: opaque, token: token) as Future);
  }

  @override
  Future<void> setLocalMediaSettings(base_settings.MediaStreamSettings settings,
      bool stopFirst, bool rollbackOnFail) async {
    await (api.roomHandleSetLocalMediaSettings(
        roomHandle: opaque,
        settings: (settings as MediaStreamSettings).opaque,
        stopFirst: stopFirst,
        rollbackOnFail: rollbackOnFail) as Future);
  }

  @override
  Future<void> muteAudio() async {
    await (api.roomHandleMuteAudio(roomHandle: opaque) as Future);
  }

  @override
  Future<void> unmuteAudio() async {
    await (api.roomHandleUnmuteAudio(roomHandle: opaque) as Future);
  }

  @override
  Future<void> enableAudio() async {
    await (api.roomHandleEnableAudio(roomHandle: opaque) as Future);
  }

  @override
  Future<void> disableAudio() async {
    await (api.roomHandleDisableAudio(roomHandle: opaque) as Future);
  }

  @override
  Future<void> muteVideo([MediaSourceKind? kind]) async {
    await (api.roomHandleMuteVideo(roomHandle: opaque, sourceKind: kind?.index)
        as Future);
  }

  @override
  Future<void> unmuteVideo([MediaSourceKind? kind]) async {
    await (api.roomHandleUnmuteVideo(
        roomHandle: opaque, sourceKind: kind?.index) as Future);
  }

  @override
  Future<void> enableVideo([MediaSourceKind? kind]) async {
    await (api.roomHandleEnableVideo(
        roomHandle: opaque, sourceKind: kind?.index) as Future);
  }

  @override
  Future<void> disableVideo([MediaSourceKind? kind]) async {
    await (api.roomHandleDisableVideo(
        roomHandle: opaque, sourceKind: kind?.index) as Future);
  }

  @override
  Future<void> enableRemoteAudio() async {
    await (api.roomHandleEnableRemoteAudio(roomHandle: opaque) as Future);
  }

  @override
  Future<void> disableRemoteAudio() async {
    await (api.roomHandleDisableRemoteAudio(roomHandle: opaque) as Future);
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await (api.roomHandleEnableRemoteVideo(
        roomHandle: opaque, sourceKind: kind?.index) as Future);
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await (api.roomHandleDisableRemoteVideo(
        roomHandle: opaque, sourceKind: kind?.index) as Future);
  }

  @override
  void onNewConnection(void Function(ConnectionHandle) f) {
    try {
      api.roomHandleOnNewConnection(
          roomHandle: opaque,
          cb: (t) {
            f(NativeConnectionHandle(
                api.connectionHandleFromPtr(ptr: t.address)));
          });
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void onClose(void Function(RoomCloseReason) f) {
    print('RAZ');
    try {
      print('dwa');
      api.roomHandleOnClose(
          roomHandle: opaque,
          cb: (t) {
            f(NativeRoomCloseReason(
                api.roomCloseReasonFromPtr(ptr: t.address)));
          });
    print('TREE');
    } on FfiException catch (anyhow) {
    print('CHET');
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void onLocalTrack(void Function(LocalMediaTrack) f) {
    try {
      api.roomHandleOnLocalTrack(
          roomHandle: opaque,
          cb: (t) {
            f(NativeLocalMediaTrack(
                api.localMediaTrackFromPtr(ptr: t.address)));
          });
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void onConnectionLoss(void Function(ReconnectHandle) f) {
    try {
      api.roomHandleOnConnectionLoss(
          roomHandle: opaque,
          cb: (t) {
            f(NativeReconnectHandle(
                api.reconnectHandleFromPtr(ptr: t.address)));
          });
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void onFailedLocalMedia(void Function(Object) f) {
    try {
      api.roomHandleOnFailedLocalMedia(
          roomHandle: opaque,
          cb: (err) {
            f(err);
          });
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
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
