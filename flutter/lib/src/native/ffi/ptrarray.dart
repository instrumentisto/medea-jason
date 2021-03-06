import 'dart:ffi';

import '../../util/move_semantic.dart';
import '../jason.dart';

typedef _free_C = Void Function(Pointer<PtrArray>);
typedef _free_Dart = void Function(Pointer<PtrArray>);

/// Frees [PtrArray] returned from Rust.
final _free_Dart _free =
    dl.lookupFunction<_free_C, _free_Dart>('PtrArray_free');

/// Array of [Pointer]s to Rust objects.
class PtrArray extends Struct {
  /// [Pointer] to the first array element.
  external Pointer<Pointer> _ptr;

  /// Length of this [PtrArray].
  @Uint64()
  external int _len;
}

extension PtrArrayToList on Pointer<PtrArray> {
  /// Converts this [PtrArray] to a Dart's [List] of [Pointer]s.
  @moveSemantics
  List<Pointer> intoPointerList() {
    try {
      var out = List<Pointer>.empty(growable: true);
      for (var i = 0; i < ref._len; i++) {
        // ignore: omit_local_variable_types
        Pointer<Pointer> el = ref._ptr.elementAt(i);
        out.add(el.value);
      }
      return out;
    } finally {
      _free(this);
    }
  }
}
