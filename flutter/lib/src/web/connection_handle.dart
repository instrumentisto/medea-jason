import 'dart:js';

import 'package:js/js.dart';

import '../interface/connection_handle.dart';
import '../interface/remote_media_track.dart';
import '../web/jason_wasm.dart' as wasm;
import '../util/move_semantic.dart';
import '../web/remote_media_track.dart';

class WebConnectionHandle extends ConnectionHandle {
  late wasm.ConnectionHandle obj;

  WebConnectionHandle(this.obj);

  @override
  String getRemoteMemberId() {
    return obj.get_remote_member_id();
  }

  @override
  void onClose(void Function() f) {
    obj.on_close(allowInterop(f));
  }

  @override
  void onRemoteTrackAdded(void Function(RemoteMediaTrack) f) {
    obj.on_remote_track_added(allowInterop((track) {
      f(WebRemoteMediaTrack(track));
    }));
  }

  @override
  void onQualityScoreUpdate(void Function(int) f) {
    obj.on_quality_score_update(allowInterop(f));
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
