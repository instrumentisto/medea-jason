import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Pointer<Utf8> Function(Object)? _id;
Pointer<Utf8> Function(Object)? _deviceId;
int Function(Object)? _kind;
Pointer Function(Object)? _facingMode;
Pointer Function(Object)? _height;
Pointer Function(Object)? _width;
bool Function(Object)? _enabled;
void Function(Object, bool)? _setEnabled;
Object Function(Object)? _readyState;
Object Function(Object)? _stop;
void Function(Object, Object)? _onEnded;
Object Function(Object)? _clone;
Object Function(Object)? _dispose;
bool Function(Object)? _isOnAudioLevelAvailable;
void Function(Object, Object)? _onAudioLevelChanged;
bool Function(Object)? _isAudioProcessingAvailable;
Object Function(Object, bool)? _setNoiseSuppressionEnabled;
Object Function(Object, int)? _setNoiseSuppressionLevel;
Object Function(Object, bool)? _setEchoCancellationEnabled;
Object Function(Object, bool)? _setAutoGainControlEnabled;
Object Function(Object, bool)? _setHighPassFilterEnabled;
Object Function(Object)? _isNoiseSuppressionEnabled;
Object Function(Object)? _getNoiseSuppressionLevel;
Object Function(Object)? _isAutoGainControlEnabled;
Object Function(Object)? _isEchoCancellationEnabled;
Object Function(Object)? _isHighPassFilterEnabled;

_ErrorSetterFnDart? _media_stream_track__id__set_error;
_ErrorSetterFnDart? _media_stream_track__device_id__set_error;
_ErrorSetterFnDart? _media_stream_track__kind__set_error;
_ErrorSetterFnDart? _media_stream_track__facing_mode__set_error;
_ErrorSetterFnDart? _media_stream_track__height__set_error;
_ErrorSetterFnDart? _media_stream_track__width__set_error;
_ErrorSetterFnDart? _media_stream_track__enabled__set_error;
_ErrorSetterFnDart? _media_stream_track__set_enabled__set_error;
_ErrorSetterFnDart? _media_stream_track__ready_state__set_error;
_ErrorSetterFnDart? _media_stream_track__stop__set_error;
_ErrorSetterFnDart? _media_stream_track__on_ended__set_error;
_ErrorSetterFnDart? _media_stream_track__clone__set_error;
_ErrorSetterFnDart? _media_stream_track__dispose__set_error;
_ErrorSetterFnDart? _media_stream_track__is_on_audio_level_available__set_error;
_ErrorSetterFnDart? _media_stream_track__on_audio_level_changed__set_error;
_ErrorSetterFnDart? _media_stream_track__is_audio_processing_available__set_error;
_ErrorSetterFnDart? _media_stream_track__set_noise_suppression_enabled__set_error;
_ErrorSetterFnDart? _media_stream_track__set_noise_suppression_level__set_error;
_ErrorSetterFnDart? _media_stream_track__set_echo_cancellation_enabled__set_error;
_ErrorSetterFnDart? _media_stream_track__set_auto_gain_control_enabled__set_error;
_ErrorSetterFnDart? _media_stream_track__set_high_pass_filter_enabled__set_error;
_ErrorSetterFnDart? _media_stream_track__is_noise_suppression_enabled__set_error;
_ErrorSetterFnDart? _media_stream_track__get_noise_suppression_level__set_error;
_ErrorSetterFnDart? _media_stream_track__is_auto_gain_control_enabled__set_error;
_ErrorSetterFnDart? _media_stream_track__is_echo_cancellation_enabled__set_error;
_ErrorSetterFnDart? _media_stream_track__is_high_pass_filter_enabled__set_error;

