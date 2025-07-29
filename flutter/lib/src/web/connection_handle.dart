// ignore_for_file: avoid_web_libraries_in_flutter

import 'dart:js_interop';

import 'package:js_interop_utils/js_interop_utils.dart';

import '../interface/connection_handle.dart';
import '../interface/enums.dart'
    show MemberConnectionState, MemberConnectionStateKind, PeerConnectionState;
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
  MemberConnectionState? getState() {
    return freezeState(obj.get_state());
  }

  @override
  void onStateChange(void Function(MemberConnectionState) f) {
    void fn(JSAny state) =>
        f(freezeState(state as wasm.MemberConnectionState)!);
    fallibleFunction(() => obj.on_state_change(fn.toJS));
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

  MemberConnectionState? freezeState(wasm.MemberConnectionState? state) {
    if (state == null) {
      return null;
    }

    if (MemberConnectionStateKind.values[state.kind().toInt()] ==
        MemberConnectionStateKind.p2p) {
      return MemberConnectionState.p2p(
        PeerConnectionState.values[(state.value() as JSNumber).toDartInt],
      );
    }

    // TODO: implement for SFU.
    throw UnimplementedError(
      'Only MemberConnectionStateKind.p2p is supported.',
    );
  }
}
