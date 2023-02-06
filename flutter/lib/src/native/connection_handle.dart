import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

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
    try {
      return api.connectionHandleGetRemoteMemberId(
          connection: opaque.innerOpaque);
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
    }
  }

  @override
  void onClose(void Function() f) {
    try {
      api.connectionHandleOnClose(connection: opaque.innerOpaque, f: f);
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
    }
  }

  @override
  void onRemoteTrackAdded(void Function(RemoteMediaTrack) f) {
    try {
      api.connectionHandleOnRemoteTrackAdded(
          connection: opaque.innerOpaque,
          f: (t) {
            f(NativeRemoteMediaTrack(
                api.remoteMediaTrackFromPtr(ptr: t.address)));
          });
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
    }
  }

  @override
  void onQualityScoreUpdate(void Function(int) f) {
    try {
      api.connectionHandleOnQualityScoreUpdate(
          connection: opaque.innerOpaque, f: f);
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
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
    try {
      await (api.connectionHandleEnableRemoteAudio(
          connection: opaque.innerOpaque) as Future);
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
    }
  }

  @override
  Future<void> disableRemoteAudio() async {
    try {
      await (api.connectionHandleDisableRemoteAudio(
          connection: opaque.innerOpaque) as Future);
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
    }
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    try {
      await (api.connectionHandleEnableRemoteVideo(
          connection: opaque.innerOpaque) as Future);
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
    }
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    try {
      await (api.connectionHandleDisableRemoteVideo(
          connection: opaque.innerOpaque) as Future);
    } on FfiException catch (anyhow) {
      throw anyhow.parse();
    }
  }
}
