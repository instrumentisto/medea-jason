import 'dart:ffi';

import '../jason.dart';

typedef _UnboxDartHandleC = Handle Function(Pointer<Handle>);
typedef _UnboxDartHandleDart = Object Function(Pointer<Handle>);
typedef _BoxDartHandleC = Pointer<Handle> Function(Handle);
typedef _BoxDartHandleDart = Pointer<Handle> Function(Object);
typedef _FreeBoxedDartHandleC = Void Function(Pointer<Handle>);
typedef _FreeBoxedDartHandleDart = void Function(Pointer<Handle>);

final _boxDartHandle = dl.lookupFunction<_BoxDartHandleC, _BoxDartHandleDart>(
  'box_dart_handle',
);
final _unboxDartHandle =
    dl.lookupFunction<_UnboxDartHandleC, _UnboxDartHandleDart>(
  'unbox_dart_handle',
);
final _freeBoxedDartHandle =
    dl.lookupFunction<_FreeBoxedDartHandleC, _FreeBoxedDartHandleDart>(
  'free_boxed_dart_handle',
);

/// Converts a [`Pointer<Handle>`] to an [Object] using a Rust trampoline.
Object unboxDartHandle(Pointer<Handle> ptr) {
  return _unboxDartHandle(ptr);
}

/// Frees the provided [`Pointer<Handle`].
void freeBoxedDartHandle(Pointer<Handle> ptr) {
  _freeBoxedDartHandle(ptr);
}

/// Converts an [Object] into a [`Pointer<Handle>`] using a Rust trampoline.
Pointer<Handle> boxDartHandle(Object ptr) {
  return _boxDartHandle(ptr);
}
