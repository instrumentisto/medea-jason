import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/enums.dart';
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/frb//api/dart/api.dart' as frb;

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
    return frb.localMediaTrackKind(track: opaque.innerOpaque);
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return frb.localMediaTrackMediaSourceKind(track: opaque.innerOpaque);
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return frb.localMediaTrackGetTrack(track: opaque.innerOpaque)
        as webrtc.MediaStreamTrack;
  }

  @moveSemantics
  @override
  Future<void> free() async {
    if (!opaque.isStale()) {
      RustHandlesStorage().removeHandle(this);
      await (frb.localMediaTrackFree(track: opaque.moveOpaque) as Future);
    }
  }

  @override
  void onEnded(OnEndedCallback f) {
    frb.localMediaTrackOnEnded(track: opaque.innerOpaque, f: f);
  }

  @override
  Future<MediaStreamTrackState> state() async {
    var index =
        await (frb.localMediaTrackState(track: opaque.innerOpaque) as Future);
    return MediaStreamTrackState.values[index];
  }

  @override
  bool isOnAudioLevelAvailable() {
    return frb.isOnAudioLevelAvailable(track: opaque.innerOpaque);
  }

  @override
  void onAudioLevelChanged(OnAudioLevelChangedCallback f) {
    frb.onAudioLevelChanged(track: opaque.innerOpaque, f: f);
  }
}
