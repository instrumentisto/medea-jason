import 'dart:ffi';

import 'package:ffi/ffi.dart';

import '../../util/move_semantic.dart';
import '../jason.dart';
import 'box_handle.dart';
import 'nullable_pointer.dart';

typedef _BoxForeignValueC = Pointer Function(ForeignValue);
typedef _BoxForeignValueDart = Pointer Function(ForeignValue);

final _BoxForeignValueDart _boxForeignValue =
    dl.lookupFunction<_BoxForeignValueC, _BoxForeignValueDart>(
        'box_foreign_value');

/// Type-erased value that can be transferred via FFI boundaries to/from Rust.
final class ForeignValue extends Struct {
  /// Index of the used [_ForeignValueFields] union field.
  ///
  /// `0` goes for no value.
  @Uint8()
  external int _tag;

  /// Actual [ForeignValue] payload.
  external _ForeignValueFields _payload;

  /// Private constructor.
  ///
  /// This class is a reference backed by a native memory, so it cannot be
  /// instantiated like a normal Dart class.
  ForeignValue._();

  /// Returns Dart representation of the underlying foreign value.
  ///
  /// Returns `null` if underlying value has no value.
  dynamic toDart() {
    switch (_tag) {
      case 0:
        return;
      case 1:
        return _payload.ptr;
      case 2:
        return unboxDartHandle(_payload.handlePtr);
      case 3:
        return _payload.stringPtr.string.toDartString();
      case 4:
        return _payload.number;
      case 5:
        return _payload.float;
      case 6:
        return _payload.boolean;
      default:
        throw TypeError();
    }
  }

  /// Allocates a new [ForeignValue] guessing the provided [val] type.
  static Pointer<ForeignValue> fromDart(Object? val) {
    if (val == null) {
      return ForeignValue.none();
    } else if (val is int) {
      return ForeignValue.fromInt(val);
    } else if (val is double) {
      return ForeignValue.fromDouble(val);
    } else if (val is bool) {
      return ForeignValue.fromBool(val);
    } else if (val is String) {
      return ForeignValue.fromString(val);
    } else if (val is NullablePointer) {
      return ForeignValue.fromPtr(val);
    } else {
      return ForeignValue.fromHandle(val);
    }
  }

  /// Allocates a new [ForeignValue] with no value.
  ///
  /// This can be used when calling native function with an optional argument.
  static Pointer<ForeignValue> none() {
    return calloc<ForeignValue>();
  }

  /// Allocates a new [ForeignValue] with the provided pointer to some Rust
  /// object.
  static Pointer<ForeignValue> fromPtr(NullablePointer ptr) {
    var fVal = calloc<ForeignValue>();
    fVal.ref._tag = 1;
    fVal.ref._payload.ptr = ptr.getInnerPtr();
    return fVal;
  }

  /// Allocates a new [ForeignValue] with the provided [Object] converting it
  /// to a [Handle].
  static Pointer<ForeignValue> fromHandle(Object obj) {
    var fVal = calloc<ForeignValue>();
    fVal.ref._tag = 2;
    fVal.ref._payload.handlePtr = boxDartHandle(obj);
    return fVal;
  }

  /// Allocates a new [ForeignValue] with the provided [String].
  static Pointer<ForeignValue> fromString(String str) {
    var fVal = calloc<ForeignValue>();
    fVal.ref._tag = 3;
    fVal.ref._payload.stringPtr.string = str.toNativeUtf8();
    return fVal;
  }

  /// Allocates a new [ForeignValue] with the provided [int] value.
  static Pointer<ForeignValue> fromInt(int num) {
    var fVal = calloc<ForeignValue>();
    fVal.ref._tag = 4;
    fVal.ref._payload.number = num;
    return fVal;
  }

  static Pointer<ForeignValue> fromDouble(double num) {
    var fVal = calloc<ForeignValue>();
    fVal.ref._tag = 5;
    fVal.ref._payload.float = num;
    return fVal;
  }

  static Pointer<ForeignValue> fromBool(bool boolean) {
    var fVal = calloc<ForeignValue>();
    fVal.ref._tag = 6;
    fVal.ref._payload.boolean = boolean;
    return fVal;
  }
}

extension ForeignValuePointer on Pointer<ForeignValue> {
  /// Transfers [ForeignValue] ownership to Rust.
  ///
  /// Frees Dart side [ForeignValue].
  Pointer intoRustOwned() {
    var out = _boxForeignValue(ref);
    calloc.free(this);
    return out;
  }

  /// Releases the memory allocated on a native heap.
  @moveSemantics
  void free() {
    if (ref._tag == 2) {
      freeBoxedDartHandle(ref._payload.handlePtr);
    }
    calloc.free(this);
  }
}

/// Possible fields of a [ForeignValue].
final class _ForeignValueFields extends Union {
  /// [Pointer] to some Rust object.
  external Pointer ptr;

  /// [Pointer] to a [Handle] to some Dart object.
  external Pointer<Handle> handlePtr;

  /// [Pointer] to a native string.
  external _StringPointer stringPtr;

  /// Integer value.
  @Int64()
  external int number;

  /// Double value.
  @Double()
  external double float;

  /// Bool value.
  @Bool()
  external bool boolean;
}

/// [Pointer] to a native string along with information of its owner.
final class _StringPointer extends Struct {
  /// [Pointer] to the native string.
  external Pointer<Utf8> string;

  /// Indicator of who allocated the native [string].
  ///
  /// `0` if the native string was allocated by `Rust`, and `1` if it was
  /// allocated by `Dart`.
  @Uint8()
  external int memoryOwner;
}
