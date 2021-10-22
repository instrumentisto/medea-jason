import 'dart:ffi';

import '../../ffi/foreign_value.dart';
import '../../jason.dart';

typedef _callbackCall_C = Void Function(Pointer, ForeignValue);
typedef _callbackCall_Dart = void Function(Pointer, ForeignValue);
final _callbackCall =
    dl.lookupFunction<_callbackCall_C, _callbackCall_Dart>('Callback__call');

/// Registers functions required for callback to work.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Callback__callback')(
      Pointer.fromFunction<Handle Function(Pointer)>(callback));
}

/// Returns closure which will call the provided Rust `Callback` when it will
/// be called.
Object callback(Pointer cb) {
  return (val) {
    var arg = ForeignValue.fromDart(val);
    _callbackCall(cb, arg.ref);
    arg.free();
  };
}
