import '../interface/device_video_track_constraints.dart';
import '../web/jason_wasm.dart' as wasm;
import '../util/move_semantic.dart';

class DeviceVideoTrackConstraints extends IDeviceVideoTrackConstraints {
  final wasm.DeviceVideoTrackConstraints obj =
      wasm.DeviceVideoTrackConstraints();

  @override
  void deviceId(String deviceId) {
    obj.device_id(deviceId);
  }

  @override
  void exactFacingMode(FacingMode facingMode) {
    obj.exact_facing_mode(facingMode.index);
  }

  @override
  void idealFacingMode(FacingMode facingMode) {
    obj.ideal_facing_mode(facingMode.index);
  }

  @override
  void exactHeight(int height) {
    obj.exact_height(height);
  }

  @override
  void idealHeight(int height) {
    obj.ideal_height(height);
  }

  @override
  void heightInRange(int min, int max) {
    obj.height_in_range(min, max);
  }

  @override
  void exactWidth(int width) {
    obj.exact_width(width);
  }

  @override
  void idealWidth(int width) {
    obj.ideal_width(width);
  }

  @override
  void widthInRange(int min, int max) {
    obj.width_in_range(min, max);
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
