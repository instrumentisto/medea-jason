import '../interface/audio_track_constraints.dart' as base;
import 'ffi/jason_api.g.dart' as frb;

class AudioTrackConstraints extends base.AudioTrackConstraints {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final frb.ApiAudioTrackConstraints constraints =
      frb.ApiAudioTrackConstraints(deviceId: null);

  @override
  void deviceId(String deviceId) {
    constraints.deviceId = deviceId;
  }

  @override
  void free() {}
}
