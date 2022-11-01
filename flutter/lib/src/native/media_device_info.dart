import 'dart:ffi';

import '../interface/media_device_info.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as api;
import 'jason.dart';

class NativeMediaDeviceInfo extends MediaDeviceInfo {
  /// [Pointer] to the Rust struct backing this object.
  late api.MediaDeviceInfo opaque;

  /// Constructs a new [MediaDeviceInfo] backed by a Rust struct behind the
  /// provided [Pointer].

  NativeMediaDeviceInfo.opaque(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String deviceId() {
    return impl_api.mediaDeviceInfoDeviceId(mediaDevice: opaque);
  }

  @override
  String label() {
    return impl_api.mediaDeviceInfoLabel(mediaDevice: opaque);
  }

  @override
  MediaDeviceKind kind() {
    return MediaDeviceKind
        .values[impl_api.mediaDeviceInfoKind(mediaDevice: opaque)];
  }

  @override
  String? groupId() {
    return impl_api.mediaDeviceInfoGroupId(mediaDevice: opaque);
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
