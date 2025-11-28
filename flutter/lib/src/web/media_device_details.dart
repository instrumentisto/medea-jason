import '../interface/media_device_details.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class WebMediaDeviceDetails implements MediaDeviceDetails {
  late wasm.MediaDeviceDetails obj;

  WebMediaDeviceDetails(this.obj);

  @override
  String deviceId() {
    return fallibleFunction(() => obj.device_id());
  }

  @override
  String label() {
    return fallibleFunction(() => obj.label());
  }

  @override
  MediaDeviceKind kind() {
    return fallibleFunction(() => MediaDeviceKind.values[obj.kind().toInt()]);
  }

  @override
  String? groupId() {
    return fallibleFunction(() => obj.group_id());
  }

  @override
  AudioDeviceKind? audioDeviceKind() {
    // No way to implement on web.
    return null;
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }

  @override
  bool isFailed() {
    return false;
  }
}
