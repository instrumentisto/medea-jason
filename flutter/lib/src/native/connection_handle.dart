import 'package:medea_jason/src/native/remote_media_track.dart';
import '../interface/connection_handle.dart';
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/frb/frb.dart' as frb;

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
    return opaque.getRemoteMemberId();
  }

  @override
  void onClose(void Function() f) {
    opaque.onClose(f: f);
  }

  @override
  void onRemoteTrackAdded(void Function(RemoteMediaTrack) f) {
    opaque.onRemoteTrackAdded(f: (t) {
      f(NativeRemoteMediaTrack(frb.RemoteMediaTrack.fromRaw(ptr: t.address)));
    });
  }

  @override
  void onQualityScoreUpdate(void Function(int) f) {
    opaque.onQualityScoreUpdate(f: f);
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
}
