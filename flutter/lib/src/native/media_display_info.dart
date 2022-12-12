import '../interface/media_display_info.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/jason_api.g.dart' as frb;
import 'jason.dart';

class NativeMediaDisplayInfo extends MediaDisplayInfo {
  /// `flutter_rust_bridge` Rust opaque type backing this object.
  late frb.MediaDisplayInfo opaque;

  /// Constructs a new [MediaDisplayInfo] backed by a Rust struct behind the
  /// provided [frb.MediaDisplayInfo].
  NativeMediaDisplayInfo(this.opaque) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String deviceId() {
    return api.mediaDisplayInfoDeviceId(mediaDisplay: opaque);
  }

  @override
  String? title() {
    return api.mediaDisplayInfoTitle(mediaDisplay: opaque);
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
