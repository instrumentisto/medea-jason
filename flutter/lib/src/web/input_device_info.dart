import '../interface/input_device_info.dart';
import '../interface/track_kinds.dart';
import '../util/move_semantic.dart';
import 'jason_wasm.dart' as wasm;

class WebInputDeviceInfo extends InputDeviceInfo {
  late wasm.InputDeviceInfo obj;

  WebInputDeviceInfo(this.obj);

  @override
  String deviceId() {
    return obj.device_id();
  }

  @override
  String label() {
    return obj.label();
  }

  @override
  MediaKind kind() {
    return MediaKind.values[obj.kind().toInt()];
  }

  @override
  String groupId() {
    return obj.group_id();
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
