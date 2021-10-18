import 'dart:ffi';
import 'package:medea_jason/ffi/foreign_value.dart';

import '../../jason.dart';

typedef _futureResolve_C = Void Function(Pointer, ForeignValue);
typedef _futureResolve_Dart = void Function(Pointer, ForeignValue);
final _futureResolve = dl.lookupFunction<_futureResolve_C, _futureResolve_Dart>(
    'DartFutureResolver__resolve');

void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_DartFutureResolver__spawner')(
      Pointer.fromFunction<Void Function(Handle, Pointer)>(resolver));
}

void resolver(Object fut, Pointer resolver) {
  fut as Future;
  fut.then((val) {
    Pointer<ForeignValue> arg;
    if (val == null) {
      arg = ForeignValue.none();
    } else if (val is int) {
      arg = ForeignValue.fromInt(val);
    } else if (val is String) {
      arg = ForeignValue.fromString(val);
    } else if (val is Object) {
      arg = ForeignValue.fromHandle(val);
    } else {
      throw UnimplementedError(
          "Future can't process provided type. " + val.runtimeType.toString());
    }
    _futureResolve(resolver, arg.ref);
  });
}
