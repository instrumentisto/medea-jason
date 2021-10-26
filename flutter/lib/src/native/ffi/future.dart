import 'dart:ffi';

import 'foreign_value.dart';
import '../jason.dart';

typedef _futureResolveOk_C = Void Function(Pointer, ForeignValue);
typedef _futureResolveOk_Dart = void Function(Pointer, ForeignValue);
final _futureResolveOk =
    dl.lookupFunction<_futureResolveOk_C, _futureResolveOk_Dart>(
        'FutureFromDart__resolve_ok');

typedef _futureResolveErr_C = Void Function(Pointer, Handle);
typedef _futureResolveErr_Dart = void Function(Pointer, Object);
final _futureResolveErr =
    dl.lookupFunction<_futureResolveErr_C, _futureResolveErr_Dart>(
        'FutureFromDart__resolve_err');

/// Registers functions required for Rust's `FutureFromDart` to work.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_FutureFromDart__complete_proxy')(
      Pointer.fromFunction<Void Function(Handle, Pointer)>(completeProxy));
}

/// Registers callbacks on the provided [Future] completing Rust's
/// `FutureFromDart`.
void completeProxy(Object f, Pointer rustFuture) {
  f as Function;
  Future fut = f();
  fut.then((val) {
    var arg = ForeignValue.fromDart(val);
    _futureResolveOk(rustFuture, arg.ref);
    arg.free();
  }).onError((error, stackTrace) {
    _futureResolveErr(rustFuture, error!);
  });
}
