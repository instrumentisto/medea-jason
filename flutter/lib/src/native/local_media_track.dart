import 'dart:ffi';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/local_media_track.dart';
import '../interface/track_kinds.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/nullable_pointer.dart';
import 'jason.dart';

typedef _kind_C = Uint8 Function(Pointer);
typedef _kind_Dart = int Function(Pointer);

typedef _mediaSourceKind_C = Uint8 Function(Pointer);
typedef _mediaSourceKind_Dart = int Function(Pointer);

typedef _getTrack_C = Handle Function(Pointer);
typedef _getTrack_Dart = Object Function(Pointer);

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

final _kind = dl.lookupFunction<_kind_C, _kind_Dart>('LocalMediaTrack__kind');

final _sourceKind =
    dl.lookupFunction<_mediaSourceKind_C, _mediaSourceKind_Dart>(
        'LocalMediaTrack__media_source_kind');

final _getTrack = dl
    .lookupFunction<_getTrack_C, _getTrack_Dart>('LocalMediaTrack__get_track');

final _free = dl.lookupFunction<_free_C, _free_Dart>('LocalMediaTrack__free');

class NativeLocalMediaTrack extends LocalMediaTrack {
  /// [Pointer] to the Rust struct backing this object.
  late NullablePointer ptr;

  /// Constructs a new [LocalMediaTrack] backed by the Rust struct behind the
  /// provided [Pointer].
  NativeLocalMediaTrack(this.ptr) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  MediaKind kind() {
    var index = _kind(ptr.getInnerPtr());
    return MediaKind.values[index];
  }

  @override
  MediaSourceKind mediaSourceKind() {
    var index = _sourceKind(ptr.getInnerPtr());
    return MediaSourceKind.values[index];
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return _getTrack(ptr.getInnerPtr()) as webrtc.MediaStreamTrack;
  }

  @moveSemantics
  @override
  void free() {
    if (!ptr.isFreed()) {
      RustHandlesStorage().removeHandle(this);
      _free(ptr.getInnerPtr());
      ptr.free();
    }
  }
}
