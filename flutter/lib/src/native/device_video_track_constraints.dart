import '../interface/device_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as frb;
import 'jason.dart';

class DeviceVideoTrackConstraints extends base.DeviceVideoTrackConstraints {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  frb.DeviceVideoTrackConstraints opaque = api.deviceVideoTrackConstraintsNew();

  /// Constructs a new [DeviceVideoTrackConstraints] backed by the Rust struct behind the
  /// provided [frb.RefCellDeviceVideoTrackConstraints].
  DeviceVideoTrackConstraints() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void deviceId(String deviceId) {
    opaque.move = true;
    opaque = api.deviceVideoTrackConstraintsDeviceId(
        constraints: opaque, deviceId: deviceId);
  }

  @override
  void exactFacingMode(base.FacingMode facingMode) {
    opaque.move = true;
    opaque = api.deviceVideoTrackConstraintsExactFacingMode(
        constraints: opaque, facingMode: facingMode.index);
  }

  @override
  void idealFacingMode(base.FacingMode facingMode) {
    opaque.move = true;
    opaque = api.deviceVideoTrackConstraintsIdealFacingMode(
        constraints: opaque, facingMode: facingMode.index);
  }

  @override
  void exactHeight(int height) {
    opaque.move = true;
    opaque = api.deviceVideoTrackConstraintsExactHeight(
        constraints: opaque, exactHeight: height);
  }

  @override
  void idealHeight(int height) {
    opaque.move = true;
    opaque = api.deviceVideoTrackConstraintsIdealHeight(
        constraints: opaque, idealHeight: height);
  }

  @override
  void heightInRange(int min, int max) {
    opaque.move = true;
    opaque = api.deviceVideoTrackConstraintsHeightInRange(
        constraints: opaque, min: min, max: max);
  }

  @override
  void exactWidth(int width) {
    opaque.move = true;
    opaque = api.deviceVideoTrackConstraintsExactWidth(
        constraints: opaque, exactWidth: width);
  }

  @override
  void idealWidth(int width) {
    opaque.move = true;
    opaque = api.deviceVideoTrackConstraintsIdealWidth(
        constraints: opaque, idealWidth: width);
  }

  @override
  void widthInRange(int min, int max) {
    opaque.move = true;
    opaque = api.deviceVideoTrackConstraintsWidthInRange(
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
