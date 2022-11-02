import '../interface/connection_handle.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/media_track.dart';
import '../interface/reconnect_handle.dart';
import '../interface/room_close_reason.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'connection_handle.dart';
import 'ffi/api_api.g.dart' as frb;
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
    await rust_future_to_dart_future(
        api.roomHandleJoin(roomHandle: opaque, token: token));
  }

  @override
  Future<void> setLocalMediaSettings(base_settings.MediaStreamSettings settings,
      bool stopFirst, bool rollbackOnFail) async {
    await rust_future_to_dart_future(api.roomHandleSetLocalMediaSettings(
        roomHandle: opaque,
        settings: (settings as MediaStreamSettings).opaque,
        stopFirst: stopFirst,
        rollbackOnFail: rollbackOnFail));
  }

  @override
  Future<void> muteAudio() async {
    await rust_future_to_dart_future(
        api.roomHandleMuteAudio(roomHandle: opaque));
  }

  @override
  Future<void> unmuteAudio() async {
    await rust_future_to_dart_future(
        api.roomHandleUnmuteAudio(roomHandle: opaque));
  }

  @override
  Future<void> enableAudio() async {
    await rust_future_to_dart_future(
        api.roomHandleEnableAudio(roomHandle: opaque));
  }

  @override
  Future<void> disableAudio() async {
    await rust_future_to_dart_future(
        api.roomHandleDisableAudio(roomHandle: opaque));
  }

  @override
  Future<void> muteVideo([MediaSourceKind? kind]) async {
    await rust_future_to_dart_future(
        api.roomHandleMuteVideo(roomHandle: opaque, sourceKind: kind?.index));
  }

  @override
  Future<void> unmuteVideo([MediaSourceKind? kind]) async {
    await rust_future_to_dart_future(
        api.roomHandleUnmuteVideo(roomHandle: opaque, sourceKind: kind?.index));
  }

  @override
  Future<void> enableVideo([MediaSourceKind? kind]) async {
    await rust_future_to_dart_future(
        api.roomHandleEnableVideo(roomHandle: opaque, sourceKind: kind?.index));
  }

  @override
  Future<void> disableVideo([MediaSourceKind? kind]) async {
    await rust_future_to_dart_future(api.roomHandleDisableVideo(
        roomHandle: opaque, sourceKind: kind?.index));
  }

  @override
  Future<void> enableRemoteAudio() async {
    await rust_future_to_dart_future(
        api.roomHandleEnableRemoteAudio(roomHandle: opaque));
  }

  @override
  Future<void> disableRemoteAudio() async {
    await rust_future_to_dart_future(
        api.roomHandleDisableRemoteAudio(roomHandle: opaque));
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await rust_future_to_dart_future(api.roomHandleEnableRemoteVideo(
        roomHandle: opaque, sourceKind: kind?.index));
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await rust_future_to_dart_future(api.roomHandleDisableRemoteVideo(
        roomHandle: opaque, sourceKind: kind?.index));
  }

  @override
  void onNewConnection(void Function(ConnectionHandle) f) {
    api.roomHandleOnNewConnection(
        roomHandle: opaque,
        cb: dart_object_to_persistent_rust_opaque((t) {
          f(NativeConnectionHandle(
              api.connectionHandleFromPtr(ptr: t.address)));
        }));
  }

  @override
  void onClose(void Function(RoomCloseReason) f) {
    api.roomHandleOnClose(
        roomHandle: opaque,
        cb: dart_object_to_persistent_rust_opaque((t) {
          f(NativeRoomCloseReason(api.roomCloseReasonFromPtr(ptr: t.address)));
        }));
  }

  @override
  void onLocalTrack(void Function(LocalMediaTrack) f) {
    api.roomHandleOnLocalTrack(
        roomHandle: opaque,
        cb: dart_object_to_persistent_rust_opaque((t) {
          f(NativeLocalMediaTrack(api.localMediaTrackFromPtr(ptr: t.address)));
        }));
  }

  @override
  void onConnectionLoss(void Function(ReconnectHandle) f) {
    api.roomHandleOnConnectionLoss(
        roomHandle: opaque,
        cb: dart_object_to_persistent_rust_opaque((t) {
          f(NativeReconnectHandle(api.reconnectHandleFromPtr(ptr: t.address)));
        }));
  }

  @override
  void onFailedLocalMedia(void Function(Object) f) {
    api.roomHandleOnFailedLocalMedia(
        roomHandle: opaque,
        cb: dart_object_to_persistent_rust_opaque((err) {
          f(err);
        }));
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
