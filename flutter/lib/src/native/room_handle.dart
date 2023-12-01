import '../interface/connection_handle.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/media_track.dart';
import '../interface/reconnect_handle.dart';
import '../interface/room_close_reason.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'connection_handle.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';
import 'local_media_track.dart';
import 'media_stream_settings.dart';
import 'reconnect_handle.dart';
import 'room_close_reason.dart';

class NativeRoomHandle implements RoomHandle {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final RustOpaque<frb.RoomHandle> opaque;

  /// Constructs a new [RoomHandle] backed by the Rust struct behind the
  /// provided [frb.RoomHandle].
  NativeRoomHandle(frb.RoomHandle roomHandle)
      : opaque = RustOpaque(roomHandle) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<void> join(String token) async {
    await (api.roomHandleJoin(roomHandle: opaque.innerOpaque, token: token)
        as Future);
  }

  @override
  Future<void> setLocalMediaSettings(base_settings.MediaStreamSettings settings,
      bool stopFirst, bool rollbackOnFail) async {
    await (api.roomHandleSetLocalMediaSettings(
        roomHandle: opaque.innerOpaque,
        settings: (settings as MediaStreamSettings).setting,
        stopFirst: stopFirst,
        rollbackOnFail: rollbackOnFail) as Future);
  }

  @override
  Future<void> muteAudio() async {
    await (api.roomHandleMuteAudio(roomHandle: opaque.innerOpaque) as Future);
  }

  @override
  Future<void> unmuteAudio() async {
    await (api.roomHandleUnmuteAudio(roomHandle: opaque.innerOpaque) as Future);
  }

  @override
  Future<void> enableAudio() async {
    await (api.roomHandleEnableAudio(roomHandle: opaque.innerOpaque) as Future);
  }

  @override
  Future<void> disableAudio() async {
    await (api.roomHandleDisableAudio(roomHandle: opaque.innerOpaque)
        as Future);
  }

  @override
  Future<void> muteVideo([MediaSourceKind? kind]) async {
    await (api.roomHandleMuteVideo(
        roomHandle: opaque.innerOpaque, sourceKind: kind) as Future);
  }

  @override
  Future<void> unmuteVideo([MediaSourceKind? kind]) async {
    await (api.roomHandleUnmuteVideo(
        roomHandle: opaque.innerOpaque, sourceKind: kind) as Future);
  }

  @override
  Future<void> enableVideo([MediaSourceKind? kind]) async {
    await (api.roomHandleEnableVideo(
        roomHandle: opaque.innerOpaque, sourceKind: kind) as Future);
  }

  @override
  Future<void> disableVideo([MediaSourceKind? kind]) async {
    await (api.roomHandleDisableVideo(
        roomHandle: opaque.innerOpaque, sourceKind: kind) as Future);
  }

  @override
  Future<void> enableRemoteAudio() async {
    await (api.roomHandleEnableRemoteAudio(roomHandle: opaque.innerOpaque)
        as Future);
  }

  @override
  Future<void> disableRemoteAudio() async {
    await (api.roomHandleDisableRemoteAudio(roomHandle: opaque.innerOpaque)
        as Future);
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await (api.roomHandleEnableRemoteVideo(
        roomHandle: opaque.innerOpaque, sourceKind: kind) as Future);
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await (api.roomHandleDisableRemoteVideo(
        roomHandle: opaque.innerOpaque, sourceKind: kind) as Future);
  }

  @override
  void onNewConnection(void Function(ConnectionHandle) f) {
    api.roomHandleOnNewConnection(
        roomHandle: opaque.innerOpaque,
        cb: (t) {
          f(NativeConnectionHandle(
              api.connectionHandleFromPtr(ptr: t.address)));
        });
  }

  @override
  void onClose(void Function(RoomCloseReason) f) {
    api.roomHandleOnClose(
        roomHandle: opaque.innerOpaque,
        cb: (t) {
          f(NativeRoomCloseReason(api.roomCloseReasonFromPtr(ptr: t.address)));
        });
  }

  @override
  void onLocalTrack(void Function(LocalMediaTrack) f) {
    api.roomHandleOnLocalTrack(
        roomHandle: opaque.innerOpaque,
        cb: (t) {
          f(NativeLocalMediaTrack(api.localMediaTrackFromPtr(ptr: t.address)));
        });
  }

  @override
  void onConnectionLoss(void Function(ReconnectHandle) f) {
    api.roomHandleOnConnectionLoss(
        roomHandle: opaque.innerOpaque,
        cb: (t) {
          f(NativeReconnectHandle(api.reconnectHandleFromPtr(ptr: t.address)));
        });
  }

  @override
  void onFailedLocalMedia(void Function(Object) f) {
    api.roomHandleOnFailedLocalMedia(
        roomHandle: opaque.innerOpaque,
        cb: (err) {
          f(err);
        });
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
