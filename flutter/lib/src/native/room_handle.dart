import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

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
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';
import 'local_media_track.dart';
import 'media_stream_settings.dart';
import 'reconnect_handle.dart';
import 'room_close_reason.dart';

class NativeRoomHandle extends RoomHandle {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late RustOpaque<frb.RoomHandle> opaque;

  /// Constructs a new [RoomHandle] backed by the Rust struct behind the
  /// provided [frb.RoomHandle].
  NativeRoomHandle(frb.RoomHandle roomHandle)
      : opaque = RustOpaque(roomHandle) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  Future<void> join(String token) async {
    try {
      await (api.roomHandleJoin(roomHandle: opaque.innerOpaque, token: token)
          as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> setLocalMediaSettings(base_settings.MediaStreamSettings settings,
      bool stopFirst, bool rollbackOnFail) async {
    try {
      await (api.roomHandleSetLocalMediaSettings(
          roomHandle: opaque.innerOpaque,
          settings: (settings as MediaStreamSettings).opaque.innerOpaque,
          stopFirst: stopFirst,
          rollbackOnFail: rollbackOnFail) as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> muteAudio() async {
    try {
      await (api.roomHandleMuteAudio(roomHandle: opaque.innerOpaque) as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> unmuteAudio() async {
    try {
      await (api.roomHandleUnmuteAudio(roomHandle: opaque.innerOpaque)
          as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> enableAudio() async {
    try {
      await (api.roomHandleEnableAudio(roomHandle: opaque.innerOpaque)
          as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> disableAudio() async {
    try {
      await (api.roomHandleDisableAudio(roomHandle: opaque.innerOpaque)
          as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> muteVideo([MediaSourceKind? kind]) async {
    try {
      await (api.roomHandleMuteVideo(
          roomHandle: opaque.innerOpaque, sourceKind: kind?.index) as Future);
    } on FfiException catch (anyhow) {
      objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> unmuteVideo([MediaSourceKind? kind]) async {
    try {
      await (api.roomHandleUnmuteVideo(
          roomHandle: opaque.innerOpaque, sourceKind: kind?.index) as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> enableVideo([MediaSourceKind? kind]) async {
    try {
      await (api.roomHandleEnableVideo(
          roomHandle: opaque.innerOpaque, sourceKind: kind?.index) as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> disableVideo([MediaSourceKind? kind]) async {
    try {
      await (api.roomHandleDisableVideo(
          roomHandle: opaque.innerOpaque, sourceKind: kind?.index) as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> enableRemoteAudio() async {
    try {
      await (api.roomHandleEnableRemoteAudio(roomHandle: opaque.innerOpaque)
          as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> disableRemoteAudio() async {
    try {
      await (api.roomHandleDisableRemoteAudio(roomHandle: opaque.innerOpaque)
          as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    try {
      await (api.roomHandleEnableRemoteVideo(
          roomHandle: opaque.innerOpaque, sourceKind: kind?.index) as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    try {
      await (api.roomHandleDisableRemoteVideo(
          roomHandle: opaque.innerOpaque, sourceKind: kind?.index) as Future);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void onNewConnection(void Function(ConnectionHandle) f) {
    try {
      api.roomHandleOnNewConnection(
          roomHandle: opaque.innerOpaque,
          cb: (t) {
            f(NativeConnectionHandle(
                api.connectionHandleFromPtr(ptr: t.address)));
          });
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void onClose(void Function(RoomCloseReason) f) {
    try {
      api.roomHandleOnClose(
          roomHandle: opaque.innerOpaque,
          cb: (t) {
            f(NativeRoomCloseReason(
                api.roomCloseReasonFromPtr(ptr: t.address)));
          });
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void onLocalTrack(void Function(LocalMediaTrack) f) {
    try {
      api.roomHandleOnLocalTrack(
          roomHandle: opaque.innerOpaque,
          cb: (t) {
            f(NativeLocalMediaTrack(
                api.localMediaTrackFromPtr(ptr: t.address)));
          });
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void onConnectionLoss(void Function(ReconnectHandle) f) {
    try {
      api.roomHandleOnConnectionLoss(
          roomHandle: opaque.innerOpaque,
          cb: (t) {
            f(NativeReconnectHandle(
                api.reconnectHandleFromPtr(ptr: t.address)));
          });
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void onFailedLocalMedia(void Function(Object) f) {
    try {
      api.roomHandleOnFailedLocalMedia(
          roomHandle: opaque.innerOpaque,
          cb: (err) {
            f(err);
          });
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
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
}
