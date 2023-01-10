import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;
import 'package:medea_flutter_webrtc/src/platform/web/media_stream_track.dart';

import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class WebLocalMediaTrack implements LocalMediaTrack {
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
  Future<void> free() async {
    obj.free();
  }
}