void registerFunction(DynamicLibrary dl, {
required Pointer<Utf8>  Function(Object) id,
required Pointer<Utf8>  Function(Object) deviceId,
required int  Function(Object) kind,
required Pointer  Function(Object) facingMode,
required Pointer  Function(Object) height,
required Pointer  Function(Object) width,
required bool  Function(Object) enabled,
required void  Function(Object, bool) setEnabled,
required Object  Function(Object) readyState,
required Object  Function(Object) stop,
required void  Function(Object, Object) onEnded,
required Object  Function(Object) clone,
required Object  Function(Object) dispose,
required bool  Function(Object) isOnAudioLevelAvailable,
required void  Function(Object, Object) onAudioLevelChanged,
required bool  Function(Object) isAudioProcessingAvailable,
required Object  Function(Object, bool) setNoiseSuppressionEnabled,
required Object  Function(Object, int) setNoiseSuppressionLevel,
required Object  Function(Object, bool) setEchoCancellationEnabled,
required Object  Function(Object, bool) setAutoGainControlEnabled,
required Object  Function(Object, bool) setHighPassFilterEnabled,
required Object  Function(Object) isNoiseSuppressionEnabled,
required Object  Function(Object) getNoiseSuppressionLevel,
required Object  Function(Object) isAutoGainControlEnabled,
required Object  Function(Object) isEchoCancellationEnabled,
required Object  Function(Object) isHighPassFilterEnabled,
} ) {
_id = id;
_deviceId = deviceId;
_kind = kind;
_facingMode = facingMode;
_height = height;
_width = width;
_enabled = enabled;
_setEnabled = setEnabled;
_readyState = readyState;
_stop = stop;
_onEnded = onEnded;
_clone = clone;
_dispose = dispose;
_isOnAudioLevelAvailable = isOnAudioLevelAvailable;
_onAudioLevelChanged = onAudioLevelChanged;
_isAudioProcessingAvailable = isAudioProcessingAvailable;
_setNoiseSuppressionEnabled = setNoiseSuppressionEnabled;
_setNoiseSuppressionLevel = setNoiseSuppressionLevel;
_setEchoCancellationEnabled = setEchoCancellationEnabled;
_setAutoGainControlEnabled = setAutoGainControlEnabled;
_setHighPassFilterEnabled = setHighPassFilterEnabled;
_isNoiseSuppressionEnabled = isNoiseSuppressionEnabled;
_getNoiseSuppressionLevel = getNoiseSuppressionLevel;
_isAutoGainControlEnabled = isAutoGainControlEnabled;
_isEchoCancellationEnabled = isEchoCancellationEnabled;
_isHighPassFilterEnabled = isHighPassFilterEnabled;

_media_stream_track__id__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__id__set_error');
_media_stream_track__device_id__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__device_id__set_error');
_media_stream_track__kind__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__kind__set_error');
_media_stream_track__facing_mode__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__facing_mode__set_error');
_media_stream_track__height__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__height__set_error');
_media_stream_track__width__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__width__set_error');
_media_stream_track__enabled__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__enabled__set_error');
_media_stream_track__set_enabled__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__set_enabled__set_error');
_media_stream_track__ready_state__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__ready_state__set_error');
_media_stream_track__stop__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__stop__set_error');
_media_stream_track__on_ended__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__on_ended__set_error');
_media_stream_track__clone__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__clone__set_error');
_media_stream_track__dispose__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__dispose__set_error');
_media_stream_track__is_on_audio_level_available__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__is_on_audio_level_available__set_error');
_media_stream_track__on_audio_level_changed__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__on_audio_level_changed__set_error');
_media_stream_track__is_audio_processing_available__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__is_audio_processing_available__set_error');
_media_stream_track__set_noise_suppression_enabled__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__set_noise_suppression_enabled__set_error');
_media_stream_track__set_noise_suppression_level__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__set_noise_suppression_level__set_error');
_media_stream_track__set_echo_cancellation_enabled__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__set_echo_cancellation_enabled__set_error');
_media_stream_track__set_auto_gain_control_enabled__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__set_auto_gain_control_enabled__set_error');
_media_stream_track__set_high_pass_filter_enabled__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__set_high_pass_filter_enabled__set_error');
_media_stream_track__is_noise_suppression_enabled__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__is_noise_suppression_enabled__set_error');
_media_stream_track__get_noise_suppression_level__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__get_noise_suppression_level__set_error');
_media_stream_track__is_auto_gain_control_enabled__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__is_auto_gain_control_enabled__set_error');
_media_stream_track__is_echo_cancellation_enabled__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__is_echo_cancellation_enabled__set_error');
_media_stream_track__is_high_pass_filter_enabled__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_stream_track__is_high_pass_filter_enabled__set_error');

Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> id_native = Pointer.fromFunction(_idProxy,);
Pointer<NativeFunction<Pointer<Utf8> Function(Handle)>> deviceId_native = Pointer.fromFunction(_deviceIdProxy,);
Pointer<NativeFunction<Int64 Function(Handle)>> kind_native = Pointer.fromFunction(_kindProxy,0);
Pointer<NativeFunction<Pointer Function(Handle)>> facingMode_native = Pointer.fromFunction(_facingModeProxy,);
Pointer<NativeFunction<Pointer Function(Handle)>> height_native = Pointer.fromFunction(_heightProxy,);
Pointer<NativeFunction<Pointer Function(Handle)>> width_native = Pointer.fromFunction(_widthProxy,);
Pointer<NativeFunction<Bool Function(Handle)>> enabled_native = Pointer.fromFunction(_enabledProxy,false);
Pointer<NativeFunction<Void Function(Handle, Bool)>> setEnabled_native = Pointer.fromFunction(_setEnabledProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> readyState_native = Pointer.fromFunction(_readyStateProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> stop_native = Pointer.fromFunction(_stopProxy,);
Pointer<NativeFunction<Void Function(Handle, Handle)>> onEnded_native = Pointer.fromFunction(_onEndedProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> clone_native = Pointer.fromFunction(_cloneProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> dispose_native = Pointer.fromFunction(_disposeProxy,);
Pointer<NativeFunction<Bool Function(Handle)>> isOnAudioLevelAvailable_native = Pointer.fromFunction(_isOnAudioLevelAvailableProxy,false);
Pointer<NativeFunction<Void Function(Handle, Handle)>> onAudioLevelChanged_native = Pointer.fromFunction(_onAudioLevelChangedProxy,);
Pointer<NativeFunction<Bool Function(Handle)>> isAudioProcessingAvailable_native = Pointer.fromFunction(_isAudioProcessingAvailableProxy,false);
Pointer<NativeFunction<Handle Function(Handle, Bool)>> setNoiseSuppressionEnabled_native = Pointer.fromFunction(_setNoiseSuppressionEnabledProxy,);
Pointer<NativeFunction<Handle Function(Handle, Int64)>> setNoiseSuppressionLevel_native = Pointer.fromFunction(_setNoiseSuppressionLevelProxy,);
Pointer<NativeFunction<Handle Function(Handle, Bool)>> setEchoCancellationEnabled_native = Pointer.fromFunction(_setEchoCancellationEnabledProxy,);
Pointer<NativeFunction<Handle Function(Handle, Bool)>> setAutoGainControlEnabled_native = Pointer.fromFunction(_setAutoGainControlEnabledProxy,);
Pointer<NativeFunction<Handle Function(Handle, Bool)>> setHighPassFilterEnabled_native = Pointer.fromFunction(_setHighPassFilterEnabledProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> isNoiseSuppressionEnabled_native = Pointer.fromFunction(_isNoiseSuppressionEnabledProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> getNoiseSuppressionLevel_native = Pointer.fromFunction(_getNoiseSuppressionLevelProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> isAutoGainControlEnabled_native = Pointer.fromFunction(_isAutoGainControlEnabledProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> isEchoCancellationEnabled_native = Pointer.fromFunction(_isEchoCancellationEnabledProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> isHighPassFilterEnabled_native = Pointer.fromFunction(_isHighPassFilterEnabledProxy,);

dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer), void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer)>('register_media_stream_track')(

id_native,
deviceId_native,
kind_native,
facingMode_native,
height_native,
width_native,
enabled_native,
setEnabled_native,
readyState_native,
stop_native,
onEnded_native,
clone_native,
dispose_native,
isOnAudioLevelAvailable_native,
onAudioLevelChanged_native,
isAudioProcessingAvailable_native,
setNoiseSuppressionEnabled_native,
setNoiseSuppressionLevel_native,
setEchoCancellationEnabled_native,
setAutoGainControlEnabled_native,
setHighPassFilterEnabled_native,
isNoiseSuppressionEnabled_native,
getNoiseSuppressionLevel_native,
isAutoGainControlEnabled_native,
isEchoCancellationEnabled_native,
isHighPassFilterEnabled_native,
);}
Pointer<Utf8> _idProxy(Object arg0) {try {
                        return _id!(arg0); } catch (e) { _media_stream_track__id__set_error!(e); return Pointer.fromAddress(0);
                     } }
Pointer<Utf8> _deviceIdProxy(Object arg0) {try {
                        return _deviceId!(arg0); } catch (e) { _media_stream_track__device_id__set_error!(e); return Pointer.fromAddress(0);
                     } }
int _kindProxy(Object arg0) {try {
                        return _kind!(arg0); } catch (e) { _media_stream_track__kind__set_error!(e); return 0;
                     } }
Pointer _facingModeProxy(Object arg0) {try {
                        return _facingMode!(arg0); } catch (e) { _media_stream_track__facing_mode__set_error!(e); return Pointer.fromAddress(0);
                     } }
Pointer _heightProxy(Object arg0) {try {
                        return _height!(arg0); } catch (e) { _media_stream_track__height__set_error!(e); return Pointer.fromAddress(0);
                     } }
Pointer _widthProxy(Object arg0) {try {
                        return _width!(arg0); } catch (e) { _media_stream_track__width__set_error!(e); return Pointer.fromAddress(0);
                     } }
bool _enabledProxy(Object arg0) {try {
                        return _enabled!(arg0); } catch (e) { _media_stream_track__enabled__set_error!(e); return false;
                     } }
void _setEnabledProxy(Object arg0, bool arg1) {try {
                        return _setEnabled!(arg0, arg1); } catch (e) { _media_stream_track__set_enabled__set_error!(e); return ;
                     } }
Object _readyStateProxy(Object arg0) {try {
                        return _readyState!(arg0); } catch (e) { _media_stream_track__ready_state__set_error!(e); return 0;
                     } }
Object _stopProxy(Object arg0) {try {
                        return _stop!(arg0); } catch (e) { _media_stream_track__stop__set_error!(e); return 0;
                     } }
void _onEndedProxy(Object arg0, Object arg1) {try {
                        return _onEnded!(arg0, arg1); } catch (e) { _media_stream_track__on_ended__set_error!(e); return ;
                     } }
Object _cloneProxy(Object arg0) {try {
                        return _clone!(arg0); } catch (e) { _media_stream_track__clone__set_error!(e); return 0;
                     } }
Object _disposeProxy(Object arg0) {try {
                        return _dispose!(arg0); } catch (e) { _media_stream_track__dispose__set_error!(e); return 0;
                     } }
bool _isOnAudioLevelAvailableProxy(Object arg0) {try {
                        return _isOnAudioLevelAvailable!(arg0); } catch (e) { _media_stream_track__is_on_audio_level_available__set_error!(e); return false;
                     } }
void _onAudioLevelChangedProxy(Object arg0, Object arg1) {try {
                        return _onAudioLevelChanged!(arg0, arg1); } catch (e) { _media_stream_track__on_audio_level_changed__set_error!(e); return ;
                     } }
bool _isAudioProcessingAvailableProxy(Object arg0) {try {
                        return _isAudioProcessingAvailable!(arg0); } catch (e) { _media_stream_track__is_audio_processing_available__set_error!(e); return false;
                     } }
Object _setNoiseSuppressionEnabledProxy(Object arg0, bool arg1) {try {
                        return _setNoiseSuppressionEnabled!(arg0, arg1); } catch (e) { _media_stream_track__set_noise_suppression_enabled__set_error!(e); return 0;
                     } }
Object _setNoiseSuppressionLevelProxy(Object arg0, int arg1) {try {
                        return _setNoiseSuppressionLevel!(arg0, arg1); } catch (e) { _media_stream_track__set_noise_suppression_level__set_error!(e); return 0;
                     } }
Object _setEchoCancellationEnabledProxy(Object arg0, bool arg1) {try {
                        return _setEchoCancellationEnabled!(arg0, arg1); } catch (e) { _media_stream_track__set_echo_cancellation_enabled__set_error!(e); return 0;
                     } }
Object _setAutoGainControlEnabledProxy(Object arg0, bool arg1) {try {
                        return _setAutoGainControlEnabled!(arg0, arg1); } catch (e) { _media_stream_track__set_auto_gain_control_enabled__set_error!(e); return 0;
                     } }
Object _setHighPassFilterEnabledProxy(Object arg0, bool arg1) {try {
                        return _setHighPassFilterEnabled!(arg0, arg1); } catch (e) { _media_stream_track__set_high_pass_filter_enabled__set_error!(e); return 0;
                     } }
Object _isNoiseSuppressionEnabledProxy(Object arg0) {try {
                        return _isNoiseSuppressionEnabled!(arg0); } catch (e) { _media_stream_track__is_noise_suppression_enabled__set_error!(e); return 0;
                     } }
Object _getNoiseSuppressionLevelProxy(Object arg0) {try {
                        return _getNoiseSuppressionLevel!(arg0); } catch (e) { _media_stream_track__get_noise_suppression_level__set_error!(e); return 0;
                     } }
Object _isAutoGainControlEnabledProxy(Object arg0) {try {
                        return _isAutoGainControlEnabled!(arg0); } catch (e) { _media_stream_track__is_auto_gain_control_enabled__set_error!(e); return 0;
                     } }
Object _isEchoCancellationEnabledProxy(Object arg0) {try {
                        return _isEchoCancellationEnabled!(arg0); } catch (e) { _media_stream_track__is_echo_cancellation_enabled__set_error!(e); return 0;
                     } }
Object _isHighPassFilterEnabledProxy(Object arg0) {try {
                        return _isHighPassFilterEnabled!(arg0); } catch (e) { _media_stream_track__is_high_pass_filter_enabled__set_error!(e); return 0;
                     } }
