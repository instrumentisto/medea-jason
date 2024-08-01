import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/frb//api/dart/api.dart' as frb;

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
    return frb.remoteMediaTrackMuted(track: opaque);
  }

  @override
  MediaKind kind() {
    return frb.remoteMediaTrackKind(track: opaque);
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return frb.remoteMediaTrackMediaSourceKind(track: opaque);
  }

  @override
  TrackMediaDirection mediaDirection() {
    return frb.remoteMediaTrackMediaDirection(track: opaque);
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return frb.remoteMediaTrackGetTrack(track: opaque)
        as webrtc.MediaStreamTrack;
  }

  @override
  void onMuted(void Function() f) {
    return frb.remoteMediaTrackOnMuted(track: opaque, f: f);
  }

  @override
  void onUnmuted(void Function() f) {
    return frb.remoteMediaTrackOnUnmuted(track: opaque, f: f);
  }

  @override
  void onStopped(void Function() f) {
    return frb.remoteMediaTrackOnStopped(track: opaque, f: f);
  }

  @override
  void onMediaDirectionChanged(void Function(TrackMediaDirection) f) {
    frb.remoteMediaTrackOnMediaDirectionChanged(
        track: opaque, f: (i) => f(TrackMediaDirection.values[i]));
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
