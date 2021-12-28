import '../interface/input_device_info.dart';
import '../interface/track_kinds.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class WebInputDeviceInfo extends InputDeviceInfo {
  late wasm.InputDeviceInfo obj;

  WebInputDeviceInfo(this.obj);

  @override
  String deviceId() {
    return fallibleFunction(() => obj.device_id());
  }

  @override
  String label() {
    return fallibleFunction(() => obj.label());
  }

  @override
  MediaKind kind() {
    return fallibleFunction(() => MediaKind.values[obj.kind().toInt()]);
  }

  @override
  String? groupId() {
    return fallibleFunction(() => obj.group_id());
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
