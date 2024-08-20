import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/enums.dart';
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/frb/frb.dart' as frb;

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
    return opaque.inner.kind();
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return opaque.inner.mediaSourceKind();
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return opaque.inner.getTrack() as webrtc.MediaStreamTrack;
  }

  @moveSemantics
  @override
  Future<void> free() async {
    if (!opaque.isDisposed) {
      RustHandlesStorage().removeHandle(this);
      await (opaque.inner.free() as Future);
      opaque.dispose();
    }
  }

  @override
  void onEnded(OnEndedCallback f) {
    opaque.inner.onEnded(f: f);
  }

  @override
  Future<MediaStreamTrackState> state() async {
    var index = await (opaque.inner.state() as Future);
    return MediaStreamTrackState.values[index];
  }

  @override
  bool isOnAudioLevelAvailable() {
    return opaque.inner.isOnAudioLevelAvailable();
  }

  @override
  void onAudioLevelChanged(OnAudioLevelChangedCallback f) {
    opaque.inner.onAudioLevelChanged(f: f);
  }
}
