import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/frb/frb.dart' as frb;

class NativeRemoteMediaTrack implements RemoteMediaTrack {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final frb.RemoteMediaTrack opaque;

  /// Constructs a new [RemoteMediaTrack] backed by the Rust struct behind the
  /// provided [frb.RemoteMediaTrack].
  NativeRemoteMediaTrack(frb.RemoteMediaTrack remoteMediaTrack)
      : opaque = remoteMediaTrack {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  bool muted() {
    return opaque.muted();
  }

  @override
  MediaKind kind() {
    return opaque.kind();
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return opaque.mediaSourceKind();
  }

  @override
  TrackMediaDirection mediaDirection() {
    return opaque.mediaDirection();
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return opaque.getTrack() as webrtc.MediaStreamTrack;
  }

  @override
  void onMuted(void Function() f) {
    return opaque.onMuted(f: f);
  }

  @override
  void onUnmuted(void Function() f) {
    return opaque.onUnmuted(f: f);
  }

  @override
  void onStopped(void Function() f) {
    return opaque.onStopped(f: f);
  }

  @override
  void onMediaDirectionChanged(void Function(TrackMediaDirection) f) {
    opaque.onMediaDirectionChanged(f: (i) => f(TrackMediaDirection.values[i]));
  }

  @moveSemantics
  @override
  Future<void> free() async {
    if (!opaque.isDisposed) {
      RustHandlesStorage().removeHandle(this);

      opaque.dispose();
    }
  }
}
