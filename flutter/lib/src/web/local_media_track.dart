import 'package:flutter_webrtc/flutter_webrtc.dart' as webrtc;
import 'package:flutter_webrtc/src/platform/web/media_stream_track.dart';

import '../interface/local_media_track.dart';
import '../interface/track_kinds.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class WebLocalMediaTrack extends LocalMediaTrack {
  late wasm.LocalMediaTrack obj;

  WebLocalMediaTrack(this.obj);

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
  webrtc.MediaStreamTrack getTrack() {
    return fallibleFunction(() => WebMediaStreamTrack(obj.get_track()));
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
