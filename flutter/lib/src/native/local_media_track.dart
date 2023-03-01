import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';

class NativeLocalMediaTrack implements LocalMediaTrack {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final RustOpaque<frb.LocalMediaTrack> opaque;

  /// Constructs a new [LocalMediaTrack] backed by the Rust struct behind the
  /// provided [frb.LocalMediaTrack].
  NativeLocalMediaTrack(frb.LocalMediaTrack localMediaTrack)
      : opaque = RustOpaque(localMediaTrack) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  MediaKind kind() {
    return api.localMediaTrackKind(track: opaque.innerOpaque);
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return api.localMediaTrackMediaSourceKind(track: opaque.innerOpaque);
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return api.localMediaTrackGetTrack(track: opaque.innerOpaque)
        as webrtc.MediaStreamTrack;
  }

  @moveSemantics
  @override
  Future<void> free() async {
    if (!opaque.isStale()) {
      RustHandlesStorage().removeHandle(this);
      await (api.localMediaTrackFree(track: opaque.moveOpaque) as Future);
    }
  }

  @override
  void onEnded(OnEndedCallback f) {
    api.localMediaTrackOnEnded(track: opaque.innerOpaque, f: f);
  }

  @override
  Future<webrtc.MediaStreamTrackState> state() async {
    var index =
        await (api.localMediaTrackState(track: opaque.innerOpaque) as Future);
    return webrtc.MediaStreamTrackState.values[index];
  }
}
