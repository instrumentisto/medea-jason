import 'dart:ffi';

import '../jason.dart';
import 'foreign_value.dart';
import 'callback.g.dart' as bridge;

typedef _callbackCall_C = Void Function(Pointer, ForeignValue);
typedef _callbackCall_Dart = void Function(Pointer, ForeignValue);
typedef _callbackTwoArgCall_C = Void Function(
    Pointer, ForeignValue, ForeignValue);
typedef _callbackTwoArgCall_Dart = void Function(
    Pointer, ForeignValue, ForeignValue);

final _callbackCall =
    dl.lookupFunction<_callbackCall_C, _callbackCall_Dart>('Callback__call');
final _callbackTwoArgCall =
    dl.lookupFunction<_callbackTwoArgCall_C, _callbackTwoArgCall_Dart>(
        'Callback__call_two_arg');

/// Registers the closure callers functions in Rust.
void registerFunctions(DynamicLibrary dl) {
  bridge.registerFunction(
    dl,
    callProxy: callback,
    callTwoArgProxy: callbackTwoArg,
  );
}

/// Returns a closure calling the provided Rust function [Pointer].
Object callback(Pointer cb) {
  return (val) {
    var arg = ForeignValue.fromDart(val);
    _callbackCall(cb, arg.ref);
    arg.free();
  };
}

/// Returns a two args closure calling the provided Rust function [Pointer].
Object callbackTwoArg(Pointer cb) {
  return (first, second) {
    var firstArg = ForeignValue.fromDart(first);
    var secondArg = ForeignValue.fromDart(second);
    _callbackTwoArgCall(cb, firstArg.ref, secondArg.ref);
    firstArg.free();
    secondArg.free();
  };
}
