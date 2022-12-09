import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import '../interface/device_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as frb;
import 'jason.dart';

class DeviceVideoTrackConstraints extends base.DeviceVideoTrackConstraints {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  frb.ApiWrapDeviceVideoTrackConstraints opaque =
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
    try {
      api.deviceVideoTrackConstraintsExactHeight(
          constraints: opaque, exactHeight: height);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void idealHeight(int height) {
    try {
      api.deviceVideoTrackConstraintsIdealHeight(
          constraints: opaque, idealHeight: height);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void heightInRange(int min, int max) {
    try {
      api.deviceVideoTrackConstraintsHeightInRange(
          constraints: opaque, min: min, max: max);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void exactWidth(int width) {
    try {
      api.deviceVideoTrackConstraintsExactWidth(
          constraints: opaque, exactWidth: width);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void idealWidth(int width) {
    try {
      api.deviceVideoTrackConstraintsIdealWidth(
          constraints: opaque, idealWidth: width);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void widthInRange(int min, int max) {
    try {
      api.deviceVideoTrackConstraintsWidthInRange(
          constraints: opaque, min: min, max: max);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
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
