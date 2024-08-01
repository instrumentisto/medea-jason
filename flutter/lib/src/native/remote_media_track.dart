import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/frb//api/dart/api.dart' as frb;

class NativeRemoteMediaTrack implements RemoteMediaTrack {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final RustOpaque<frb.RemoteMediaTrack> opaque;

  /// Constructs a new [RemoteMediaTrack] backed by the Rust struct behind the
  /// provided [frb.RemoteMediaTrack].
  NativeRemoteMediaTrack(frb.RemoteMediaTrack remoteMediaTrack)
      : opaque = RustOpaque(remoteMediaTrack) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  bool muted() {
    return frb.remoteMediaTrackMuted(track: opaque.innerOpaque);
  }

  @override
  MediaKind kind() {
    return frb.remoteMediaTrackKind(track: opaque.innerOpaque);
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return frb.remoteMediaTrackMediaSourceKind(track: opaque.innerOpaque);
  }

  @override
  TrackMediaDirection mediaDirection() {
    return frb.remoteMediaTrackMediaDirection(track: opaque.innerOpaque);
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return frb.remoteMediaTrackGetTrack(track: opaque.innerOpaque)
        as webrtc.MediaStreamTrack;
  }

  @override
  void onMuted(void Function() f) {
    return frb.remoteMediaTrackOnMuted(track: opaque.innerOpaque, f: f);
  }

  @override
  void onUnmuted(void Function() f) {
    return frb.remoteMediaTrackOnUnmuted(track: opaque.innerOpaque, f: f);
  }

  @override
  void onStopped(void Function() f) {
    return frb.remoteMediaTrackOnStopped(track: opaque.innerOpaque, f: f);
  }

  @override
  void onMediaDirectionChanged(void Function(TrackMediaDirection) f) {
    frb.remoteMediaTrackOnMediaDirectionChanged(
        track: opaque.innerOpaque, f: (i) => f(TrackMediaDirection.values[i]));
  }

  @moveSemantics
  @override
  Future<void> free() async {
    if (!opaque.isStale()) {
      RustHandlesStorage().removeHandle(this);

      opaque.dispose();
    }
  }
}
