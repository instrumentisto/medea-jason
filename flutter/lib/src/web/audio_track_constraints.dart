import '../interface/audio_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class AudioTrackConstraints extends base.AudioTrackConstraints {
  final wasm.AudioTrackConstraints obj = wasm.AudioTrackConstraints();

  @override
  void deviceId(String deviceId) {
    failableFunction(() => obj.device_id(deviceId));
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
