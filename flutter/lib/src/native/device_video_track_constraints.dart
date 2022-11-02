import '../interface/device_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as frb;
import 'jason.dart';

class DeviceVideoTrackConstraints extends base.DeviceVideoTrackConstraints {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final frb.RefCellDeviceVideoTrackConstraints opaque =
      api.deviceVideoTrackConstraintsNew();

  /// Constructs a new [DeviceVideoTrackConstraints] backed by the Rust struct behind the
  /// provided [frb.RefCellDeviceVideoTrackConstraints].
  DeviceVideoTrackConstraints() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void deviceId(String deviceId) {
    api.deviceVideoTrackConstraintsDeviceId(
        constraints: opaque, deviceId: deviceId);
  }

  @override
  void exactFacingMode(base.FacingMode facingMode) {
    api.deviceVideoTrackConstraintsExactFacingMode(
        constraints: opaque, facingMode: facingMode.index);
  }

  @override
  void idealFacingMode(base.FacingMode facingMode) {
    api.deviceVideoTrackConstraintsIdealFacingMode(
        constraints: opaque, facingMode: facingMode.index);
  }

  @override
  void exactHeight(int height) {
    api.deviceVideoTrackConstraintsExactHeight(
        constraints: opaque, exactHeight: height);
  }

  @override
  void idealHeight(int height) {
    api.deviceVideoTrackConstraintsIdealHeight(
        constraints: opaque, idealHeight: height);
  }

  @override
  void heightInRange(int min, int max) {
    api.deviceVideoTrackConstraintsHeightInRange(
        constraints: opaque, min: min, max: max);
  }

  @override
  void exactWidth(int width) {
    api.deviceVideoTrackConstraintsExactWidth(
        constraints: opaque, exactWidth: width);
  }

  @override
  void idealWidth(int width) {
    api.deviceVideoTrackConstraintsIdealWidth(
        constraints: opaque, idealWidth: width);
  }

  @override
  void widthInRange(int min, int max) {
    api.deviceVideoTrackConstraintsWidthInRange(
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
