import '../interface/media_display_details.dart';
import 'ffi/jason_api.g.dart' as frb;

class NativeMediaDisplayDetails implements MediaDisplayDetails {
  /// Rust `flutter_rust_bridge` API representation.
  final frb.ApiMediaDisplayDetails _info;

  /// Constructs a new [MediaDisplayDetails] backed by a Rust struct behind the
  /// provided [frb.ApiMediaDisplayInfo].
  NativeMediaDisplayDetails(this._info);

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
