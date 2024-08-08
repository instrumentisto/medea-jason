import '../interface/connection_handle.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/media_track.dart';
import '../interface/reconnect_handle.dart';
import '../interface/room_close_reason.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'connection_handle.dart';
import 'ffi/frb/frb.dart' as frb;
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
    await (opaque.join(token: token) as Future);
  }

  @override
  Future<void> setLocalMediaSettings(base_settings.MediaStreamSettings settings,
      bool stopFirst, bool rollbackOnFail) async {
    await (opaque.setLocalMediaSettings(
        settings: (settings as MediaStreamSettings).setting,
        stopFirst: stopFirst,
        rollbackOnFail: rollbackOnFail) as Future);
  }

  @override
  Future<void> muteAudio() async {
    await (opaque.muteAudio() as Future);
  }

  @override
  Future<void> unmuteAudio() async {
    await (opaque.unmuteAudio() as Future);
  }

  @override
  Future<void> enableAudio() async {
    await (opaque.enableAudio() as Future);
  }

  @override
  Future<void> disableAudio() async {
    await (opaque.disableAudio() as Future);
  }

  @override
  Future<void> muteVideo([MediaSourceKind? kind]) async {
    await (opaque.muteVideo(sourceKind: kind) as Future);
  }

  @override
  Future<void> unmuteVideo([MediaSourceKind? kind]) async {
    await (opaque.unmuteVideo(sourceKind: kind) as Future);
  }

  @override
  Future<void> enableVideo([MediaSourceKind? kind]) async {
    await (opaque.enableVideo(sourceKind: kind) as Future);
  }

  @override
  Future<void> disableVideo([MediaSourceKind? kind]) async {
    await (opaque.disableVideo(sourceKind: kind) as Future);
  }

  @override
  Future<void> enableRemoteAudio() async {
    await (opaque.enableRemoteAudio() as Future);
  }

  @override
  Future<void> disableRemoteAudio() async {
    await (opaque.disableRemoteAudio() as Future);
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await (opaque.enableRemoteVideo(sourceKind: kind) as Future);
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await (opaque.disableRemoteVideo(sourceKind: kind) as Future);
  }

  @override
  void onNewConnection(void Function(ConnectionHandle) f) {
    opaque.onNewConnection(cb: (t) {
      f(NativeConnectionHandle(frb.ConnectionHandle.fromRaw(ptr: t.address)));
    });
  }

  @override
  void onClose(void Function(RoomCloseReason) f) {
    opaque.onClose(cb: (t) {
      f(NativeRoomCloseReason(frb.RoomCloseReason.fromRaw(ptr: t.address)));
    });
  }

  @override
  void onLocalTrack(void Function(LocalMediaTrack) f) {
    opaque.onLocalTrack(cb: (t) {
      f(NativeLocalMediaTrack(frb.LocalMediaTrack.fromRaw(ptr: t.address)));
    });
  }

  @override
  void onConnectionLoss(void Function(ReconnectHandle) f) {
    opaque.onConnectionLoss(cb: (t) {
      f(NativeReconnectHandle(frb.reconnectHandleFromPtr(ptr: t.address)));
    });
  }

  @override
  void onFailedLocalMedia(void Function(Object) f) {
    opaque.onFailedLocalMedia(cb: (err) {
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
