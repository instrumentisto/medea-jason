import 'dart:js';

import 'package:js/js.dart';

import '../interface/connection_handle.dart';
import '../interface/remote_media_track.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;
import 'remote_media_track.dart';

class WebConnectionHandle extends ConnectionHandle {
  late wasm.ConnectionHandle obj;

  WebConnectionHandle(this.obj);

  @override
  String getRemoteMemberId() {
    return fallibleFunction(() => obj.get_remote_member_id());
  }

  @override
  void onClose(void Function() f) {
    fallibleFunction(() => obj.on_close(allowInterop(f)));
  }

  @override
  void onRemoteTrackAdded(void Function(RemoteMediaTrack) f) {
    fallibleFunction(() => obj.on_remote_track_added(allowInterop((track) {
          f(WebRemoteMediaTrack(track));
        })));
  }

  @override
  void onQualityScoreUpdate(void Function(int) f) {
    fallibleFunction(() => obj.on_quality_score_update(allowInterop(f)));
  }

  @override
  Future<void> enableRemoteAudio() async {
    await fallibleFuture(obj.enable_remote_audio());
  }

  @override
  Future<void> disableRemoteAudio() async {
    await fallibleFuture(obj.disable_remote_audio());
  }

  @override
  Future<void> enableRemoteVideo() async {
    await fallibleFuture(obj.enable_remote_video());
  }

  @override
  Future<void> disableRemoteVideo() async {
    await fallibleFuture(obj.disable_remote_video());
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
