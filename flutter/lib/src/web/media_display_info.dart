import '../interface/media_display_info.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class WebMediaDisplayInfo extends MediaDisplayInfo {
  late wasm.MediaDisplayInfo obj;

  WebMediaDisplayInfo(this.obj);

  @override
  String deviceId() {
    return '';
  }

  @override
  String? title() {
    return null;
  }

  @moveSemantics
  @override
  void free() {
    obj.free();
  }
}
