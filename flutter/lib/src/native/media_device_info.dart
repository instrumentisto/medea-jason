import '../interface/media_device_info.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';

class NativeMediaDeviceInfo extends MediaDeviceInfo {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late frb.MediaDeviceInfo opaque;

  /// Constructs a new [MediaDeviceInfo] backed by a Rust struct behind the
  /// provided [frb.MediaDeviceInfo].
  NativeMediaDeviceInfo(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String deviceId() {
    return api.mediaDeviceInfoDeviceId(mediaDevice: opaque);
  }

  @override
  String label() {
    return api.mediaDeviceInfoLabel(mediaDevice: opaque);
  }

  @override
  MediaDeviceKind kind() {
    return MediaDeviceKind.values[api.mediaDeviceInfoKind(mediaDevice: opaque)];
  }

  @override
  String? groupId() {
    return api.mediaDeviceInfoGroupId(mediaDevice: opaque);
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
