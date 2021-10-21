import 'dart:ffi';

import '../../ffi/foreign_value.dart';
import '../../jason.dart';

typedef _futureResolve_C = Void Function(Pointer, ForeignValue);
typedef _futureResolve_Dart = void Function(Pointer, ForeignValue);
final _futureResolve = dl.lookupFunction<_futureResolve_C, _futureResolve_Dart>(
    'DartFutureResolver__resolve');

typedef _fallibleResolveOk_C = Void Function(Pointer, ForeignValue);
typedef _fallibleResolveOk_Dart = void Function(Pointer, ForeignValue);
final _fallibleResolveOk =
    dl.lookupFunction<_fallibleResolveOk_C, _fallibleResolveOk_Dart>(
        'FallibleDartFutureResolver__resolve_ok');

typedef _fallibleResolveErr_C = Void Function(Pointer, Handle);
typedef _fallibleResolveErr_Dart = void Function(Pointer, Object);
final _fallibleResolveErr =
    dl.lookupFunction<_fallibleResolveErr_C, _fallibleResolveErr_Dart>(
        'FallibleDartFutureResolver__resolve_err');

/// Registers functions needed for `DartFutureResolver` working.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_DartFutureResolver__spawner')(
      Pointer.fromFunction<Void Function(Handle, Pointer)>(resolver));
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_FallibleDartFutureResolver__spawner')(
      Pointer.fromFunction<Void Function(Handle, Pointer)>(fallibleResolver));
}

/// Returns `Future` which will call provided `DartFutureResolver` on resolve.
void resolver(Object f, Pointer resolver) {
  f as Function;
  Future fut = f();
  fut.then((val) {
    var arg = ForeignValue.fromDart(val);
    _futureResolve(resolver, arg.ref);
    arg.free();
  });
}

/// Returns `Future` which will call provided `FallibleDartFutureResolver` on resolve.
void fallibleResolver(Object f, Pointer resolver) {
  f as Function;
  Future fut = f();
  fut.then((val) {
    var arg = ForeignValue.fromDart(val);
    _fallibleResolveOk(resolver, arg.ref);
    arg.free();
  }).onError((error, stackTrace) {
    _fallibleResolveErr(resolver, error!);
  });
}
