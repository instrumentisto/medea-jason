import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function(Pointer<Utf8>, bool)? _newSendEncodingParameters;
Pointer<Utf8> Function(Object)? _getRid;
void Function(Object, bool)? _setActive;
void Function(Object, int)? _setMaxBitrate;
void Function(Object, int)? _setScaleResolutionDownBy;
void Function(Object, Pointer<Utf8>)? _setScalabilityMode;

_ErrorSetterFnDart? _send_encoding_parameters__new_send_encoding_parameters__set_error;
_ErrorSetterFnDart? _send_encoding_parameters__get_rid__set_error;
_ErrorSetterFnDart? _send_encoding_parameters__set_active__set_error;
_ErrorSetterFnDart? _send_encoding_parameters__set_max_bitrate__set_error;
_ErrorSetterFnDart? _send_encoding_parameters__set_scale_resolution_down_by__set_error;
_ErrorSetterFnDart? _send_encoding_parameters__set_scalability_mode__set_error;

void registerFunction(DynamicLibrary dl, {
required Object  Function(Pointer<Utf8>, bool) newSendEncodingParameters,
required Pointer<Utf8>  Function(Object) getRid,
required void  Function(Object, bool) setActive,
required void  Function(Object, int) setMaxBitrate,
required void  Function(Object, int) setScaleResolutionDownBy,
required void  Function(Object, Pointer<Utf8>) setScalabilityMode,
} ) {
_newSendEncodingParameters = newSendEncodingParameters;
_getRid = getRid;
_setActive = setActive;
_setMaxBitrate = setMaxBitrate;
_setScaleResolutionDownBy = setScaleResolutionDownBy;
_setScalabilityMode = setScalabilityMode;

_send_encoding_parameters__new_send_encoding_parameters__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('send_encoding_parameters__new_send_encoding_parameters__set_error');
_send_encoding_parameters__get_rid__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('send_encoding_parameters__get_rid__set_error');
_send_encoding_parameters__set_active__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('send_encoding_parameters__set_active__set_error');
_send_encoding_parameters__set_max_bitrate__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('send_encoding_parameters__set_max_bitrate__set_error');
_send_encoding_parameters__set_scale_resolution_down_by__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('send_encoding_parameters__set_scale_resolution_down_by__set_error');
_send_encoding_parameters__set_scalability_mode__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('send_encoding_parameters__set_scalability_mode__set_error');

Pointer<NativeFunction<Handle Function(Pointer<Utf8>, Bool)>> newSendEncodingParameters_native = Pointer.fromFunction(_newSendEncodingParametersProxy,);
Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> getRid_native = Pointer.fromFunction(_getRidProxy,);
Pointer<NativeFunction<Void Function(Handle, Bool)>> setActive_native = Pointer.fromFunction(_setActiveProxy,);
Pointer<NativeFunction<Void Function(Handle, Int64)>> setMaxBitrate_native = Pointer.fromFunction(_setMaxBitrateProxy,);
Pointer<NativeFunction<Void Function(Handle, Int64)>> setScaleResolutionDownBy_native = Pointer.fromFunction(_setScaleResolutionDownByProxy,);
Pointer<NativeFunction<Void Function(Handle, Pointer<Utf8>)>> setScalabilityMode_native = Pointer.fromFunction(_setScalabilityModeProxy,);

dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer), void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer)>('register_send_encoding_parameters')(

newSendEncodingParameters_native,
getRid_native,
setActive_native,
setMaxBitrate_native,
setScaleResolutionDownBy_native,
setScalabilityMode_native,
);}
Object _newSendEncodingParametersProxy(Pointer<Utf8> arg0, bool arg1) {try {
                        return _newSendEncodingParameters!(arg0, arg1); } catch (e) { _send_encoding_parameters__new_send_encoding_parameters__set_error!(e); return 0;
                     } }
Pointer<Utf8> _getRidProxy(Object arg0) {try {
                        return _getRid!(arg0); } catch (e) { _send_encoding_parameters__get_rid__set_error!(e); return Pointer.fromAddress(0);
                     } }
void _setActiveProxy(Object arg0, bool arg1) {try {
                        return _setActive!(arg0, arg1); } catch (e) { _send_encoding_parameters__set_active__set_error!(e); return ;
                     } }
void _setMaxBitrateProxy(Object arg0, int arg1) {try {
                        return _setMaxBitrate!(arg0, arg1); } catch (e) { _send_encoding_parameters__set_max_bitrate__set_error!(e); return ;
                     } }
void _setScaleResolutionDownByProxy(Object arg0, int arg1) {try {
                        return _setScaleResolutionDownBy!(arg0, arg1); } catch (e) { _send_encoding_parameters__set_scale_resolution_down_by__set_error!(e); return ;
                     } }
void _setScalabilityModeProxy(Object arg0, Pointer<Utf8> arg1) {try {
                        return _setScalabilityMode!(arg0, arg1); } catch (e) { _send_encoding_parameters__set_scalability_mode__set_error!(e); return ;
                     } }
