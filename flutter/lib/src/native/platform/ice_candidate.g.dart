import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function(ForeignValue, ForeignValue, ForeignValue)? _init;
Pointer<Utf8> Function(Object)? _candidate;
int Function(Object)? _sdpMLineIndex;
Pointer<Utf8> Function(Object)? _sdpMid;

_ErrorSetterFnDart? _ice_candidate__init__set_error;
_ErrorSetterFnDart? _ice_candidate__candidate__set_error;
_ErrorSetterFnDart? _ice_candidate__sdp_m_line_index__set_error;
_ErrorSetterFnDart? _ice_candidate__sdp_mid__set_error;

void registerFunction(DynamicLibrary dl, {
required Object  Function(ForeignValue, ForeignValue, ForeignValue) init,
required Pointer<Utf8>  Function(Object) candidate,
required int  Function(Object) sdpMLineIndex,
required Pointer<Utf8>  Function(Object) sdpMid,
} ) {
_init = init;
_candidate = candidate;
_sdpMLineIndex = sdpMLineIndex;
_sdpMid = sdpMid;

_ice_candidate__init__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('ice_candidate__init__set_error');
_ice_candidate__candidate__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('ice_candidate__candidate__set_error');
_ice_candidate__sdp_m_line_index__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('ice_candidate__sdp_m_line_index__set_error');
_ice_candidate__sdp_mid__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('ice_candidate__sdp_mid__set_error');

Pointer<NativeFunction<Handle Function(ForeignValue, ForeignValue, ForeignValue)>> init_native = Pointer.fromFunction(_initProxy,);
Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> candidate_native = Pointer.fromFunction(_candidateProxy,);
Pointer<NativeFunction<Uint64 Function(Handle)>> sdpMLineIndex_native = Pointer.fromFunction(_sdpMLineIndexProxy,0);
Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> sdpMid_native = Pointer.fromFunction(_sdpMidProxy,);

dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer), void Function(Pointer, Pointer, Pointer, Pointer)>('register_ice_candidate')(

init_native,
candidate_native,
sdpMLineIndex_native,
sdpMid_native,
);}
Object _initProxy(ForeignValue arg0, ForeignValue arg1, ForeignValue arg2) {try {
                        return _init!(arg0, arg1, arg2); } catch (e) { _ice_candidate__init__set_error!(e); return 0;
                     } }
Pointer<Utf8> _candidateProxy(Object arg0) {try {
                        return _candidate!(arg0); } catch (e) { _ice_candidate__candidate__set_error!(e); return Pointer.fromAddress(0);
                     } }
int _sdpMLineIndexProxy(Object arg0) {try {
                        return _sdpMLineIndex!(arg0); } catch (e) { _ice_candidate__sdp_m_line_index__set_error!(e); return 0;
                     } }
Pointer<Utf8> _sdpMidProxy(Object arg0) {try {
                        return _sdpMid!(arg0); } catch (e) { _ice_candidate__sdp_mid__set_error!(e); return Pointer.fromAddress(0);
                     } }
