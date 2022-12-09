import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'package:medea_jason/src/native/remote_media_track.dart';
import '../interface/connection_handle.dart';
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as frb;
import 'jason.dart';

class NativeConnectionHandle extends ConnectionHandle {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late frb.ConnectionHandle opaque;

  /// Constructs a new [ConnectionHandle] backed by a Rust struct behind the
  /// provided [frb.ConnectionHandle].
  NativeConnectionHandle(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String getRemoteMemberId() {
    try {
      return api.connectionHandleGetRemoteMemberId(connection: opaque);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow.message);
    }
  }

  @override
  void onClose(void Function() f) {
    try {
      api.connectionHandleOnClose(connection: opaque, f: f);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow.message);
    }
  }

  @override
  void onRemoteTrackAdded(void Function(RemoteMediaTrack) f) {
    try {
      api.connectionHandleOnRemoteTrackAdded(
          connection: opaque,
          f: (t) {
            f(NativeRemoteMediaTrack(
                api.remoteMediaTrackFromPtr(ptr: t.address)));
          });
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow.message);
    }
  }

  @override
  void onQualityScoreUpdate(void Function(int) f) {
    try {
      api.connectionHandleOnQualityScoreUpdate(connection: opaque, f: f);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow.message);
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

  @override
  Future<void> enableRemoteAudio() async {
    await (api.connectionHandleEnableRemoteAudio(connection: opaque)
        as Future<void>);
  }

  @override
  Future<void> disableRemoteAudio() async {
    await (api.connectionHandleDisableRemoteAudio(connection: opaque)
        as Future<void>);
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await (api.connectionHandleEnableRemoteVideo(connection: opaque)
        as Future<void>);
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await (api.connectionHandleDisableRemoteVideo(connection: opaque)
        as Future<void>);
  }
}
