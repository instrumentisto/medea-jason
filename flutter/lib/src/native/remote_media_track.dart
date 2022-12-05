import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as frb;
import 'jason.dart';

class NativeRemoteMediaTrack extends RemoteMediaTrack {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late frb.RemoteMediaTrack opaque;

  /// Constructs a new [RemoteMediaTrack] backed by the Rust struct behind the
  /// provided [frb.RemoteMediaTrack].
  NativeRemoteMediaTrack(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  bool muted() {
    return api.remoteMediaTrackMuted(track: opaque);
  }

  @override
  MediaKind kind() {
    return MediaKind.values[api.remoteMediaTrackKind(track: opaque)];
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return MediaSourceKind
        .values[api.remoteMediaTrackMediaSourceKind(track: opaque)];
  }

  @override
  TrackMediaDirection mediaDirection() {
    return TrackMediaDirection
        .values[api.remoteMediaTrackMediaDirection(track: opaque)];
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return api.remoteMediaTrackGetTrack(track: opaque)
        as webrtc.MediaStreamTrack;
  }

  @override
  void onMuted(void Function() f) {
    return api.remoteMediaTrackOnMuted(track: opaque, f: f);
  }

  @override
  void onUnmuted(void Function() f) {
    return api.remoteMediaTrackOnUnmuted(track: opaque, f: f);
  }

  @override
  void onStopped(void Function() f) {
    return api.remoteMediaTrackOnStopped(track: opaque, f: f);
  }

  @moveSemantics
  @override
  void free() {
    if (!opaque.isStale()) {
      RustHandlesStorage().removeHandle(this);

      opaque.dispose();
    }
  }

  @override
  void onMediaDirectionChanged(void Function(TrackMediaDirection) f) {
    api.remoteMediaTrackOnMediaDirectionChanged(
        track: opaque, f: (i) => f(TrackMediaDirection.values[i]));
  }
}
