import 'package:medea_jason/src/native/remote_media_track.dart';
import '../interface/connection_handle.dart';
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';

class NativeConnectionHandle implements ConnectionHandle {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final RustOpaque<frb.ConnectionHandle> opaque;

  /// Constructs a new [ConnectionHandle] backed by a Rust struct behind the
  /// provided [frb.ConnectionHandle].
  NativeConnectionHandle(frb.ConnectionHandle connectionHandle)
      : opaque = RustOpaque(connectionHandle) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String getRemoteMemberId() {
    return api.connectionHandleGetRemoteMemberId(
        connection: opaque.innerOpaque);
  }

  @override
  void onClose(void Function() f) {
    api.connectionHandleOnClose(connection: opaque.innerOpaque, f: f);
  }

  @override
  void onRemoteTrackAdded(void Function(RemoteMediaTrack) f) {
    api.connectionHandleOnRemoteTrackAdded(
        connection: opaque.innerOpaque,
        f: (t) {
          f(NativeRemoteMediaTrack(
              api.remoteMediaTrackFromPtr(ptr: t.address)));
        });
  }

  @override
  void onQualityScoreUpdate(void Function(int) f) {
    api.connectionHandleOnQualityScoreUpdate(
        connection: opaque.innerOpaque, f: f);
  }

  @moveSemantics
  @override
  void free() {
    if (!opaque.isStale()) {
      RustHandlesStorage().removeHandle(this);

      opaque.dispose();
    }
  }

  @override
  Future<void> enableRemoteAudio() async {
    await (api.connectionHandleEnableRemoteAudio(connection: opaque.innerOpaque)
        as Future);
  }

  @override
  Future<void> disableRemoteAudio() async {
    await (api.connectionHandleDisableRemoteAudio(
        connection: opaque.innerOpaque) as Future);
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await (api.connectionHandleEnableRemoteVideo(
        connection: opaque.innerOpaque, sourceKind: kind) as Future);
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await (api.connectionHandleDisableRemoteVideo(
        connection: opaque.innerOpaque, sourceKind: kind) as Future);
  }
}
