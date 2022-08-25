import 'dart:ffi';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;

import '../interface/remote_media_track.dart';
import '../interface/track_kinds.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/foreign_value.dart';
import 'ffi/nullable_pointer.dart';
import 'jason.dart';

typedef _muted_C = Uint8 Function(Pointer);
typedef _muted_Dart = int Function(Pointer);

typedef _kind_C = Uint8 Function(Pointer);
typedef _kind_Dart = int Function(Pointer);

typedef _mediaSourceKind_C = Uint8 Function(Pointer);
typedef _mediaSourceKind_Dart = int Function(Pointer);

typedef _mediaDirection_C = Uint8 Function(Pointer);
typedef _mediaDirection_Dart = int Function(Pointer);

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

typedef _onMuted_C = Void Function(Pointer, Handle);
typedef _onMuted_Dart = void Function(Pointer, void Function());

typedef _onUnmuted_C = Void Function(Pointer, Handle);
typedef _onUnmuted_Dart = void Function(Pointer, void Function());

typedef _onStopped_C = Void Function(Pointer, Handle);
typedef _onStopped_Dart = void Function(Pointer, void Function());

typedef _onMediaDirectionChanged_C = Void Function(Pointer, Handle);
typedef _onMediaDirectionChanged_Dart = void Function(
    Pointer, void Function(int));

typedef _getTrack_C = ForeignValue Function(Pointer);
typedef _getTrack_Dart = ForeignValue Function(Pointer);

typedef _waitTrack_C = Handle Function(Pointer);
typedef _waitTrack_Dart = Object Function(Pointer);

final _muted =
    dl.lookupFunction<_muted_C, _muted_Dart>('RemoteMediaTrack__muted');

final _kind = dl.lookupFunction<_kind_C, _kind_Dart>('RemoteMediaTrack__kind');

final _mediaSourceKind =
    dl.lookupFunction<_mediaSourceKind_C, _mediaSourceKind_Dart>(
        'RemoteMediaTrack__media_source_kind');

final _mediaDirectionKind =
    dl.lookupFunction<_mediaDirection_C, _mediaDirection_Dart>(
        'RemoteMediaTrack__media_direction');

final _onMuted =
    dl.lookupFunction<_onMuted_C, _onMuted_Dart>('RemoteMediaTrack__on_muted');

final _onUnmuted = dl.lookupFunction<_onUnmuted_C, _onUnmuted_Dart>(
    'RemoteMediaTrack__on_unmuted');

final _onStopped = dl.lookupFunction<_onStopped_C, _onStopped_Dart>(
    'RemoteMediaTrack__on_stopped');

final _onMediaDirectionChanged = dl
    .lookupFunction<_onMediaDirectionChanged_C, _onMediaDirectionChanged_Dart>(
        'RemoteMediaTrack__on_media_direction_changed');

final _getTrack = dl
    .lookupFunction<_getTrack_C, _getTrack_Dart>('RemoteMediaTrack__get_track');

final _waitTrack = dl.lookupFunction<_waitTrack_C, _waitTrack_Dart>(
    'RemoteMediaTrack__wait_track');

final _free = dl.lookupFunction<_free_C, _free_Dart>('RemoteMediaTrack__free');

class NativeRemoteMediaTrack extends RemoteMediaTrack {
  /// [Pointer] to the Rust struct that backing this object.
  late NullablePointer ptr;

  /// Constructs a new [RemoteMediaTrack] backed by the Rust struct behind the
  /// provided [Pointer].
  NativeRemoteMediaTrack(this.ptr) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  bool muted() {
    return _muted(ptr.getInnerPtr()) > 0;
  }

  @override
  MediaKind kind() {
    var index = _kind(ptr.getInnerPtr());
    return MediaKind.values[index];
  }

  @override
  MediaSourceKind mediaSourceKind() {
    var index = _mediaSourceKind(ptr.getInnerPtr());
    return MediaSourceKind.values[index];
  }

  @override
  TrackMediaDirection mediaDirection() {
    var index = _mediaDirectionKind(ptr.getInnerPtr());
    return TrackMediaDirection.values[index];
  }

  @override
  webrtc.MediaStreamTrack? getTrack() {
    return _getTrack(ptr.getInnerPtr()).toDart();
  }

  @override
  Future<webrtc.MediaStreamTrack> waitTrack() async {
    return await (_waitTrack(ptr.getInnerPtr()) as Future);
  }

  @override
  void onMuted(void Function() f) {
    _onMuted(ptr.getInnerPtr(), f);
  }

  @override
  void onUnmuted(void Function() f) {
    _onUnmuted(ptr.getInnerPtr(), f);
  }

  @override
  void onStopped(void Function() f) {
    _onStopped(ptr.getInnerPtr(), f);
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

  @override
  void onMediaDirectionChanged(void Function(TrackMediaDirection) f) {
    _onMediaDirectionChanged(
        ptr.getInnerPtr(), (i) => f(TrackMediaDirection.values[i]));
  }
}
