import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:flutter_webrtc/src/web/media_stream_track_impl.dart';
import 'package:js/js.dart';

import '../interface/remote_media_track.dart';
import '../interface/track_kinds.dart';
import '../web/jason_wasm.dart' as wasm;
import '../util/move_semantic.dart';

class WebRemoteMediaTrack extends RemoteMediaTrack {
  late wasm.RemoteMediaTrack obj;

  WebRemoteMediaTrack(this.obj);

  @override
  bool enabled() {
    return obj.enabled();
  }

  @override
  bool muted() {
    return obj.muted();
  }

  @override
  MediaKind kind() {
    return MediaKind.values[obj.kind().toInt()];
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return MediaSourceKind.values[obj.media_source_kind().toInt()];
  }

  @override
  MediaStreamTrack getTrack() {
    return MediaStreamTrackWeb(obj.get_track());
  }

  @override
  void onEnabled(void Function() f) {
    obj.on_enabled(allowInterop(f));
  }

  @override
  void onDisabled(void Function() f) {
    obj.on_disabled(allowInterop(f));
  }

  @override
  void onMuted(void Function() f) {
    obj.on_muted(allowInterop(f));
  }

  @override
  void onUnmuted(void Function() f) {
    obj.on_unmuted(allowInterop(f));
  }

  @override
  void onStopped(void Function() f) {
    obj.on_stopped(allowInterop(f));
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
