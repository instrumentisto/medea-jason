import 'package:medea_jason/src/native/remote_media_track.dart';
import '../interface/connection_handle.dart';
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/frb//api/dart/api.dart' as frb;

class NativeConnectionHandle implements ConnectionHandle {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final frb.ConnectionHandle opaque;

  /// Constructs a new [ConnectionHandle] backed by a Rust struct behind the
  /// provided [frb.ConnectionHandle].
  NativeConnectionHandle(frb.ConnectionHandle connectionHandle)
      : opaque = connectionHandle {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String getRemoteMemberId() {
    return frb.connectionHandleGetRemoteMemberId(connection: opaque);
  }

  @override
  void onClose(void Function() f) {
    frb.connectionHandleOnClose(connection: opaque, f: f);
  }

  @override
  void onRemoteTrackAdded(void Function(RemoteMediaTrack) f) {
    frb.connectionHandleOnRemoteTrackAdded(
        connection: opaque,
        f: (t) {
          f(NativeRemoteMediaTrack(
              frb.remoteMediaTrackFromPtr(ptr: t.address)));
        });
  }

  @override
  void onQualityScoreUpdate(void Function(int) f) {
    frb.connectionHandleOnQualityScoreUpdate(connection: opaque, f: f);
  }

  @moveSemantics
  @override
  void free() {
    if (!opaque.isDisposed) {
      RustHandlesStorage().removeHandle(this);

      opaque.dispose();
    }
  }

  @override
  Future<void> enableRemoteAudio() async {
    await (frb.connectionHandleEnableRemoteAudio(connection: opaque) as Future);
  }

  @override
  Future<void> disableRemoteAudio() async {
    await (frb.connectionHandleDisableRemoteAudio(connection: opaque)
        as Future);
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await (frb.connectionHandleEnableRemoteVideo(
        connection: opaque, sourceKind: kind) as Future);
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await (frb.connectionHandleDisableRemoteVideo(
        connection: opaque, sourceKind: kind) as Future);
  }
}
