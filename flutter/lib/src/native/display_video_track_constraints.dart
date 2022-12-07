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
    api.displayVideoTrackConstraintsExactHeight(
        constraints: opaque, exactHeight: height);
  }

  @override
  void idealHeight(int height) {
    api.displayVideoTrackConstraintsIdealHeight(
        constraints: opaque, idealHeight: height);
  }

  @override
  void exactWidth(int width) {
    api.displayVideoTrackConstraintsExactWidth(
        constraints: opaque, exactWidth: width);
  }

  @override
  void idealWidth(int width) {
    api.displayVideoTrackConstraintsIdealWidth(
        constraints: opaque, idealWidth: width);
  }

  @override
  void exactFrameRate(int frameRate) {
    api.displayVideoTrackConstraintsExactFrameRate(
        constraints: opaque, exactFrameRate: frameRate);
  }

  @override
  void idealFrameRate(int frameRate) {
    api.displayVideoTrackConstraintsIdealFrameRate(
        constraints: opaque, idealFrameRate: frameRate);
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
