import 'package:medea_jason/src/web/exceptions.dart';

import '../interface/device_video_track_constraints.dart';
import '../web/jason_wasm.dart' as wasm;
import '../util/move_semantic.dart';

class DeviceVideoTrackConstraints extends IDeviceVideoTrackConstraints {
  final wasm.DeviceVideoTrackConstraints obj =
      wasm.DeviceVideoTrackConstraints();

  @override
  void deviceId(String deviceId) {
    failableFunction(() => obj.device_id(deviceId));
  }

  @override
  void exactFacingMode(FacingMode facingMode) {
    failableFunction(() => obj.exact_facing_mode(facingMode.index));
  }

  @override
  void idealFacingMode(FacingMode facingMode) {
    failableFunction(() => obj.ideal_facing_mode(facingMode.index));
  }

  @override
  void exactHeight(int height) {
    failableFunction(() => obj.exact_height(height));
  }

  @override
  void idealHeight(int height) {
    failableFunction(() => obj.ideal_height(height));
  }

  @override
  void heightInRange(int min, int max) {
    failableFunction(() => obj.height_in_range(min, max));
  }

  @override
  void exactWidth(int width) {
    failableFunction(() => obj.exact_width(width));
  }

  @override
  void idealWidth(int width) {
    failableFunction(() => obj.ideal_width(width));
  }

  @override
  void widthInRange(int min, int max) {
    failableFunction(() => obj.width_in_range(min, max));
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
