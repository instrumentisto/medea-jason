import 'dart:ffi';

import '../jason.dart';
import 'callback.g.dart' as bridge;
import 'foreign_value.dart';

typedef _CallbackCallC = Void Function(Pointer, ForeignValue);
typedef _CallbackCallDart = void Function(Pointer, ForeignValue);
typedef _CallbackTwoArgCallC = Void Function(
    Pointer, ForeignValue, ForeignValue);
typedef _CallbackTwoArgCallDart = void Function(
    Pointer, ForeignValue, ForeignValue);

final _callbackCall =
    dl.lookupFunction<_CallbackCallC, _CallbackCallDart>('Callback__call');
final _callbackTwoArgCall =
    dl.lookupFunction<_CallbackTwoArgCallC, _CallbackTwoArgCallDart>(
        'Callback__call_two_arg');

/// Registers the closure callers functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    callProxy: _callProxy,
    callTwoArgProxy: _callTwoArgProxy,
  );
}

/// Returns a closure calling the provided Rust function [Pointer].
Object _callProxy(Pointer cb) {
  return (val) {
    var arg = ForeignValue.fromDart(val);
    _callbackCall(cb, arg.ref);
    arg.free();
  };
}

/// Returns a two args closure calling the provided Rust function [Pointer].
Object _callTwoArgProxy(Pointer cb) {
  return (first, second) {
    var firstArg = ForeignValue.fromDart(first);
    var secondArg = ForeignValue.fromDart(second);
    _callbackTwoArgCall(cb, firstArg.ref, secondArg.ref);
    firstArg.free();
    secondArg.free();
  };
}
