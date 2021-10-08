import 'package:flutter_webrtc/src/web/media_stream_track_impl.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';

import 'exceptions.dart';
import '../interface/local_media_track.dart';
import '../interface/track_kinds.dart';
import '../web/jason_wasm.dart' as wasm;
import '../util/move_semantic.dart';

class WebLocalMediaTrack extends LocalMediaTrack {
  late wasm.LocalMediaTrack obj;

  WebLocalMediaTrack(this.obj);

  @override
  MediaKind kind() {
    return failableFunction(() => MediaKind.values[obj.kind().toInt()]);
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return failableFunction(
        () => MediaSourceKind.values[obj.media_source_kind().toInt()]);
  }

  @override
  MediaStreamTrack getTrack() {
    return failableFunction(() => MediaStreamTrackWeb(obj.get_track()));
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
