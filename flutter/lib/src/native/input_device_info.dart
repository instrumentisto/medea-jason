import 'dart:ffi';

import 'package:ffi/ffi.dart';

import '../interface/input_device_info.dart';
import '../interface/track_kinds.dart';
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

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

final _nativeGroupId = dl.lookupFunction<_nativeGroupId_C, _nativeGroupId_Dart>(
    'InputDeviceInfo__group_id');

final _kind = dl.lookupFunction<_kind_C, _kind_Dart>('InputDeviceInfo__kind');

final _label =
    dl.lookupFunction<_label_C, _label_Dart>('InputDeviceInfo__label');

final _deviceId = dl
    .lookupFunction<_deviceId_C, _deviceId_Dart>('InputDeviceInfo__device_id');

final _free = dl.lookupFunction<_free_C, _free_Dart>('InputDeviceInfo__free');

class NativeInputDeviceInfo extends InputDeviceInfo {
  /// [Pointer] to the Rust struct backing this object.
  late NullablePointer ptr;

  /// Constructs a new [InputDeviceInfo] backed by a Rust struct behind the
  /// provided [Pointer].
  NativeInputDeviceInfo(this.ptr) {
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
  MediaKind kind() {
    var index = _kind(ptr.getInnerPtr());
    return MediaKind.values[index];
  }

  @override
  String? groupId() {
    return _nativeGroupId(ptr.getInnerPtr()).toDart();
  }

  @moveSemantics
  @override
  void free() {
    _free(ptr.getInnerPtr());
    ptr.free();
  }
}
