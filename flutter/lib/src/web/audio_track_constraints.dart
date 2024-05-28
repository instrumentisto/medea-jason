import '../interface/audio_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class AudioTrackConstraints implements base.AudioTrackConstraints {
  final wasm.AudioTrackConstraints obj = wasm.AudioTrackConstraints();

  @override
  void deviceId(String deviceId) {
    fallibleFunction(() => obj.device_id(deviceId));
  }

  @override
  void exactAutoGainControl(bool autoGainControl) {
    fallibleFunction(() => obj.exact_auto_gain_control(autoGainControl));
  }

  @override
  void idealAutoGainControl(bool autoGainControl) {
    fallibleFunction(() => obj.ideal_auto_gain_control(autoGainControl));
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
