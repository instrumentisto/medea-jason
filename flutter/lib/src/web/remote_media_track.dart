import 'package:flutter_webrtc/flutter_webrtc.dart' as webrtc;
import 'package:flutter_webrtc/src/platform/web/media_stream_track.dart';
import 'package:js/js.dart';

import '../interface/remote_media_track.dart';
import '../interface/track_kinds.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class WebRemoteMediaTrack extends RemoteMediaTrack {
  late wasm.RemoteMediaTrack obj;

  WebRemoteMediaTrack(this.obj);

  @override
  bool muted() {
    return fallibleFunction(() => obj.muted());
  }

  @override
  MediaKind kind() {
    return fallibleFunction(() => MediaKind.values[obj.kind().toInt()]);
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return fallibleFunction(
        () => MediaSourceKind.values[obj.media_source_kind().toInt()]);
  }

  @override
  TrackMediaDirection mediaDirection() {
    return fallibleFunction(
        () => TrackMediaDirection.values[obj.media_direction().toInt()]);
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return fallibleFunction(() => WebMediaStreamTrack(obj.get_track()));
  }

  @override
  void onMuted(void Function() f) {
    fallibleFunction(() => obj.on_muted(allowInterop(f)));
  }

  @override
  void onUnmuted(void Function() f) {
    fallibleFunction(() => obj.on_unmuted(allowInterop(f)));
  }

  @override
  void onStopped(void Function() f) {
    fallibleFunction(() => obj.on_stopped(allowInterop(f)));
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }

  @override
  void onMediaDirectionChanged(void Function(TrackMediaDirection) f) {
    fallibleFunction(() => obj.on_media_direction_changed(
        allowInterop((i) => f(TrackMediaDirection.values[i]))));
  }
}
