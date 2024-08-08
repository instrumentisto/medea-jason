import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/enums.dart';
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/frb/frb.dart' as frb;

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
    return opaque.kind();
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return opaque.mediaSourceKind();
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return opaque.getTrack() as webrtc.MediaStreamTrack;
  }

  @moveSemantics
  @override
  Future<void> free() async {
    if (!opaque.isDisposed) {
      RustHandlesStorage().removeHandle(this);
      await (opaque.free() as Future);
    }
  }

  @override
  void onEnded(OnEndedCallback f) {
    opaque.onEnded(f: f);
  }

  @override
  Future<MediaStreamTrackState> state() async {
    var index = await (opaque.state() as Future);
    return MediaStreamTrackState.values[index];
  }

  @override
  bool isOnAudioLevelAvailable() {
    return opaque.isOnAudioLevelAvailable();
  }

  @override
  void onAudioLevelChanged(OnAudioLevelChangedCallback f) {
    opaque.onAudioLevelChanged(f: f);
  }
}
