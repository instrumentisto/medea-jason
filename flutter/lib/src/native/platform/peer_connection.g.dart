import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

int Function(Object)? _iceConnectionState;
void Function(Object, Object)? _onConnectionStateChange;
int Function(Object)? _connectionState;
void Function(Object)? _restartIce;
Object Function(Object)? _rollback;
Object Function(Object)? _getStats;
void Function(Object, Object)? _onTrack;
void Function(Object, Object)? _onIceCandidate;
void Function(Object, Object)? _onIceCandidateError;
Object Function(Object, Pointer<Utf8>)? _getTransceiverByMid;
Object Function(Object, Object)? _addIceCandidate;
void Function(Object, Object)? _onIceConnectionStateChange;
Object Function(Object, bool)? _newPeer;
Object Function(Object, int, Object)? _addTransceiver;
Object Function(Object)? _createOffer;
Object Function(Object)? _createAnswer;
Object Function(Object, Pointer<Utf8>, Pointer<Utf8>)? _setLocalDescription;
Object Function(Object, Pointer<Utf8>, Pointer<Utf8>)? _setRemoteDescription;
void Function(Object)? _close;

_ErrorSetterFnDart? _peer_connection__ice_connection_state__set_error;
_ErrorSetterFnDart? _peer_connection__on_connection_state_change__set_error;
_ErrorSetterFnDart? _peer_connection__connection_state__set_error;
_ErrorSetterFnDart? _peer_connection__restart_ice__set_error;
_ErrorSetterFnDart? _peer_connection__rollback__set_error;
_ErrorSetterFnDart? _peer_connection__get_stats__set_error;
_ErrorSetterFnDart? _peer_connection__on_track__set_error;
_ErrorSetterFnDart? _peer_connection__on_ice_candidate__set_error;
_ErrorSetterFnDart? _peer_connection__on_ice_candidate_error__set_error;
_ErrorSetterFnDart? _peer_connection__get_transceiver_by_mid__set_error;
_ErrorSetterFnDart? _peer_connection__add_ice_candidate__set_error;
_ErrorSetterFnDart? _peer_connection__on_ice_connection_state_change__set_error;
_ErrorSetterFnDart? _peer_connection__new_peer__set_error;
_ErrorSetterFnDart? _peer_connection__add_transceiver__set_error;
_ErrorSetterFnDart? _peer_connection__create_offer__set_error;
_ErrorSetterFnDart? _peer_connection__create_answer__set_error;
_ErrorSetterFnDart? _peer_connection__set_local_description__set_error;
_ErrorSetterFnDart? _peer_connection__set_remote_description__set_error;
_ErrorSetterFnDart? _peer_connection__close__set_error;

