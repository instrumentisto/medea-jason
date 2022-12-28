import '../interface/media_device_info.dart';
import '../util/move_semantic.dart';
import '../util/rust_opaque.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';

class NativeMediaDeviceInfo extends MediaDeviceInfo {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late RustOpaque<frb.MediaDeviceInfo> opaque;

  /// Constructs a new [MediaDeviceInfo] backed by a Rust struct behind the
  /// provided [frb.MediaDeviceInfo].
  NativeMediaDeviceInfo(frb.MediaDeviceInfo mediaDeviceInfo)
      : opaque = RustOpaque(mediaDeviceInfo) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String deviceId() {
    return api.mediaDeviceInfoDeviceId(mediaDevice: opaque.innerOpaque);
  }

  @override
  String label() {
    return api.mediaDeviceInfoLabel(mediaDevice: opaque.innerOpaque);
  }

  @override
  MediaDeviceKind kind() {
    return api.mediaDeviceInfoKind(mediaDevice: opaque.innerOpaque);
  }

  @override
  String? groupId() {
    return api.mediaDeviceInfoGroupId(mediaDevice: opaque.innerOpaque);
  }

  @moveSemantics
  @override
  void free() {
    if (!opaque.isStale()) {
      RustHandlesStorage().removeHandle(this);
      opaque.dispose();
    }
  }
}
