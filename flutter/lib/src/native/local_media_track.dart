import 'dart:ffi';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as api;
import 'jason.dart';

class NativeLocalMediaTrack extends LocalMediaTrack {
  /// [Pointer] to the Rust struct backing this object.
  late api.LocalMediaTrack opaque;

  /// Constructs a new [LocalMediaTrack] backed by the Rust struct behind the
  /// provided [Pointer].

  NativeLocalMediaTrack.opaque(this.opaque) {
  }

  @override
  MediaKind kind() {
    return MediaKind.values[impl_api.localMediaTrackKind(track: opaque)];
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return MediaSourceKind
        .values[impl_api.localMediaTrackMediaSourceKind(track: opaque)];
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return rust2dart2(impl_api.localMediaTrackGetTrack(track: opaque))
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
