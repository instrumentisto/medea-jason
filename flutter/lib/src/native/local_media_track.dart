import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';

class NativeLocalMediaTrack extends LocalMediaTrack {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late RustOpaque<frb.LocalMediaTrack> opaque;

  /// Constructs a new [LocalMediaTrack] backed by the Rust struct behind the
  /// provided [frb.LocalMediaTrack].
  NativeLocalMediaTrack(frb.LocalMediaTrack localMediaTrack)
      : opaque = RustOpaque(localMediaTrack) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  MediaKind kind() {
    return MediaKind.values[api.localMediaTrackKind(track: opaque.innerOpaque)];
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return MediaSourceKind
        .values[api.localMediaTrackMediaSourceKind(track: opaque.innerOpaque)];
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return api.localMediaTrackGetTrack(track: opaque.innerOpaque)
        as webrtc.MediaStreamTrack;
  }

  @moveSemantics
  @override
  void free() {
    if (!opaque.isStale()) {
      RustHandlesStorage().removeHandle(this);

      opaque.dispose();
    }
  }
}
