import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import '../interface/display_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as frb;
import 'jason.dart';

class DisplayVideoTrackConstraints extends base.DisplayVideoTrackConstraints {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  frb.ApiWrapDisplayVideoTrackConstraints opaque =
      api.displayVideoTrackConstraintsNew();

  /// Constructs a new [DisplayVideoTrackConstraints] backed by the Rust struct behind the
  /// provided [frb.RefCellDisplayVideoTrackConstraints].
  DisplayVideoTrackConstraints() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void exactHeight(int height) {
    try {
      api.displayVideoTrackConstraintsExactHeight(
          constraints: opaque, exactHeight: height);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow.message);
    }
  }

  @override
  void idealHeight(int height) {
    try {
      api.displayVideoTrackConstraintsIdealHeight(
          constraints: opaque, idealHeight: height);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow.message);
    }
  }

  @override
  void exactWidth(int width) {
    try {
      api.displayVideoTrackConstraintsExactWidth(
          constraints: opaque, exactWidth: width);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow.message);
    }
  }

  @override
  void idealWidth(int width) {
    try {
      api.displayVideoTrackConstraintsIdealWidth(
          constraints: opaque, idealWidth: width);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow.message);
    }
  }

  @override
  void exactFrameRate(int frameRate) {
    try {
      api.displayVideoTrackConstraintsExactFrameRate(
          constraints: opaque, exactFrameRate: frameRate);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow.message);
    }
  }

  @override
  void idealFrameRate(int frameRate) {
    try {
      api.displayVideoTrackConstraintsIdealFrameRate(
          constraints: opaque, idealFrameRate: frameRate);
    } on FfiException catch (anyhow) {
      throw objectFromAnyhow(anyhow.message);
    }
  }

  @override
  void deviceId(String deviceId) {
    api.displayVideoTrackConstraintsDeviceId(
        constraints: opaque, deviceId: deviceId);
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
