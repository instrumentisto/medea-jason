import 'dart:ffi';

import '../jason.dart';
import 'foreign_value.dart';
import 'future.g.dart' as bridge;

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
  bridge.registerFunction(
    dl,
    completeProxy: Pointer.fromFunction(_completeProxy),
  );
}

/// Registers callbacks on the provided [Future] completing Rust's
/// `FutureFromDart`.
void _completeProxy(Function f, Pointer rustFuture) {
  Future fut = f();
  fut.then((val) {
    var arg = ForeignValue.fromDart(val);
    _futureResolveOk(rustFuture, arg.ref);
  }).onError((error, stackTrace) {
    _futureResolveErr(rustFuture, error!);
  });
}
