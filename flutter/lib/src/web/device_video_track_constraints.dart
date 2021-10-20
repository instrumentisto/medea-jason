import '../interface/device_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class DeviceVideoTrackConstraints extends base.DeviceVideoTrackConstraints {
  final wasm.DeviceVideoTrackConstraints obj =
      wasm.DeviceVideoTrackConstraints();

  @override
  void deviceId(String deviceId) {
    fallibleFunction(() => obj.device_id(deviceId));
  }

  @override
  void exactFacingMode(base.FacingMode facingMode) {
    fallibleFunction(() => obj.exact_facing_mode(facingMode.index));
  }

  @override
  void idealFacingMode(base.FacingMode facingMode) {
    fallibleFunction(() => obj.ideal_facing_mode(facingMode.index));
  }

  @override
  void exactHeight(int height) {
    fallibleFunction(() => obj.exact_height(height));
  }

  @override
  void idealHeight(int height) {
    fallibleFunction(() => obj.ideal_height(height));
  }

  @override
  void heightInRange(int min, int max) {
    fallibleFunction(() => obj.height_in_range(min, max));
  }

  @override
  void exactWidth(int width) {
    fallibleFunction(() => obj.exact_width(width));
  }

  @override
  void idealWidth(int width) {
    fallibleFunction(() => obj.ideal_width(width));
  }

  @override
  void widthInRange(int min, int max) {
    fallibleFunction(() => obj.width_in_range(min, max));
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
