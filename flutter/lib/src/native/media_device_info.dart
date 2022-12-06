import 'dart:ffi';

import 'package:ffi/ffi.dart';

import '../interface/media_device_info.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/foreign_value.dart';
import 'ffi/native_string.dart';
import 'ffi/nullable_pointer.dart';
import 'jason.dart';

typedef _deviceId_C = Pointer<Utf8> Function(Pointer);
typedef _deviceId_Dart = Pointer<Utf8> Function(Pointer);

typedef _label_C = Pointer<Utf8> Function(Pointer);
typedef _label_Dart = Pointer<Utf8> Function(Pointer);

typedef _kind_C = Uint8 Function(Pointer);
typedef _kind_Dart = int Function(Pointer);

typedef _nativeGroupId_C = ForeignValue Function(Pointer);
typedef _nativeGroupId_Dart = ForeignValue Function(Pointer);

typedef _nativeIsFailed_C = Bool Function(Pointer);
typedef _nativeIsFailed_Dart = bool Function(Pointer);

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

final _nativeGroupId = dl.lookupFunction<_nativeGroupId_C, _nativeGroupId_Dart>(
    'MediaDeviceInfo__group_id');

final _kind = dl.lookupFunction<_kind_C, _kind_Dart>('MediaDeviceInfo__kind');

final _label =
    dl.lookupFunction<_label_C, _label_Dart>('MediaDeviceInfo__label');

final _deviceId = dl
    .lookupFunction<_deviceId_C, _deviceId_Dart>('MediaDeviceInfo__device_id');

final _isFailed = dl.lookupFunction<_nativeIsFailed_C, _nativeIsFailed_Dart>(
    'MediaDeviceInfo__is_failed');

final _free = dl.lookupFunction<_free_C, _free_Dart>('MediaDeviceInfo__free');

class NativeMediaDeviceInfo extends MediaDeviceInfo {
  /// [Pointer] to the Rust struct backing this object.
  late NullablePointer ptr;

  /// Constructs a new [MediaDeviceInfo] backed by a Rust struct behind the
  /// provided [Pointer].
  NativeMediaDeviceInfo(this.ptr) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String deviceId() {
    return _deviceId(ptr.getInnerPtr()).nativeStringToDartString();
  }

  @override
  String label() {
    return _label(ptr.getInnerPtr()).nativeStringToDartString();
  }

  @override
  MediaDeviceKind kind() {
    var index = _kind(ptr.getInnerPtr());
    return MediaDeviceKind.values[index];
  }

  @override
  String? groupId() {
    return _nativeGroupId(ptr.getInnerPtr()).toDart();
  }

  @override
  bool isFailed() {
    var isFailed = _isFailed(ptr.getInnerPtr());
    print("isFailed: $isFailed");
    return isFailed;
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