void registerFunction(DynamicLibrary dl, {
required int  Function(Object) iceConnectionState,
required void  Function(Object, Object) onConnectionStateChange,
required int  Function(Object) connectionState,
required void  Function(Object) restartIce,
required Object  Function(Object) rollback,
required Object  Function(Object) getStats,
required void  Function(Object, Object) onTrack,
required void  Function(Object, Object) onIceCandidate,
required void  Function(Object, Object) onIceCandidateError,
required Object  Function(Object, Pointer<Utf8>) getTransceiverByMid,
required Object  Function(Object, Object) addIceCandidate,
required void  Function(Object, Object) onIceConnectionStateChange,
required Object  Function(Object, bool) newPeer,
required Object  Function(Object, int, Object) addTransceiver,
required Object  Function(Object) createOffer,
required Object  Function(Object) createAnswer,
required Object  Function(Object, Pointer<Utf8>, Pointer<Utf8>) setLocalDescription,
required Object  Function(Object, Pointer<Utf8>, Pointer<Utf8>) setRemoteDescription,
required void  Function(Object) close,
} ) {
_iceConnectionState = iceConnectionState;
_onConnectionStateChange = onConnectionStateChange;
_connectionState = connectionState;
_restartIce = restartIce;
_rollback = rollback;
_getStats = getStats;
_onTrack = onTrack;
_onIceCandidate = onIceCandidate;
_onIceCandidateError = onIceCandidateError;
_getTransceiverByMid = getTransceiverByMid;
_addIceCandidate = addIceCandidate;
_onIceConnectionStateChange = onIceConnectionStateChange;
_newPeer = newPeer;
_addTransceiver = addTransceiver;
_createOffer = createOffer;
_createAnswer = createAnswer;
_setLocalDescription = setLocalDescription;
_setRemoteDescription = setRemoteDescription;
_close = close;

_peer_connection__ice_connection_state__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__ice_connection_state__set_error');
_peer_connection__on_connection_state_change__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__on_connection_state_change__set_error');
_peer_connection__connection_state__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__connection_state__set_error');
_peer_connection__restart_ice__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__restart_ice__set_error');
_peer_connection__rollback__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__rollback__set_error');
_peer_connection__get_stats__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__get_stats__set_error');
_peer_connection__on_track__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__on_track__set_error');
_peer_connection__on_ice_candidate__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__on_ice_candidate__set_error');
_peer_connection__on_ice_candidate_error__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__on_ice_candidate_error__set_error');
_peer_connection__get_transceiver_by_mid__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__get_transceiver_by_mid__set_error');
_peer_connection__add_ice_candidate__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__add_ice_candidate__set_error');
_peer_connection__on_ice_connection_state_change__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__on_ice_connection_state_change__set_error');
_peer_connection__new_peer__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__new_peer__set_error');
_peer_connection__add_transceiver__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__add_transceiver__set_error');
_peer_connection__create_offer__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__create_offer__set_error');
_peer_connection__create_answer__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__create_answer__set_error');
_peer_connection__set_local_description__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__set_local_description__set_error');
_peer_connection__set_remote_description__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__set_remote_description__set_error');
_peer_connection__close__set_error = dl.lookupFunction<_ErrorSetterFnC,_ErrorSetterFnDart>('peer_connection__close__set_error');

Pointer<NativeFunction<Int32 Function(Handle)>> iceConnectionState_native = Pointer.fromFunction(_iceConnectionStateProxy,0);
Pointer<NativeFunction<Void Function(Handle, Handle)>> onConnectionStateChange_native = Pointer.fromFunction(_onConnectionStateChangeProxy,);
Pointer<NativeFunction<Int32 Function(Handle)>> connectionState_native = Pointer.fromFunction(_connectionStateProxy,0);
Pointer<NativeFunction<Void Function(Handle)>> restartIce_native = Pointer.fromFunction(_restartIceProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> rollback_native = Pointer.fromFunction(_rollbackProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> getStats_native = Pointer.fromFunction(_getStatsProxy,);
Pointer<NativeFunction<Void Function(Handle, Handle)>> onTrack_native = Pointer.fromFunction(_onTrackProxy,);
Pointer<NativeFunction<Void Function(Handle, Handle)>> onIceCandidate_native = Pointer.fromFunction(_onIceCandidateProxy,);
Pointer<NativeFunction<Void Function(Handle, Handle)>> onIceCandidateError_native = Pointer.fromFunction(_onIceCandidateErrorProxy,);
Pointer<NativeFunction<Handle Function(Handle, Pointer<Utf8>)>> getTransceiverByMid_native = Pointer.fromFunction(_getTransceiverByMidProxy,);
Pointer<NativeFunction<Handle Function(Handle, Handle)>> addIceCandidate_native = Pointer.fromFunction(_addIceCandidateProxy,);
Pointer<NativeFunction<Void Function(Handle, Handle)>> onIceConnectionStateChange_native = Pointer.fromFunction(_onIceConnectionStateChangeProxy,);
Pointer<NativeFunction<Handle Function(Handle, Bool)>> newPeer_native = Pointer.fromFunction(_newPeerProxy,);
Pointer<NativeFunction<Handle Function(Handle, Int64, Handle)>> addTransceiver_native = Pointer.fromFunction(_addTransceiverProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> createOffer_native = Pointer.fromFunction(_createOfferProxy,);
Pointer<NativeFunction<Handle Function(Handle)>> createAnswer_native = Pointer.fromFunction(_createAnswerProxy,);
Pointer<NativeFunction<Handle Function(Handle, Pointer<Utf8>, Pointer<Utf8>)>> setLocalDescription_native = Pointer.fromFunction(_setLocalDescriptionProxy,);
Pointer<NativeFunction<Handle Function(Handle, Pointer<Utf8>, Pointer<Utf8>)>> setRemoteDescription_native = Pointer.fromFunction(_setRemoteDescriptionProxy,);
Pointer<NativeFunction<Void Function(Handle)>> close_native = Pointer.fromFunction(_closeProxy,);

dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer), void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer)>('register_peer_connection')(

iceConnectionState_native,
onConnectionStateChange_native,
connectionState_native,
restartIce_native,
rollback_native,
getStats_native,
onTrack_native,
onIceCandidate_native,
onIceCandidateError_native,
getTransceiverByMid_native,
addIceCandidate_native,
onIceConnectionStateChange_native,
newPeer_native,
addTransceiver_native,
createOffer_native,
createAnswer_native,
setLocalDescription_native,
setRemoteDescription_native,
close_native,
);}
int _iceConnectionStateProxy(Object arg0) {try {
                        return _iceConnectionState!(arg0); } catch (e) { _peer_connection__ice_connection_state__set_error!(e); return 0;
                     } }
void _onConnectionStateChangeProxy(Object arg0, Object arg1) {try {
                        return _onConnectionStateChange!(arg0, arg1); } catch (e) { _peer_connection__on_connection_state_change__set_error!(e); return ;
                     } }
int _connectionStateProxy(Object arg0) {try {
                        return _connectionState!(arg0); } catch (e) { _peer_connection__connection_state__set_error!(e); return 0;
                     } }
void _restartIceProxy(Object arg0) {try {
                        return _restartIce!(arg0); } catch (e) { _peer_connection__restart_ice__set_error!(e); return ;
                     } }
Object _rollbackProxy(Object arg0) {try {
                        return _rollback!(arg0); } catch (e) { _peer_connection__rollback__set_error!(e); return 0;
                     } }
Object _getStatsProxy(Object arg0) {try {
                        return _getStats!(arg0); } catch (e) { _peer_connection__get_stats__set_error!(e); return 0;
                     } }
void _onTrackProxy(Object arg0, Object arg1) {try {
                        return _onTrack!(arg0, arg1); } catch (e) { _peer_connection__on_track__set_error!(e); return ;
                     } }
void _onIceCandidateProxy(Object arg0, Object arg1) {try {
                        return _onIceCandidate!(arg0, arg1); } catch (e) { _peer_connection__on_ice_candidate__set_error!(e); return ;
                     } }
void _onIceCandidateErrorProxy(Object arg0, Object arg1) {try {
                        return _onIceCandidateError!(arg0, arg1); } catch (e) { _peer_connection__on_ice_candidate_error__set_error!(e); return ;
                     } }
Object _getTransceiverByMidProxy(Object arg0, Pointer<Utf8> arg1) {try {
                        return _getTransceiverByMid!(arg0, arg1); } catch (e) { _peer_connection__get_transceiver_by_mid__set_error!(e); return 0;
                     } }
Object _addIceCandidateProxy(Object arg0, Object arg1) {try {
                        return _addIceCandidate!(arg0, arg1); } catch (e) { _peer_connection__add_ice_candidate__set_error!(e); return 0;
                     } }
void _onIceConnectionStateChangeProxy(Object arg0, Object arg1) {try {
                        return _onIceConnectionStateChange!(arg0, arg1); } catch (e) { _peer_connection__on_ice_connection_state_change__set_error!(e); return ;
                     } }
Object _newPeerProxy(Object arg0, bool arg1) {try {
                        return _newPeer!(arg0, arg1); } catch (e) { _peer_connection__new_peer__set_error!(e); return 0;
                     } }
Object _addTransceiverProxy(Object arg0, int arg1, Object arg2) {try {
                        return _addTransceiver!(arg0, arg1, arg2); } catch (e) { _peer_connection__add_transceiver__set_error!(e); return 0;
                     } }
Object _createOfferProxy(Object arg0) {try {
                        return _createOffer!(arg0); } catch (e) { _peer_connection__create_offer__set_error!(e); return 0;
                     } }
Object _createAnswerProxy(Object arg0) {try {
                        return _createAnswer!(arg0); } catch (e) { _peer_connection__create_answer__set_error!(e); return 0;
                     } }
Object _setLocalDescriptionProxy(Object arg0, Pointer<Utf8> arg1, Pointer<Utf8> arg2) {try {
                        return _setLocalDescription!(arg0, arg1, arg2); } catch (e) { _peer_connection__set_local_description__set_error!(e); return 0;
                     } }
Object _setRemoteDescriptionProxy(Object arg0, Pointer<Utf8> arg1, Pointer<Utf8> arg2) {try {
                        return _setRemoteDescription!(arg0, arg1, arg2); } catch (e) { _peer_connection__set_remote_description__set_error!(e); return 0;
                     } }
void _closeProxy(Object arg0) {try {
                        return _close!(arg0); } catch (e) { _peer_connection__close__set_error!(e); return ;
                     } }
