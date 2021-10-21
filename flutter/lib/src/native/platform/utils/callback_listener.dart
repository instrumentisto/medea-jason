import 'dart:ffi';

import '../../ffi/foreign_value.dart';
import '../../jason.dart';

typedef _callbackCall_C = Void Function(Pointer, ForeignValue);
typedef _callbackCall_Dart = void Function(Pointer, ForeignValue);
final _callbackCall =
    dl.lookupFunction<_callbackCall_C, _callbackCall_Dart>('Callback__call');

/// Registers functions needed for callback working.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Callback__callback')(
      Pointer.fromFunction<Handle Function(Pointer)>(callback));
}

/// Returns closure which will call provided `Callback` when it will be called.
Object callback(Pointer caller) {
  return (val) {
    var arg = ForeignValue.fromDart(val);
    _callbackCall(caller, arg.ref);
    arg.free();
  };
}
