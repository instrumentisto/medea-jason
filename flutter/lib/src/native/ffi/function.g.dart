import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

void Function(Object, ForeignValue)? _caller;

_ErrorSetterFnDart? _function__caller__set_error;

void registerFunction(DynamicLibrary dl, {
required void  Function(Object, ForeignValue) caller,
} ) {
_caller = caller;

_function__caller__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('function__caller__set_error');

Pointer<NativeFunction<Void Function(Handle, ForeignValue)>> caller_native = Pointer.fromFunction(_callerProxy,);

dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>('register_function')(

caller_native,
);}
void _callerProxy(Object arg0, ForeignValue arg1) {try {
                        return _caller!(arg0, arg1); } catch (e) { _function__caller__set_error!(e); return ;
                     } }
