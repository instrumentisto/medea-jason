import 'dart:ffi';

import '../interface/display_video_track_constraints.dart' as base;
import '../util/move_semantic.dart';
import 'ffi/nullable_pointer.dart';
import 'jason.dart';

typedef _new_C = Pointer Function();
typedef _new_Dart = Pointer Function();

typedef _free_C = Void Function(Pointer);
typedef _free_Dart = void Function(Pointer);

final _new =
    dl.lookupFunction<_new_C, _new_Dart>('DisplayVideoTrackConstraints__new');

final _free_Dart _free = dl
    .lookupFunction<_free_C, _free_Dart>('DisplayVideoTrackConstraints__free');

class DisplayVideoTrackConstraints extends base.DisplayVideoTrackConstraints {
  /// [Pointer] to the Rust struct backing this object.
  final NullablePointer ptr = NullablePointer(_new());

  @moveSemantics
  @override
  void free() {
    _free(ptr.getInnerPtr());
    ptr.free();
  }
}
