// ignore_for_file: avoid_web_libraries_in_flutter

import 'dart:js_interop';

import '../interface/connection_handle.dart';
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;
import 'remote_media_track.dart';

class WebConnectionHandle implements ConnectionHandle {
  late wasm.ConnectionHandle obj;

  WebConnectionHandle(this.obj);

  @override
  String getRemoteMemberId() {
    return fallibleFunction(() => obj.get_remote_member_id());
  }

  @override
  void onClose(void Function() f) {
    fallibleFunction(() => obj.on_close(f.toJS));
  }

  @override
  void onRemoteTrackAdded(void Function(RemoteMediaTrack) f) {
    void fn(JSAny? track) =>
        f(WebRemoteMediaTrack(track as wasm.RemoteMediaTrack));
    fallibleFunction(() => obj.on_remote_track_added(fn.toJS));
  }

  @override
  void onQualityScoreUpdate(void Function(int) f) {
    fallibleFunction(() => obj.on_quality_score_update(f.toJS));
  }

  @override
  Future<void> enableRemoteAudio() async {
    await fallibleFuture(obj.enable_remote_audio().toDart);
  }

  @override
  Future<void> disableRemoteAudio() async {
    await fallibleFuture(obj.disable_remote_audio().toDart);
  }

  @override
  Future<void> enableRemoteVideo([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.enable_remote_video(kind?.index).toDart);
  }

  @override
  Future<void> disableRemoteVideo([MediaSourceKind? kind]) async {
    await fallibleFuture(obj.disable_remote_video(kind?.index).toDart);
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
