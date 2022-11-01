import 'dart:ffi';

import '../interface/device_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as api;
import 'jason.dart';

class DeviceVideoTrackConstraints extends base.DeviceVideoTrackConstraints {
  /// [Pointer] to the Rust struct backing this object.

  final api.RefCellDeviceVideoTrackConstraints opaque =
      impl_api.deviceVideoTrackConstraintsNew();

  DeviceVideoTrackConstraints() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void deviceId(String deviceId) {
    impl_api.deviceVideoTrackConstraintsDeviceId(
        constraints: opaque, deviceId: deviceId);
  }

  @override
  void exactFacingMode(base.FacingMode facingMode) {
    impl_api.deviceVideoTrackConstraintsExactFacingMode(
        constraints: opaque, facingMode: facingMode.index);
  }

  @override
  void idealFacingMode(base.FacingMode facingMode) {
    impl_api.deviceVideoTrackConstraintsIdealFacingMode(
        constraints: opaque, facingMode: facingMode.index);
  }

  @override
  void exactHeight(int height) {
    impl_api.deviceVideoTrackConstraintsExactHeight(
        constraints: opaque, exactHeight: height);
  }

  @override
  void idealHeight(int height) {
    impl_api.deviceVideoTrackConstraintsIdealHeight(
        constraints: opaque, idealHeight: height);
  }

  @override
  void heightInRange(int min, int max) {
    impl_api.deviceVideoTrackConstraintsHeightInRange(
        constraints: opaque, min: min, max: max);
  }

  @override
  void exactWidth(int width) {
    impl_api.deviceVideoTrackConstraintsExactWidth(
        constraints: opaque, exactWidth: width);
  }

  @override
  void idealWidth(int width) {
    impl_api.deviceVideoTrackConstraintsIdealWidth(
        constraints: opaque, idealWidth: width);
  }

  @override
  void widthInRange(int min, int max) {
    impl_api.deviceVideoTrackConstraintsWidthInRange(
        constraints: opaque, min: min, max: max);
  }

  @moveSemantics
  @override
  void free() {
    if (!opaque.isStale()) {
      RustHandlesStorage().removeHandle(this);

      opaque.dispose();
    }
  }
}
