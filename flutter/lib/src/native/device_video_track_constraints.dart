import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/util/rust_handles_storage.dart';

import '../interface/device_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import 'ffi/nullable_pointer.dart';
import 'ffi/result.dart';
import 'jason.dart';

typedef _new_C = Pointer Function();
typedef _new_Dart = Pointer Function();

typedef _deviceId_C = Void Function(Pointer, Pointer<Utf8>);
typedef _deviceId_Dart = void Function(Pointer, Pointer<Utf8>);

typedef _exactFacingMode_C = Void Function(Pointer, Uint8);
typedef _exactFacingMode_Dart = void Function(Pointer, int);

typedef _idealFacingMode_C = Void Function(Pointer, Uint8);
typedef _idealFacingMode_Dart = void Function(Pointer, int);

typedef _exactHeight_C = Result Function(Pointer, Int64);
typedef _exactHeight_Dart = Result Function(Pointer, int);

typedef _idealHeight_C = Result Function(Pointer, Int64);
typedef _idealHeight_Dart = Result Function(Pointer, int);

typedef _heightInRange_C = Result Function(Pointer, Int64, Int64);
typedef _heightInRange_Dart = Result Function(Pointer, int, int);

typedef _exactWidth_C = Result Function(Pointer, Int64);
typedef _exactWidth_Dart = Result Function(Pointer, int);

typedef _idealWidth_C = Result Function(Pointer, Int64);
typedef _idealWidth_Dart = Result Function(Pointer, int);

typedef _widthInRange_C = Result Function(Pointer, Int64, Int64);
typedef _widthInRange_Dart = Result Function(Pointer, int, int);

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

final _new =
    dl.lookupFunction<_new_C, _new_Dart>('DeviceVideoTrackConstraints__new');

final _deviceId = dl.lookupFunction<_deviceId_C, _deviceId_Dart>(
    'DeviceVideoTrackConstraints__device_id');

final _exactFacingMode =
    dl.lookupFunction<_exactFacingMode_C, _exactFacingMode_Dart>(
        'DeviceVideoTrackConstraints__exact_facing_mode');

final _idealFacingMode =
    dl.lookupFunction<_idealFacingMode_C, _idealFacingMode_Dart>(
        'DeviceVideoTrackConstraints__ideal_facing_mode');

final _exactHeight = dl.lookupFunction<_exactHeight_C, _exactHeight_Dart>(
    'DeviceVideoTrackConstraints__exact_height');

final _idealHeight = dl.lookupFunction<_idealHeight_C, _idealHeight_Dart>(
    'DeviceVideoTrackConstraints__ideal_height');

final _heightInRange = dl.lookupFunction<_heightInRange_C, _heightInRange_Dart>(
    'DeviceVideoTrackConstraints__height_in_range');

final _exactWidth = dl.lookupFunction<_exactWidth_C, _exactWidth_Dart>(
    'DeviceVideoTrackConstraints__exact_width');

final _idealWidth = dl.lookupFunction<_idealWidth_C, _idealWidth_Dart>(
    'DeviceVideoTrackConstraints__ideal_width');

final _widthInRange = dl.lookupFunction<_widthInRange_C, _widthInRange_Dart>(
    'DeviceVideoTrackConstraints__width_in_range');

final _free =
    dl.lookupFunction<_free_C, _free_Dart>('DeviceVideoTrackConstraints__free');

class DeviceVideoTrackConstraints extends base.DeviceVideoTrackConstraints {
  /// [Pointer] to the Rust struct backing this object.
  final NullablePointer ptr = NullablePointer(_new());

  DeviceVideoTrackConstraints() {
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

  @override
  void exactFacingMode(base.FacingMode facingMode) {
    _exactFacingMode(ptr.getInnerPtr(), facingMode.index);
  }

  @override
  void idealFacingMode(base.FacingMode facingMode) {
    _idealFacingMode(ptr.getInnerPtr(), facingMode.index);
  }

  @override
  void exactHeight(int height) {
    _exactHeight(ptr.getInnerPtr(), height).unwrap();
  }

  @override
  void idealHeight(int height) {
    _idealHeight(ptr.getInnerPtr(), height).unwrap();
  }

  @override
  void heightInRange(int min, int max) {
    _heightInRange(ptr.getInnerPtr(), min, max).unwrap();
  }

  @override
  void exactWidth(int width) {
    _exactWidth(ptr.getInnerPtr(), width).unwrap();
  }

  @override
  void idealWidth(int width) {
    _idealWidth(ptr.getInnerPtr(), width).unwrap();
  }

  @override
  void widthInRange(int min, int max) {
    _widthInRange(ptr.getInnerPtr(), min, max).unwrap();
  }

  @moveSemantics
  @override
  void free() {
    _free(ptr.getInnerPtr());
    ptr.free();
  }
}
