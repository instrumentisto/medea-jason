import 'package:js/js.dart';

import '../interface/connection_handle.dart';
import '../interface/local_media_track.dart';
import '../interface/media_stream_settings.dart' as base_settings;
import '../interface/reconnect_handle.dart';
import '../interface/room_close_reason.dart';
import '../interface/room_handle.dart';
import '../interface/track_kinds.dart';
import '../util/move_semantic.dart';
import 'connection_handle.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;
import 'local_media_track.dart';
import 'media_stream_settings.dart';
import 'reconnect_handle.dart';
import 'room_close_reason.dart';

class WebRoomHandle extends RoomHandle {
  late wasm.RoomHandle obj;

  WebRoomHandle(this.obj);

  @override
  Future<void> join(String token) async {
    await failableFuture(obj.join(token));
  }

  @override
  Future<void> setLocalMediaSettings(base_settings.MediaStreamSettings settings,
      bool stopFirst, bool rollbackOnFail) async {
    await failableFuture(obj.set_local_media_settings(
        (settings as MediaStreamSettings).obj, stopFirst, rollbackOnFail));
  }

  @override
  Future<void> muteAudio() async {
    await failableFuture(obj.mute_audio());
  }

  @override
  Future<void> unmuteAudio() async {
    await failableFuture(obj.unmute_audio());
  }

  @override
  Future<void> enableAudio() async {
    await failableFuture(obj.enable_audio());
  }

  @override
  Future<void> disableAudio() async {
    await failableFuture(obj.disable_audio());
  }

  @override
  Future<void> muteVideo([MediaSourceKind? kind]) async {
    await failableFuture(obj.mute_video(kind?.index));
  }

  @override
  Future<void> unmuteVideo([MediaSourceKind? kind]) async {
    await failableFuture(obj.unmute_video(kind?.index));
  }

  @override
  Future<void> enableVideo([MediaSourceKind? kind]) async {
    await failableFuture(obj.enable_video(kind?.index));
  }

  @override
  Future<void> disableVideo([MediaSourceKind? kind]) async {
    await failableFuture(obj.disable_video(kind?.index));
  }

  @override
  Future<void> enableRemoteAudio() async {
    await failableFuture(obj.enable_remote_audio());
  }

  @override
  Future<void> disableRemoteAudio() async {
    await failableFuture(obj.disable_remote_audio());
  }

  @override
  Future<void> enableRemoteVideo() async {
    await failableFuture(obj.enable_remote_video());
  }

  @override
  Future<void> disableRemoteVideo() async {
    await failableFuture(obj.disable_remote_video());
  }

  @override
  void onNewConnection(void Function(ConnectionHandle) f) {
    failableFunction(() => obj.on_new_connection(allowInterop((handle) {
          f(WebConnectionHandle(handle));
        })));
  }

  @override
  void onClose(void Function(RoomCloseReason) f) {
    failableFunction(() => obj.on_close(allowInterop((reason) {
          f(WebRoomCloseReason(reason));
        })));
  }

  @override
  void onLocalTrack(void Function(LocalMediaTrack) f) {
    failableFunction(() => obj.on_local_track(allowInterop((track) {
          f(WebLocalMediaTrack(track));
        })));
  }

  @override
  void onConnectionLoss(void Function(ReconnectHandle) f) {
    failableFunction(() => obj.on_connection_loss(allowInterop((handle) {
          f(WebReconnectHandle(handle));
        })));
  }

  @override
  void onFailedLocalMedia(void Function(Object) f) {
    failableFunction(() => obj.on_failed_local_media(allowInterop(f)));
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
