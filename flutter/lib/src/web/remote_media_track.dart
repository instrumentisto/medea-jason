// ignore_for_file: implementation_imports

import 'dart:js_interop';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;
import 'package:medea_flutter_webrtc/src/platform/web/media_stream_track.dart';

import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class WebRemoteMediaTrack implements RemoteMediaTrack {
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
      () => MediaSourceKind.values[obj.media_source_kind().toInt()],
    );
  }

  @override
  TrackMediaDirection mediaDirection() {
    return fallibleFunction(
      () => TrackMediaDirection.values[obj.media_direction().toInt()],
    );
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return fallibleFunction(() => WebMediaStreamTrack(obj.get_track()));
  }

  @override
  void onMuted(void Function() f) {
    fallibleFunction(() => obj.on_muted(f.toJS));
  }

  @override
  void onUnmuted(void Function() f) {
    fallibleFunction(() => obj.on_unmuted(f.toJS));
  }

  @override
  void onStopped(void Function() f) {
    fallibleFunction(() => obj.on_stopped(f.toJS));
  }

  @moveSemantics
  @override
  Future<void> free() async {
    obj.free();
  }

  @override
  void onMediaDirectionChanged(void Function(TrackMediaDirection) f) {
    void fn(JSAny? i) =>
        f(TrackMediaDirection.values[(i as JSNumber).toDartInt]);

    fallibleFunction(() => obj.on_media_direction_changed(fn.toJS));
  }
}
