import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/enums.dart';
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/frb//api/dart/api.dart' as frb;

class NativeLocalMediaTrack implements LocalMediaTrack {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final frb.LocalMediaTrack opaque;

  /// Constructs a new [LocalMediaTrack] backed by the Rust struct behind the
  /// provided [frb.LocalMediaTrack].
  NativeLocalMediaTrack(frb.LocalMediaTrack localMediaTrack)
      : opaque = localMediaTrack {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  MediaKind kind() {
    return frb.localMediaTrackKind(track: opaque);
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return frb.localMediaTrackMediaSourceKind(track: opaque);
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return frb.localMediaTrackGetTrack(track: opaque)
        as webrtc.MediaStreamTrack;
  }

  @moveSemantics
  @override
  Future<void> free() async {
    if (!opaque.isDisposed) {
      RustHandlesStorage().removeHandle(this);
      await (frb.localMediaTrackFree(track: opaque) as Future);
    }
  }

  @override
  void onEnded(OnEndedCallback f) {
    frb.localMediaTrackOnEnded(track: opaque, f: f);
  }

  @override
  Future<MediaStreamTrackState> state() async {
    var index = await (frb.localMediaTrackState(track: opaque) as Future);
    return MediaStreamTrackState.values[index];
  }

  @override
  bool isOnAudioLevelAvailable() {
    return frb.isOnAudioLevelAvailable(track: opaque);
  }

  @override
  void onAudioLevelChanged(OnAudioLevelChangedCallback f) {
    frb.onAudioLevelChanged(track: opaque, f: f);
  }
}
