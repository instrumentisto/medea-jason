import '../interface/display_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class DisplayVideoTrackConstraints extends base.DisplayVideoTrackConstraints {
  final wasm.DisplayVideoTrackConstraints obj =
      wasm.DisplayVideoTrackConstraints();

  @override
  void exactHeight(int height) {
    fallibleFunction(() => obj.exact_height(height));
  }

  @override
  void idealHeight(int height) {
    fallibleFunction(() => obj.ideal_height(height));
  }

  @override
  void exactWidth(int width) {
    fallibleFunction(() => obj.exact_width(width));
  }

  @override
  void idealWidth(int width) {
    fallibleFunction(() => obj.ideal_width(width));
  }

  @override
  void exactFrameRate(int frameRate) {
    fallibleFunction(() => obj.exact_frame_rate(frameRate));
  }

  @override
  void idealFrameRate(int frameRate) {
    fallibleFunction(() => obj.ideal_frame_rate(frameRate));
  }

  @override
  void deviceId(String deviceId) {
    // no-op
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
