import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import '../interface/display_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';

class DisplayVideoTrackConstraints extends base.DisplayVideoTrackConstraints {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  frb.ApiWrapDisplayVideoTrackConstraints opaque =
      api.displayVideoTrackConstrNew();

  /// Constructs a new [DisplayVideoTrackConstraints] backed by the Rust struct behind the
  /// provided [frb.ApiWrapDisplayVideoTrackConstraints].
  DisplayVideoTrackConstraints() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void exactHeight(int height) {
    try {
      api.displayVideoTrackConstrExactHeight(
          constr: opaque, exactHeight: height);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void idealHeight(int height) {
    try {
      api.displayVideoTrackConstrIdealHeight(
          constr: opaque, idealHeight: height);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void exactWidth(int width) {
    try {
      api.displayVideoTrackConstrExactWidth(
          constr: opaque, exactWidth: width);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void idealWidth(int width) {
    try {
      api.displayVideoTrackConstrIdealWidth(
          constr: opaque, idealWidth: width);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void exactFrameRate(int frameRate) {
    try {
      api.displayVideoTrackConstrExactFrameRate(
          constr: opaque, exactFrameRate: frameRate);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void idealFrameRate(int frameRate) {
    try {
      api.displayVideoTrackConstrIdealFrameRate(
          constr: opaque, idealFrameRate: frameRate);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow);
    }
  }

  @override
  void deviceId(String deviceId) {
    api.displayVideoTrackConstrDeviceId(
        constr: opaque, deviceId: deviceId);
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
