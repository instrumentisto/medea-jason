import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import '../interface/device_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';

class DeviceVideoTrackConstraints extends base.DeviceVideoTrackConstraints {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  RustOpaque<frb.ApiWrapDeviceVideoTrackConstraints> opaque =
      RustOpaque(api.deviceVideoTrackConstrNew());

  /// Constructs a new [DeviceVideoTrackConstraints] backed by the Rust struct behind the
  /// provided [frb.ApiWrapDeviceVideoTrackConstraints].
  DeviceVideoTrackConstraints() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void deviceId(String deviceId) {
    api.deviceVideoTrackConstrDeviceId(
        constr: opaque.innerOpaque, deviceId: deviceId);
  }

  @override
  void exactFacingMode(base.FacingMode facingMode) {
    api.deviceVideoTrackConstrExactFacingMode(
        constr: opaque.innerOpaque, facingMode: facingMode);
  }

  @override
  void idealFacingMode(base.FacingMode facingMode) {
    api.deviceVideoTrackConstrIdealFacingMode(
        constr: opaque.innerOpaque, facingMode: facingMode);
  }

  @override
  void exactHeight(int height) {
    try {
      api.deviceVideoTrackConstrExactHeight(
          constr: opaque.innerOpaque, exactHeight: height);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void idealHeight(int height) {
    try {
      api.deviceVideoTrackConstrIdealHeight(
          constr: opaque.innerOpaque, idealHeight: height);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void heightInRange(int min, int max) {
    try {
      api.deviceVideoTrackConstrHeightInRange(
          constr: opaque.innerOpaque, min: min, max: max);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void exactWidth(int width) {
    try {
      api.deviceVideoTrackConstrExactWidth(
          constr: opaque.innerOpaque, exactWidth: width);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void idealWidth(int width) {
    try {
      api.deviceVideoTrackConstrIdealWidth(
          constr: opaque.innerOpaque, idealWidth: width);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void widthInRange(int min, int max) {
    try {
      api.deviceVideoTrackConstrWidthInRange(
          constr: opaque.innerOpaque, min: min, max: max);
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
