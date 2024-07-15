import 'dart:ffi';

import '../jason.dart';
import 'foreign_value.dart';
import 'future.g.dart' as bridge;

typedef _FutureResolveOkC = Void Function(Pointer, ForeignValue);
typedef _FutureResolveOkDart = void Function(Pointer, ForeignValue);
final _futureResolveOk =
    dl.lookupFunction<_FutureResolveOkC, _FutureResolveOkDart>(
        'FutureFromDart__resolve_ok');

typedef _FutureResolveErrC = Void Function(Pointer, Handle);
typedef _FutureResolveErrDart = void Function(Pointer, Object);
final _futureResolveErr =
    dl.lookupFunction<_FutureResolveErrC, _FutureResolveErrDart>(
        'FutureFromDart__resolve_err');

/// Registers functions required for Rust's `FutureFromDart` to work.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    completeProxy: _completeProxy,
  );
}

/// Registers callbacks on the provided [Future] completing Rust's
/// `FutureFromDart`.
void _completeProxy(Object f, Pointer rustFuture) {
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
