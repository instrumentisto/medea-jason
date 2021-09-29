import '../interface/audio_track_constraints.dart';
import '../web/jason_wasm.dart' as wasm;
import '../util/move_semantic.dart';

class AudioTrackConstraints extends IAudioTrackConstraints {
  final wasm.AudioTrackConstraints obj = wasm.AudioTrackConstraints();

  @override
  void deviceId(String deviceId) {
    obj.device_id(deviceId);
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
