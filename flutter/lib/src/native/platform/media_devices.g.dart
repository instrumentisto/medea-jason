import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function()? _enumerateDevices;
Object Function()? _enumerateDisplays;
Object Function(Object)? _getUserMedia;
Object Function(Object)? _getDisplayMedia;
Object Function(Pointer<Utf8>)? _setOutputAudioId;
Object Function()? _microphoneVolumeIsAvailable;
Object Function()? _microphoneVolume;
Object Function(int)? _setMicrophoneVolume;
void Function(Object)? _onDeviceChange;
int Function(Object)? _getMediaExceptionKind;

_ErrorSetterFnDart? _media_devices__enumerate_devices__set_error;
_ErrorSetterFnDart? _media_devices__enumerate_displays__set_error;
_ErrorSetterFnDart? _media_devices__get_user_media__set_error;
_ErrorSetterFnDart? _media_devices__get_display_media__set_error;
_ErrorSetterFnDart? _media_devices__set_output_audio_id__set_error;
_ErrorSetterFnDart? _media_devices__microphone_volume_is_available__set_error;
_ErrorSetterFnDart? _media_devices__microphone_volume__set_error;
_ErrorSetterFnDart? _media_devices__set_microphone_volume__set_error;
_ErrorSetterFnDart? _media_devices__on_device_change__set_error;
_ErrorSetterFnDart? _media_devices__get_media_exception_kind__set_error;

void registerFunction(DynamicLibrary dl, {
required Object  Function() enumerateDevices,
required Object  Function() enumerateDisplays,
required Object  Function(Object) getUserMedia,
required Object  Function(Object) getDisplayMedia,
required Object  Function(Pointer<Utf8>) setOutputAudioId,
required Object  Function() microphoneVolumeIsAvailable,
required Object  Function() microphoneVolume,
required Object  Function(int) setMicrophoneVolume,
required void  Function(Object) onDeviceChange,
required int  Function(Object) getMediaExceptionKind,
} ) {
_enumerateDevices = enumerateDevices;
_enumerateDisplays = enumerateDisplays;
_getUserMedia = getUserMedia;
_getDisplayMedia = getDisplayMedia;
_setOutputAudioId = setOutputAudioId;
_microphoneVolumeIsAvailable = microphoneVolumeIsAvailable;
_microphoneVolume = microphoneVolume;
_setMicrophoneVolume = setMicrophoneVolume;
_onDeviceChange = onDeviceChange;
_getMediaExceptionKind = getMediaExceptionKind;

_media_devices__enumerate_devices__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_devices__enumerate_devices__set_error');
_media_devices__enumerate_displays__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_devices__enumerate_displays__set_error');
_media_devices__get_user_media__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_devices__get_user_media__set_error');
_media_devices__get_display_media__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_devices__get_display_media__set_error');
_media_devices__set_output_audio_id__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_devices__set_output_audio_id__set_error');
_media_devices__microphone_volume_is_available__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_devices__microphone_volume_is_available__set_error');
_media_devices__microphone_volume__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_devices__microphone_volume__set_error');
_media_devices__set_microphone_volume__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_devices__set_microphone_volume__set_error');
_media_devices__on_device_change__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_devices__on_device_change__set_error');
_media_devices__get_media_exception_kind__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('media_devices__get_media_exception_kind__set_error');

Pointer<NativeFunction<Handle Function()>> enumerateDevices_native = Pointer.fromFunction(_enumerateDevicesProxy,);
Pointer<NativeFunction<Handle Function()>> enumerateDisplays_native = Pointer.fromFunction(_enumerateDisplaysProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> getUserMedia_native = Pointer.fromFunction(_getUserMediaProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> getDisplayMedia_native = Pointer.fromFunction(_getDisplayMediaProxy,);
Pointer<NativeFunction<Handle Function(Pointer<Utf8>)>> setOutputAudioId_native = Pointer.fromFunction(_setOutputAudioIdProxy,);
Pointer<NativeFunction<Handle Function()>> microphoneVolumeIsAvailable_native = Pointer.fromFunction(_microphoneVolumeIsAvailableProxy,);
Pointer<NativeFunction<Handle Function()>> microphoneVolume_native = Pointer.fromFunction(_microphoneVolumeProxy,);
Pointer<NativeFunction<Handle Function(Int64)>> setMicrophoneVolume_native = Pointer.fromFunction(_setMicrophoneVolumeProxy,);
Pointer<NativeFunction<Void Function(Handle)>> onDeviceChange_native = Pointer.fromFunction(_onDeviceChangeProxy,);
Pointer<NativeFunction<Int64 Function(Handle)>> getMediaExceptionKind_native = Pointer.fromFunction(_getMediaExceptionKindProxy,0);

dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer), void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer)>('register_media_devices')(

enumerateDevices_native,
enumerateDisplays_native,
getUserMedia_native,
getDisplayMedia_native,
setOutputAudioId_native,
microphoneVolumeIsAvailable_native,
microphoneVolume_native,
setMicrophoneVolume_native,
onDeviceChange_native,
getMediaExceptionKind_native,
);}
Object _enumerateDevicesProxy() {try {
                        return _enumerateDevices!(); } catch (e) { _media_devices__enumerate_devices__set_error!(e); return 0;
                     } }
Object _enumerateDisplaysProxy() {try {
                        return _enumerateDisplays!(); } catch (e) { _media_devices__enumerate_displays__set_error!(e); return 0;
                     } }
Object _getUserMediaProxy(Object arg0) {try {
                        return _getUserMedia!(arg0); } catch (e) { _media_devices__get_user_media__set_error!(e); return 0;
                     } }
Object _getDisplayMediaProxy(Object arg0) {try {
                        return _getDisplayMedia!(arg0); } catch (e) { _media_devices__get_display_media__set_error!(e); return 0;
                     } }
Object _setOutputAudioIdProxy(Pointer<Utf8> arg0) {try {
                        return _setOutputAudioId!(arg0); } catch (e) { _media_devices__set_output_audio_id__set_error!(e); return 0;
                     } }
Object _microphoneVolumeIsAvailableProxy() {try {
                        return _microphoneVolumeIsAvailable!(); } catch (e) { _media_devices__microphone_volume_is_available__set_error!(e); return 0;
                     } }
Object _microphoneVolumeProxy() {try {
                        return _microphoneVolume!(); } catch (e) { _media_devices__microphone_volume__set_error!(e); return 0;
                     } }
Object _setMicrophoneVolumeProxy(int arg0) {try {
                        return _setMicrophoneVolume!(arg0); } catch (e) { _media_devices__set_microphone_volume__set_error!(e); return 0;
                     } }
void _onDeviceChangeProxy(Object arg0) {try {
                        return _onDeviceChange!(arg0); } catch (e) { _media_devices__on_device_change__set_error!(e); return ;
                     } }
int _getMediaExceptionKindProxy(Object arg0) {try {
                        return _getMediaExceptionKind!(arg0); } catch (e) { _media_devices__get_media_exception_kind__set_error!(e); return 0;
                     } }
