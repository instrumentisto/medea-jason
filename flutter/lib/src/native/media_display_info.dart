import 'dart:ffi';

import 'package:ffi/ffi.dart';

import '../interface/media_display_info.dart';
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/foreign_value.dart';
import 'ffi/native_string.dart';
import 'ffi/nullable_pointer.dart';
import 'jason.dart';

typedef _deviceId_C = Pointer<Utf8> Function(Pointer);
typedef _deviceId_Dart = Pointer<Utf8> Function(Pointer);

typedef _title_C = ForeignValue Function(Pointer);
typedef _title_Dart = ForeignValue Function(Pointer);

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

final _title =
    dl.lookupFunction<_title_C, _title_Dart>('MediaDisplayInfo__title');

final _deviceId = dl
    .lookupFunction<_deviceId_C, _deviceId_Dart>('MediaDisplayInfo__device_id');

final _free = dl.lookupFunction<_free_C, _free_Dart>('MediaDisplayInfo__free');

class NativeMediaDisplayInfo implements MediaDisplayInfo {
  /// [Pointer] to the Rust struct backing this object.
  late NullablePointer ptr;

  /// Constructs a new [MediaDisplayInfo] backed by a Rust struct behind the
  /// provided [Pointer].
  NativeMediaDisplayInfo(this.ptr) {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  String deviceId() {
    return _deviceId(ptr.getInnerPtr()).nativeStringToDartString();
  }

  @override
  String? title() {
    return _title(ptr.getInnerPtr()).toDart();
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
