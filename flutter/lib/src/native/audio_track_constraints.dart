import '../interface/audio_track_constraints.dart' as base;
import 'ffi/jason_api.g.dart' as frb;

class AudioTrackConstraints implements base.AudioTrackConstraints {
  /// Rust `flutter_rust_bridge` API representation.
  final frb.ApiAudioConstraints constraints =
      frb.ApiAudioConstraints(deviceId: null);

  @override
  void deviceId(String deviceId) {
    constraints.deviceId = deviceId;
  }

  @override
  void exactAutoGainControl(bool autoGainControl) {
    constraints.autoGainControl = frb.ConstrainBoolean_Exact(autoGainControl);
  }

  @override
  void idealAutoGainControl(bool autoGainControl) {
    constraints.autoGainControl = frb.ConstrainBoolean_Ideal(autoGainControl);
  }

  @override
  void free() {}
}
