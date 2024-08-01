import '../interface/connection_handle.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/media_track.dart';
import '../interface/reconnect_handle.dart';
import '../interface/room_close_reason.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'connection_handle.dart';
import 'ffi/frb/api/dart/api.dart' as frb;
import 'local_media_track.dart';
import 'media_stream_settings.dart';
import 'reconnect_handle.dart';
import 'room_close_reason.dart';

class NativeRoomHandle implements RoomHandle {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final frb.RoomHandle opaque;

  /// Constructs a new [RoomHandle] backed by the Rust struct behind the
  /// provided [frb.RoomHandle].
  NativeRoomHandle(frb.RoomHandle roomHandle) : opaque = roomHandle {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<void> join(String token) async {
    await (frb.roomHandleJoin(roomHandle: opaque, token: token) as Future);
  }

  @override
  Future<void> setLocalMediaSettings(base_settings.MediaStreamSettings settings,
      bool stopFirst, bool rollbackOnFail) async {
    await (frb.roomHandleSetLocalMediaSettings(
        roomHandle: opaque,
        settings: (settings as MediaStreamSettings).setting,
        stopFirst: stopFirst,
        rollbackOnFail: rollbackOnFail) as Future);
  }

  @override
  Future<void> muteAudio() async {
    await (frb.roomHandleMuteAudio(roomHandle: opaque) as Future);
  }

  @override
  Future<void> unmuteAudio() async {
    await (frb.roomHandleUnmuteAudio(roomHandle: opaque) as Future);
  }

  @override
  Future<void> enableAudio() async {
    await (frb.roomHandleEnableAudio(roomHandle: opaque) as Future);
  }

  @override
  Future<void> disableAudio() async {
    await (frb.roomHandleDisableAudio(roomHandle: opaque) as Future);
  }

  @override
  Future<void> muteVideo([MediaSourceKind? kind]) async {
    await (frb.roomHandleMuteVideo(roomHandle: opaque, sourceKind: kind)
        as Future);
  }

  @override
  Future<void> unmuteVideo([MediaSourceKind? kind]) async {
    await (frb.roomHandleUnmuteVideo(roomHandle: opaque, sourceKind: kind)
        as Future);
  }

  @override
  Future<void> enableVideo([MediaSourceKind? kind]) async {
    await (frb.roomHandleEnableVideo(roomHandle: opaque, sourceKind: kind)
        as Future);
  }

  @override
  Future<void> disableVideo([MediaSourceKind? kind]) async {
    await (frb.roomHandleDisableVideo(roomHandle: opaque, sourceKind: kind)
        as Future);
  }

  @override
  Future<void> enableRemoteAudio() async {
    await (frb.roomHandleEnableRemoteAudio(roomHandle: opaque) as Future);
  }

  @override
  Future<void> disableRemoteAudio() async {
    await (frb.roomHandleDisableRemoteAudio(roomHandle: opaque) as Future);
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await (frb.roomHandleEnableRemoteVideo(roomHandle: opaque, sourceKind: kind)
        as Future);
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await (frb.roomHandleDisableRemoteVideo(
        roomHandle: opaque, sourceKind: kind) as Future);
  }

  @override
  void onNewConnection(void Function(ConnectionHandle) f) {
    frb.roomHandleOnNewConnection(
        roomHandle: opaque,
        cb: (t) {
          f(NativeConnectionHandle(
              frb.connectionHandleFromPtr(ptr: t.address)));
        });
  }

  @override
  void onClose(void Function(RoomCloseReason) f) {
    frb.roomHandleOnClose(
        roomHandle: opaque,
        cb: (t) {
          f(NativeRoomCloseReason(frb.roomCloseReasonFromPtr(ptr: t.address)));
        });
  }

  @override
  void onLocalTrack(void Function(LocalMediaTrack) f) {
    frb.roomHandleOnLocalTrack(
        roomHandle: opaque,
        cb: (t) {
          f(NativeLocalMediaTrack(frb.localMediaTrackFromPtr(ptr: t.address)));
        });
  }

  @override
  void onConnectionLoss(void Function(ReconnectHandle) f) {
    frb.roomHandleOnConnectionLoss(
        roomHandle: opaque,
        cb: (t) {
          f(NativeReconnectHandle(frb.reconnectHandleFromPtr(ptr: t.address)));
        });
  }

  @override
  void onFailedLocalMedia(void Function(Object) f) {
    frb.roomHandleOnFailedLocalMedia(
        roomHandle: opaque,
        cb: (err) {
          f(err);
        });
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
