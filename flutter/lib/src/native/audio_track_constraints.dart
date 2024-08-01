import '../interface/audio_track_constraints.dart' as base;
import 'ffi/frb/api/dart/api.dart' as frb;
import 'ffi/frb/media/constraints.dart';

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
    constraints.autoGainControl = ConstrainBoolean.exact(autoGainControl);
  }

  @override
  void idealAutoGainControl(bool autoGainControl) {
    constraints.autoGainControl = ConstrainBoolean.ideal(autoGainControl);
  }

  @override
  void free() {}
}
