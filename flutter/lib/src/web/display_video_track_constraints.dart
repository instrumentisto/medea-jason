import '../interface/display_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import 'jason_wasm.dart' as wasm;

class DisplayVideoTrackConstraints extends base.DisplayVideoTrackConstraints {
  final wasm.DisplayVideoTrackConstraints obj =
      wasm.DisplayVideoTrackConstraints();

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
