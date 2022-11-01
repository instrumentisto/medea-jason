import 'dart:ffi';

import '../interface/audio_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as api;
import 'jason.dart';

class AudioTrackConstraints extends base.AudioTrackConstraints {
  /// [Pointer] to the Rust struct backing this object.

  final api.RefCellAudioTrackConstraints opaque =
      impl_api.audioTrackConstraintsNew();

  AudioTrackConstraints() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void deviceId(String deviceId) {
    impl_api.audioTrackConstraintsDeviceId(track: opaque, deviceId: deviceId);
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
