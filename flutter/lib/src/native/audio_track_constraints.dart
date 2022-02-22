import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/util/rust_handles_storage.dart';

import '../interface/audio_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import 'ffi/nullable_pointer.dart';
import 'jason.dart';

// TODO: Typedefs should have names that are not bound to concrete methods and
//       moved to shared lib, so they can be reused and won't pollute things.
typedef _new_C = Pointer Function();
typedef _new_Dart = Pointer Function();

typedef _deviceId_C = Void Function(Pointer, Pointer<Utf8>);
typedef _deviceId_Dart = void Function(Pointer, Pointer<Utf8>);

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

final _new = dl.lookupFunction<_new_C, _new_Dart>('AudioTrackConstraints__new');

final _deviceId = dl.lookupFunction<_deviceId_C, _deviceId_Dart>(
    'AudioTrackConstraints__device_id');

final _free =
    dl.lookupFunction<_free_C, _free_Dart>('AudioTrackConstraints__free');

class AudioTrackConstraints extends base.AudioTrackConstraints {
  /// [Pointer] to the Rust struct backing this object.
  final NullablePointer ptr = NullablePointer(_new());

  AudioTrackConstraints() {
    RustHandlesStorage().insertHandle(this);
  }

  @override
  void deviceId(String deviceId) {
    var deviceIdPtr = deviceId.toNativeUtf8();
    try {
      _deviceId(ptr.getInnerPtr(), deviceIdPtr);
    } finally {
      calloc.free(deviceIdPtr);
    }
  }

  /// Drops the associated Rust struct and nulls the local [Pointer] to it.
  @moveSemantics
  @override
  void free() {
    _free(ptr.getInnerPtr());
    ptr.free();
  }
}
