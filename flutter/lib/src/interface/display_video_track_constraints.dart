import '../util/move_semantic.dart';

abstract class IDisplayVideoTrackConstraints {
  @moveSemantics
  void free() {
    throw UnimplementedError();
  }
}
