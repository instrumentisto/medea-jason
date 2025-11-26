import '../interface/media_device_details.dart';
import 'ffi/frb/frb.dart' as frb;

class NativeMediaDeviceDetails implements MediaDeviceDetails {
  /// Rust `flutter_rust_bridge` API representation.
  final frb.ApiMediaDeviceDetails _info;

  /// Constructs a new [MediaDeviceDetails] backed by a Rust struct behind the
  /// provided [frb.ApiMediaDeviceDetails].
  NativeMediaDeviceDetails(this._info);

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
  AudioDeviceKind? audioDeviceKind() {
    final idx = _info.audioDeviceKind;
    return idx == null ? null : AudioDeviceKind.values[idx.toInt()];
  }

  @override
  String? groupId() {
    return _info.groupId;
  }

  @override
  bool isFailed() {
    return _info.isFailed;
  }

  @override
  void free() {}
}
