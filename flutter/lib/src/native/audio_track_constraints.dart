import '../interface/audio_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';

class AudioTrackConstraints extends base.AudioTrackConstraints {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  frb.AudioTrackConstraints opaque = api.audioTrackConstrNew();

  /// Constructs a new [AudioTrackConstraints] backed by a Rust struct behind the
  /// provided [frb.AudioTrackConstraints].
  AudioTrackConstraints() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void deviceId(String deviceId) {
    opaque.move = true;
    opaque = api.audioTrackConstrDeviceId(track: opaque, deviceId: deviceId);
  }

  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  @override
  void free() {
    if (!opaque.isStale()) {
      RustHandlesStorage().removeHandle(this);

      opaque.dispose();
    }
  }
}
