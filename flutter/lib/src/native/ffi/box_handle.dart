import 'dart:ffi';

import '../jason.dart';

typedef _unboxDartHandle_C = Handle Function(Pointer<Handle>);
typedef _unboxDartHandle_Dart = Object Function(Pointer<Handle>);
typedef _boxDartHandle_C = Pointer<Handle> Function(Handle);
typedef _boxDartHandle_Dart = Pointer<Handle> Function(Object);
typedef _freeBoxedDartHandle_C = Handle Function(Pointer<Handle>);
typedef _freeBoxedDartHandle_Dart = Object Function(Pointer<Handle>);

final _boxDartHandle =
    dl.lookupFunction<_boxDartHandle_C, _boxDartHandle_Dart>('box_dart_handle');
final _unboxDartHandle =
    dl.lookupFunction<_unboxDartHandle_C, _unboxDartHandle_Dart>(
        'unbox_dart_handle');
final _freeBoxedDartHandle =
    dl.lookupFunction<_freeBoxedDartHandle_C, _freeBoxedDartHandle_Dart>(
        'free_boxed_dart_handle');

/// Converts a [`Pointer<Handle>`] to an [Object] using a Rust trampoline.
Object unboxDartHandle(Pointer<Handle> ptr) {
  return _unboxDartHandle(ptr);
}

/// Frees provided [`Pointer<Handle`].
void freeBoxedDartHandle(Pointer<Handle> ptr) {
  _freeBoxedDartHandle(ptr);
}

/// Converts an [Object] into a [`Pointer<Handle>`] using a Rust trampoline.
Pointer<Handle> boxDartHandle(Object ptr) {
  return _boxDartHandle(ptr);
}
