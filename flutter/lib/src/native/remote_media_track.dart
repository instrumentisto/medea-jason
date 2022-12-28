import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';

class NativeRemoteMediaTrack extends RemoteMediaTrack {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late RustOpaque<frb.RemoteMediaTrack> opaque;

  /// Constructs a new [RemoteMediaTrack] backed by the Rust struct behind the
  /// provided [frb.RemoteMediaTrack].
  NativeRemoteMediaTrack(frb.RemoteMediaTrack remoteMediaTrack)
      : opaque = RustOpaque(remoteMediaTrack) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  bool muted() {
    return api.remoteMediaTrackMuted(track: opaque.innerOpaque);
  }

  @override
  MediaKind kind() {
    return api.remoteMediaTrackKind(track: opaque.innerOpaque);
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return api.remoteMediaTrackMediaSourceKind(track: opaque.innerOpaque);
  }

  @override
  TrackMediaDirection mediaDirection() {
    return api.remoteMediaTrackMediaDirection(track: opaque.innerOpaque);
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return api.remoteMediaTrackGetTrack(track: opaque.innerOpaque)
        as webrtc.MediaStreamTrack;
  }

  @override
  void onMuted(void Function() f) {
    return api.remoteMediaTrackOnMuted(track: opaque.innerOpaque, f: f);
  }

  @override
  void onUnmuted(void Function() f) {
    return api.remoteMediaTrackOnUnmuted(track: opaque.innerOpaque, f: f);
  }

  @override
  void onStopped(void Function() f) {
    return api.remoteMediaTrackOnStopped(track: opaque.innerOpaque, f: f);
  }

  @override
  void onMediaDirectionChanged(void Function(TrackMediaDirection) f) {
    api.remoteMediaTrackOnMediaDirectionChanged(
        track: opaque.innerOpaque, f: (i) => f(TrackMediaDirection.values[i]));
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
