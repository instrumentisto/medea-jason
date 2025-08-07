import 'dart:js_interop';

import '../interface/connection_handle.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/media_track.dart';
import '../interface/reconnect_handle.dart';
import '../interface/room_close_reason.dart';
import '../interface/room_handle.dart';
import '../util/move_semantic.dart';
import 'connection_handle.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;
import 'local_media_track.dart';
import 'media_stream_settings.dart';
import 'reconnect_handle.dart';
import 'room_close_reason.dart';

class WebRoomHandle implements RoomHandle {
  late wasm.RoomHandle obj;

  WebRoomHandle(this.obj);

  @override
  Future<void> join(String token) async {
    await fallibleFuture(obj.join(token).toDart);
  }

  @override
  Future<void> setLocalMediaSettings(
    base_settings.MediaStreamSettings settings,
    bool stopFirst,
    bool rollbackOnFail,
  ) async {
    await fallibleFuture(
      obj
          .set_local_media_settings(
            (settings as MediaStreamSettings).obj,
            stopFirst,
            rollbackOnFail,
          )
          .toDart,
    );
  }

  @override
  Future<void> muteAudio([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.mute_audio(kind?.index).toDart);
  }

  @override
  Future<void> unmuteAudio([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.unmute_audio(kind?.index).toDart);
  }

  @override
  Future<void> enableAudio([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.enable_audio(kind?.index).toDart);
  }

  @override
  Future<void> disableAudio([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.disable_audio(kind?.index).toDart);
  }

  @override
  Future<void> muteVideo([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.mute_video(kind?.index).toDart);
  }

  @override
  Future<void> unmuteVideo([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.unmute_video(kind?.index).toDart);
  }

  @override
  Future<void> enableVideo([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.enable_video(kind?.index).toDart);
  }

  @override
  Future<void> disableVideo([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.disable_video(kind?.index).toDart);
  }

  @override
  Future<void> enableRemoteAudio([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.enable_remote_audio(kind?.index).toDart);
  }

  @override
  Future<void> disableRemoteAudio([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.disable_remote_audio(kind?.index).toDart);
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.enable_remote_video(kind?.index).toDart);
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.disable_remote_video(kind?.index).toDart);
  }

  @override
  void onNewConnection(void Function(ConnectionHandle) f) {
    void fn(JSAny? handle) =>
        f(WebConnectionHandle(handle as wasm.ConnectionHandle));

    fallibleFunction(() => obj.on_new_connection(fn.toJS));
  }

  @override
  void onClose(void Function(RoomCloseReason) f) {
    void fn(JSAny? reason) =>
        f(WebRoomCloseReason(reason as wasm.RoomCloseReason));
    fallibleFunction(() => obj.on_close(fn.toJS));
  }

  @override
  void onLocalTrack(void Function(LocalMediaTrack) f) {
    void fn(JSAny? track) =>
        f(WebLocalMediaTrack(track as wasm.LocalMediaTrack));
    fallibleFunction(() => obj.on_local_track(fn.toJS));
  }

  @override
  void onConnectionLoss(void Function(ReconnectHandle) f) {
    void fn(JSAny? handle) =>
        f(WebReconnectHandle(handle as wasm.ReconnectHandle));
    fallibleFunction(() => obj.on_connection_loss(fn.toJS));
  }

  @override
  void onFailedLocalMedia(void Function(Object) f) {
    void fn(JSAny? e) => f(convertException(e));
    fallibleFunction(() => obj.on_failed_local_media(fn.toJS));
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
