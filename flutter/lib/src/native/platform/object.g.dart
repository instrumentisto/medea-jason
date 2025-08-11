import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Pointer<Utf8> Function(Object)? _runtimeType;
Pointer<Utf8> Function(Object)? _toString;

_ErrorSetterFnDart? _handle__runtime_type__set_error;
_ErrorSetterFnDart? _handle__to_string__set_error;

void registerFunction(DynamicLibrary dl, {
required Pointer<Utf8>  Function(Object) runtimeType,
required Pointer<Utf8>  Function(Object) toString,
} ) {
_runtimeType = runtimeType;
_toString = toString;

_handle__runtime_type__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('handle__runtime_type__set_error');
_handle__to_string__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('handle__to_string__set_error');

Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> runtimeType_native = Pointer.fromFunction(_runtimeTypeProxy,);
Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> toString_native = Pointer.fromFunction(_toStringProxy,);

dl.lookupFunction<Void Function(Pointer, Pointer), void Function(Pointer, Pointer)>('register_handle')(

runtimeType_native,
toString_native,
);}
Pointer<Utf8> _runtimeTypeProxy(Object arg0) {try {
                        return _runtimeType!(arg0); } catch (e) { _handle__runtime_type__set_error!(e); return Pointer.fromAddress(0);
                     } }
Pointer<Utf8> _toStringProxy(Object arg0) {try {
                        return _toString!(arg0); } catch (e) { _handle__to_string__set_error!(e); return Pointer.fromAddress(0);
                     } }
