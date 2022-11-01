import 'dart:ffi';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as api;
import 'jason.dart';

class NativeRemoteMediaTrack extends RemoteMediaTrack {
  /// [Pointer] to the Rust struct that backing this object.
  late api.RemoteMediaTrack opaque;

  /// Constructs a new [RemoteMediaTrack] backed by the Rust struct behind the
  /// provided [Pointer].

  NativeRemoteMediaTrack.opaque(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  bool muted() {
    return impl_api.remoteMediaTrackMuted(track: opaque);
  }

  @override
  MediaKind kind() {
    return MediaKind.values[impl_api.remoteMediaTrackKind(track: opaque)];
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return MediaSourceKind
        .values[impl_api.remoteMediaTrackMediaSourceKind(track: opaque)];
  }

  @override
  TrackMediaDirection mediaDirection() {
    return TrackMediaDirection
        .values[impl_api.remoteMediaTrackMediaDirection(track: opaque)];
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return rust2dart2(impl_api.remoteMediaTrackGetTrack(track: opaque))
        as webrtc.MediaStreamTrack;
  }

  @override
  void onMuted(void Function() f) {
    return impl_api.remoteMediaTrackOnMuted(track: opaque, f: handle2rust(f));
  }

  @override
  void onUnmuted(void Function() f) {
    return impl_api.remoteMediaTrackOnUnmuted(track: opaque, f: handle2rust(f));
  }

  @override
  void onStopped(void Function() f) {
    return impl_api.remoteMediaTrackOnStopped(track: opaque, f: handle2rust(f));
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
    impl_api.remoteMediaTrackOnMediaDirectionChanged(
        track: opaque, f: handle2rust((i) => f(TrackMediaDirection.values[i])));
  }
}
