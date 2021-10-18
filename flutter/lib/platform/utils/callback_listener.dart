import 'dart:ffi';
import 'package:medea_jason/ffi/foreign_value.dart';

import '../../jason.dart';

typedef _callbackCall_C = Void Function(Pointer, ForeignValue);
typedef _callbackCall_Dart = void Function(Pointer, ForeignValue);
final _callbackCall =
    dl.lookupFunction<_callbackCall_C, _callbackCall_Dart>('Callback__call');

void registerFunctions(DynamicLibrary dl) {
  dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>(
          'register_Callback__callback')(
      Pointer.fromFunction<Handle Function(Pointer)>(callback));
}

Object callback(Pointer caller) {
  return (val) {
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
      throw UnimplementedError("Callback can't process provided type. " +
          val.runtimeType.toString());
    }
    _callbackCall(caller, arg.ref);
  };
}
