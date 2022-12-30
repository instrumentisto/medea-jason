import '../interface/media_device_info.dart';
import '../util/move_semantic.dart';
import 'ffi/jason_api.g.dart' as frb;

class NativeMediaDeviceInfo extends MediaDeviceInfo {
  /// Rust `flutter_rust_bridge` api representation.
  final frb.ApiMediaDeviceInfo _info;

  /// Constructs a new [MediaDeviceInfo] backed by a Rust struct behind the
  /// provided [frb.ApiMediaDeviceInfo].
  NativeMediaDeviceInfo(this._info);

  @override
  String deviceId() {
    return _info.deviceId;
  }

  @override
  String label() {
    return _info.label;
  }

  @override
  MediaDeviceKind kind() {
    return _info.kind;
  }

  @override
  String? groupId() {
    return _info.groupId;
  }

  @moveSemantics
  @override
  void free() {}
}
