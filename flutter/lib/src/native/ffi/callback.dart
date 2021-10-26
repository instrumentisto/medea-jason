import 'dart:ffi';

import '../jason.dart';
import 'foreign_value.dart';

typedef _callbackCall_C = Void Function(Pointer, ForeignValue);
typedef _callbackCall_Dart = void Function(Pointer, ForeignValue);

final _callbackCall =
    dl.lookupFunction<_callbackCall_C, _callbackCall_Dart>('Callback__call');

/// Registers the closure callers functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_fn_caller')(
      Pointer.fromFunction<Void Function(Handle, ForeignValue)>(_callFn));

  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Callback__call_proxy')(
      Pointer.fromFunction<Handle Function(Pointer)>(callback));
}

/// Function used by Rust to call closures with a single [ForeignValue]
/// argument.
void _callFn(void Function(dynamic) fn, ForeignValue value) {
  var arg = value.toDart();
  if (arg != null) {
    fn(arg);
  } else {
    (fn as void Function())();
  }
}

/// Returns a closure calling the provided Rust function [Pointer].
Object callback(Pointer cb) {
  return (val) {
    var arg = ForeignValue.fromDart(val);
    _callbackCall(cb, arg.ref);
    arg.free();
  };
}
