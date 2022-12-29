import '../interface/media_display_info.dart';
import '../util/move_semantic.dart';
import 'ffi/jason_api.g.dart' as frb;

class NativeMediaDisplayInfo extends MediaDisplayInfo {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  final frb.ApiMediaDisplayInfo _info;

  /// Constructs a new [MediaDisplayInfo] backed by a Rust struct behind the
  /// provided [frb.MediaDisplayInfo].
  NativeMediaDisplayInfo(this._info);

  @override
  String deviceId() {
    return _info.deviceId;
  }

  @override
  String? title() {
    return _info.title;
  }

  @moveSemantics
  @override
  void free() {}
}
