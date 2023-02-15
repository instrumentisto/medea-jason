import '../interface/audio_track_constraints.dart' as base;
import 'ffi/jason_api.g.dart' as frb;

class AudioTrackConstraints implements base.AudioTrackConstraints {
  /// Rust `flutter_rust_bridge` api representation.
  final frb.ApiAudioConstraints constraints =
      frb.ApiAudioConstraints(deviceId: null);

  @override
  void deviceId(String deviceId) {
    constraints.deviceId = deviceId;
  }

  @override
  void free() {}
}
