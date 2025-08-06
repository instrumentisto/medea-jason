// ignore_for_file: avoid_web_libraries_in_flutter

import 'dart:js_interop';

import '../interface/connection_handle.dart';
import '../interface/enums.dart';
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;
import 'remote_media_track.dart';

import '../interface/member_connection_state.dart'
    show
        MemberConnectionState,
        MemberConnectionStateP2P,
        MemberConnectionStateKind;

class WebConnectionHandle implements ConnectionHandle {
  late wasm.ConnectionHandle obj;

  WebConnectionHandle(this.obj);

  @override
  String getRemoteMemberId() {
    return fallibleFunction(() => obj.get_remote_member_id());
  }

  @override
  MemberConnectionState? getState() {
    return convertState(obj.get_state());
  }

  @override
  void onStateChange(void Function(MemberConnectionState) f) {
    void fn(JSAny state) =>
        f(convertState(state as wasm.MemberConnectionState)!);
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

  MemberConnectionState? convertState(wasm.MemberConnectionState? state) {
    if (state == null) {
      return null;
    }

    if (MemberConnectionStateKind.values[state.kind().toInt()] ==
        MemberConnectionStateKind.p2p) {
      var peerState =
          PeerConnectionState.values[(state.value() as JSNumber).toDartInt];

      state.free();

      return MemberConnectionStateP2P(peerState);
    }

    // TODO: implement for SFU when Jason will support it.
    // See instrumentisto/medea-jason#211 for the details:
    // https://github.com/instrumentisto/medea-jason/issues/211
    throw UnimplementedError(
      'Only MemberConnectionStateKind.p2p is supported.',
    );
  }
}
