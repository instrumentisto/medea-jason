import '../interface/audio_track_constraints.dart' as base;
import 'ffi/jason_api.g.dart' as frb;

class AudioTrackConstraints extends base.AudioTrackConstraints {
  /// Rust `flutter_rust_bridge` api representation.
  final frb.ApiAudioTrackConstrs constraints =
      frb.ApiAudioTrackConstrs(deviceId: null);

  @override
  void deviceId(String deviceId) {
    constraints.deviceId = deviceId;
  }

  @override
  void free() {}
}
