import 'dart:ffi';

import '../interface/display_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import '/src/util/rust_handles_storage.dart';
import 'ffi/nullable_pointer.dart';
import 'ffi/result.dart';
import 'jason.dart';

typedef _new_C = Pointer Function();
typedef _new_Dart = Pointer Function();

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

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

typedef _exactFrameRate_C = Result Function(Pointer, Int64);
typedef _exactFrameRate_Dart = Result Function(Pointer, int);

typedef _idealFrameRate_C = Result Function(Pointer, Int64);
typedef _idealFrameRate_Dart = Result Function(Pointer, int);

typedef _widthInRange_C = Result Function(Pointer, Int64, Int64);
typedef _widthInRange_Dart = Result Function(Pointer, int, int);

final _new =
    dl.lookupFunction<_new_C, _new_Dart>('DisplayVideoTrackConstraints__new');

final _exactHeight = dl.lookupFunction<_exactHeight_C, _exactHeight_Dart>(
    'DisplayVideoTrackConstraints__exact_height');

final _idealHeight = dl.lookupFunction<_idealHeight_C, _idealHeight_Dart>(
    'DisplayVideoTrackConstraints__ideal_height');

final _heightInRange = dl.lookupFunction<_heightInRange_C, _heightInRange_Dart>(
    'DisplayVideoTrackConstraints__height_in_range');

final _exactWidth = dl.lookupFunction<_exactWidth_C, _exactWidth_Dart>(
    'DisplayVideoTrackConstraints__exact_width');

final _idealWidth = dl.lookupFunction<_idealWidth_C, _idealWidth_Dart>(
    'DisplayVideoTrackConstraints__ideal_width');

final _exactFrameRate =
    dl.lookupFunction<_exactFrameRate_C, _exactFrameRate_Dart>(
        'DisplayVideoTrackConstraints__exact_frame_rate');

final _idealFrameRate =
    dl.lookupFunction<_idealFrameRate_C, _idealFrameRate_Dart>(
        'DisplayVideoTrackConstraints__ideal_frame_rate');

final _widthInRange = dl.lookupFunction<_widthInRange_C, _widthInRange_Dart>(
    'DisplayVideoTrackConstraints__width_in_range');

final _free_Dart _free = dl
    .lookupFunction<_free_C, _free_Dart>('DisplayVideoTrackConstraints__free');

class DisplayVideoTrackConstraints extends base.DisplayVideoTrackConstraints {
  /// [Pointer] to the Rust struct backing this object.
  final NullablePointer ptr = NullablePointer(_new());

  DisplayVideoTrackConstraints() {
    RustHandlesStorage().insertHandle(this);
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

  @override
  void exactFrameRate(int frameRate) {
    _exactFrameRate(ptr.getInnerPtr(), frameRate).unwrap();
  }

  @override
  void idealFrameRate(int frameRate) {
    _idealFrameRate(ptr.getInnerPtr(), frameRate).unwrap();
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
