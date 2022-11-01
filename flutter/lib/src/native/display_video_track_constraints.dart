import 'dart:ffi';

import '../interface/display_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as api;
import 'jason.dart';

class DisplayVideoTrackConstraints extends base.DisplayVideoTrackConstraints {
  /// [Pointer] to the Rust struct backing this object.
  final api.RefCellDisplayVideoTrackConstraints opaque =
      impl_api.displayVideoTrackConstraintsNew();

  DisplayVideoTrackConstraints() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void exactHeight(int height) {
    impl_api.displayVideoTrackConstraintsExactHeight(
        constraints: opaque, exactHeight: height);
  }

  @override
  void idealHeight(int height) {
    impl_api.displayVideoTrackConstraintsIdealHeight(
        constraints: opaque, idealHeight: height);
  }

  @override
  void exactWidth(int width) {
    impl_api.displayVideoTrackConstraintsExactWidth(
        constraints: opaque, exactWidth: width);
  }

  @override
  void idealWidth(int width) {
    impl_api.displayVideoTrackConstraintsIdealWidth(
        constraints: opaque, idealWidth: width);
  }

  @override
  void exactFrameRate(int frameRate) {
    impl_api.displayVideoTrackConstraintsExactFrameRate(
        constraints: opaque, exactFrameRate: frameRate);
  }

  @override
  void idealFrameRate(int frameRate) {
    impl_api.displayVideoTrackConstraintsIdealFrameRate(
        constraints: opaque, idealFrameRate: frameRate);
  }

  @override
  void deviceId(String deviceId) {
    impl_api.displayVideoTrackConstraintsDeviceId(
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
