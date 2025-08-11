import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

void Function(Object, Pointer)? _completeProxy;

_ErrorSetterFnDart? _future_from_dart__complete_proxy__set_error;

void registerFunction(DynamicLibrary dl, {
required void  Function(Object, Pointer) completeProxy,
} ) {
_completeProxy = completeProxy;

_future_from_dart__complete_proxy__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('future_from_dart__complete_proxy__set_error');

Pointer<NativeFunction<Void Function(Handle, Pointer)>> completeProxy_native = Pointer.fromFunction(_completeProxyProxy,);

dl.lookupFunction<Void Function(Pointer), void Function(Pointer)>('register_future_from_dart')(

completeProxy_native,
);}
void _completeProxyProxy(Object arg0, Pointer arg1) {try {
                        return _completeProxy!(arg0, arg1); } catch (e) { _future_from_dart__complete_proxy__set_error!(e); return ;
                     } }
