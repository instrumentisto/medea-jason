import '../interface/input_device_info.dart';
import '../interface/track_kinds.dart';
import '../web/jason_wasm.dart' as wasm;
import '../util/move_semantic.dart';
import 'exceptions.dart';

class WebInputDeviceInfo extends InputDeviceInfo {
  late wasm.InputDeviceInfo obj;

  WebInputDeviceInfo(this.obj);

  @override
  String deviceId() {
    return failableFunction(() => obj.device_id());
  }

  @override
  String label() {
    return failableFunction(() => obj.label());
  }

  @override
  MediaKind kind() {
    return failableFunction(() => MediaKind.values[obj.kind().toInt()]);
  }

  @override
  String groupId() {
    return failableFunction(() => obj.group_id());
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
