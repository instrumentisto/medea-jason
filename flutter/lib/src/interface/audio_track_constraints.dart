import '../util/move_semantic.dart';

abstract class IAudioTrackConstraints {
  void deviceId(String deviceId) {
    throw UnimplementedError();
  }

  @moveSemantics
  void free() {
    throw UnimplementedError();
  }
}
