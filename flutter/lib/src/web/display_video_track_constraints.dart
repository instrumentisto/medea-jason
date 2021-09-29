import '../interface/display_video_track_constraints.dart';
import '../util/move_semantic.dart';
import '../web/jason_wasm.dart' as wasm;

class WebDisplayVideoTrackConstraints extends IDisplayVideoTrackConstraints {
  final wasm.DisplayVideoTrackConstraints obj =
      wasm.DisplayVideoTrackConstraints();

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
