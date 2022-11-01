import 'dart:ffi';

import '../interface/media_display_info.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/api_api.g.dart' as api;
import 'jason.dart';

class NativeMediaDisplayInfo extends MediaDisplayInfo {
  /// [Pointer] to the Rust struct backing this object.
  late api.MediaDisplayInfo opaque;

  /// Constructs a new [MediaDisplayInfo] backed by a Rust struct behind the
  /// provided [Pointer].

  NativeMediaDisplayInfo.opaque(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String deviceId() {
    return impl_api.mediaDisplayInfoDeviceId(mediaDisplay: opaque);
  }

  @override
  String? title() {
    return impl_api.mediaDisplayInfoTitle(mediaDisplay: opaque);
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
