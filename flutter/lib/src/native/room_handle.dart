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
import 'ffi/frb/frb.dart' as frb;
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
    await (opaque.inner.join(token: token) as Future);
  }

  @override
  Future<void> setLocalMediaSettings(
    base_settings.MediaStreamSettings settings,
    bool stopFirst,
    bool rollbackOnFail,
  ) async {
    await (opaque.inner.setLocalMediaSettings(
          settings: (settings as MediaStreamSettings).setting,
          stopFirst: stopFirst,
          rollbackOnFail: rollbackOnFail,
        )
        as Future);
  }

  @override
  Future<void> muteAudio([MediaSourceKind? kind]) async {
    await (opaque.inner.muteAudio(sourceKind: kind) as Future);
  }

  @override
  Future<void> unmuteAudio([MediaSourceKind? kind]) async {
    await (opaque.inner.unmuteAudio(sourceKind: kind) as Future);
  }

  @override
  Future<void> enableAudio([MediaSourceKind? kind]) async {
    await (opaque.inner.enableAudio(sourceKind: kind) as Future);
  }

  @override
  Future<void> disableAudio([MediaSourceKind? kind]) async {
    await (opaque.inner.disableAudio(sourceKind: kind) as Future);
  }

  @override
  Future<void> muteVideo([MediaSourceKind? kind]) async {
    await (opaque.inner.muteVideo(sourceKind: kind) as Future);
  }

  @override
  Future<void> unmuteVideo([MediaSourceKind? kind]) async {
    await (opaque.inner.unmuteVideo(sourceKind: kind) as Future);
  }

  @override
  Future<void> enableVideo([MediaSourceKind? kind]) async {
    await (opaque.inner.enableVideo(sourceKind: kind) as Future);
  }

  @override
  Future<void> disableVideo([MediaSourceKind? kind]) async {
    await (opaque.inner.disableVideo(sourceKind: kind) as Future);
  }

  @override
  Future<void> enableRemoteAudio([MediaSourceKind? kind]) async {
    await (opaque.inner.enableRemoteAudio(sourceKind: kind) as Future);
  }

  @override
  Future<void> disableRemoteAudio([MediaSourceKind? kind]) async {
    await (opaque.inner.disableRemoteAudio(sourceKind: kind) as Future);
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await (opaque.inner.enableRemoteVideo(sourceKind: kind) as Future);
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await (opaque.inner.disableRemoteVideo(sourceKind: kind) as Future);
  }

  @override
  void onNewConnection(void Function(ConnectionHandle) f) {
    opaque.inner.onNewConnection(
      cb: (t) {
        f(NativeConnectionHandle(frb.ConnectionHandle.fromPtr(ptr: t.address)));
      },
    );
  }

  @override
  void onClose(void Function(RoomCloseReason) f) {
    opaque.inner.onClose(
      cb: (t) {
        f(NativeRoomCloseReason(frb.RoomCloseReason.fromPtr(ptr: t.address)));
      },
    );
  }

  @override
  void onLocalTrack(void Function(LocalMediaTrack) f) {
    opaque.inner.onLocalTrack(
      cb: (t) {
        f(NativeLocalMediaTrack(frb.LocalMediaTrack.fromPtr(ptr: t.address)));
      },
    );
  }

  @override
  void onConnectionLoss(void Function(ReconnectHandle) f) {
    opaque.inner.onConnectionLoss(
      cb: (t) {
        f(NativeReconnectHandle(frb.ReconnectHandle.fromPtr(ptr: t.address)));
      },
    );
  }

  @override
  void onFailedLocalMedia(void Function(Object) f) {
    opaque.inner.onFailedLocalMedia(
      cb: (err) {
        f(err);
      },
    );
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
