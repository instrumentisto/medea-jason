import '../interface/media_display_info.dart';
import 'ffi/jason_api.g.dart' as frb;

class NativeMediaDisplayInfo implements MediaDisplayInfo {
  /// Rust `flutter_rust_bridge` API representation.
  final frb.ApiMediaDisplayInfo _info;

  /// Constructs a new [MediaDisplayInfo] backed by a Rust struct behind the
  /// provided [frb.ApiMediaDisplayInfo].
  NativeMediaDisplayInfo(this._info);

  @override
  String deviceId() {
    return _info.deviceId;
  }

  @override
  String? title() {
    return _info.title;
  }

  @override
  void free() {}
}
