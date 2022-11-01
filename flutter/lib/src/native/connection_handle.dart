import 'dart:ffi';

import 'package:medea_jason/src/native/remote_media_track.dart';

import '../interface/connection_handle.dart';
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as api;
import 'jason.dart';

class NativeConnectionHandle extends ConnectionHandle {
  /// [Pointer] to the Rust struct backing this object.
  late api.ConnectionHandle opaque;

  /// Constructs a new [ConnectionHandle] backed by a Rust struct behind the
  /// provided [Pointer].

  NativeConnectionHandle.opaque(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String getRemoteMemberId() {
    return impl_api.connectionHandleGetRemoteMemberId(connection: opaque);
  }

  @override
  void onClose(void Function() f) {
    impl_api.connectionHandleOnClose(connection: opaque, f: handle2rust(f));
  }

  @override
  void onRemoteTrackAdded(void Function(RemoteMediaTrack) f) {
    var _f = handle2rust((t) {
      f(NativeRemoteMediaTrack.opaque(impl_api.remoteMediaTrackFromPtr(ptr: t.address)));
    });
    impl_api.connectionHandleOnRemoteTrackAdded(
        connection: opaque, f: _f);
  }

  @override
  void onQualityScoreUpdate(void Function(int) f) {
    impl_api.connectionHandleOnQualityScoreUpdate(
        connection: opaque, f: handle2rust(f));
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
    await rust2dart(
        impl_api.connectionHandleEnableRemoteAudio(connection: opaque));
  }

  @override
  Future<void> disableRemoteAudio() async {
    await rust2dart(
        impl_api.connectionHandleDisableRemoteAudio(connection: opaque));
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await rust2dart(impl_api.connectionHandleEnableRemoteVideo(
        connection: opaque, sourceKind: kind?.index));
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await rust2dart(impl_api.connectionHandleDisableRemoteVideo(
        connection: opaque, sourceKind: kind?.index));
  }
}
