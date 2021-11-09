//! Wrapper around [RTCPeerConnection][1].
//!
//! [1]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection

use std::{convert::TryFrom, future::Future, os::raw::c_char, ptr};

use dart_sys::Dart_Handle;
use derive_more::Display;
use medea_client_api_proto::{
    IceConnectionState, IceServer, PeerConnectionState,
};
use tracerr::Traced;

use crate::{
    api::{string_into_c_str, DartValueArg},
    media::MediaKind,
    platform::{
        dart::{
            ice_server::RtcIceServers,
            transceiver::Transceiver,
            utils::{
                callback::Callback, dart_future::FutureFromDart,
                handle::DartHandle, ice_connection_from_int,
                peer_connection_state_from_int,
            },
        },
        IceCandidate, RtcPeerConnectionError, RtcStats, SdpType,
        TransceiverDirection,
    },
};

use super::{
    ice_candidate::IceCandidate as PlatformIceCandidate,
    media_track::MediaStreamTrack,
};

type Result<T> = std::result::Result<T, Traced<RtcPeerConnectionError>>;

/// Pointer to an extern function returning an [`IceConnectionState`] of the
/// provided [`PeerConnection`].
type IceConnectionStateFunction = extern "C" fn(Dart_Handle) -> i32;

/// Pointer to an extern function setting the provided callback to the
/// `PeerConnection.on_connection_state_change`.
type OnConnectionStateChangeFunction = extern "C" fn(Dart_Handle, Dart_Handle);

/// Pointer to an extern function returning a [`ConnectionState`] of the
/// provided [`PeerConnection`].
type ConnectionStateFunction =
    extern "C" fn(Dart_Handle) -> ptr::NonNull<DartValueArg<Option<i32>>>;

/// Pointer to an extern function requesting an ICE candidate gathering redoing
/// on both ends of the connection.
type RestartIceFunction = extern "C" fn(Dart_Handle);

/// Pointer to an extern function rolling back SDP offer of the provided
/// [`PeerConnection`].
type RollbackFunction = extern "C" fn(Dart_Handle) -> Dart_Handle;

/// Pointer to an extern function setting `onTrack` callback of the provided
/// [`PeerConnection`].
type OnTrackFunction = extern "C" fn(Dart_Handle, Dart_Handle);

/// Pointer to an extern function that setting `onIceCandidate` callback of the
/// provided [`PeerConnection`].
type OnIceCandidateFunction = extern "C" fn(Dart_Handle, Dart_Handle);

/// Pointer to an extern function looking up transceiver in the provided
/// [`PeerConnection`] by provided [`String`].
type GetTransceiverByMid =
    extern "C" fn(Dart_Handle, ptr::NonNull<c_char>) -> Dart_Handle;

type GetTransceiverFunction =
    extern "C" fn(Dart_Handle, ptr::NonNull<c_char>, i32) -> Dart_Handle;

/// Pointer to an extern function that adds provided [`IceCandidate`] to the
/// provided [`PeerConnection`].
type AddIceCandidateFunction =
    extern "C" fn(Dart_Handle, Dart_Handle) -> Dart_Handle;

/// Pointer to an extern function that sets `onIceConnectionStateChange`
/// callback of the provided [`PeerConnection`].
type OnIceConnectionStateChangeFunction =
    extern "C" fn(Dart_Handle, Dart_Handle);

/// Pointer to an extern function that returns [`Dart_Handle`] to a newly
/// created [`PeerConnection`].
type NewPeerFunction = extern "C" fn(Dart_Handle) -> Dart_Handle;

/// Pointer to an extern function that creates new `Transceiver` in the provided
/// `PeerConnection`.
type AddTransceiverFunction =
    extern "C" fn(Dart_Handle, i64, i64) -> Dart_Handle;

/// Pointer to an extern function that returns newly created SDP offer of this
/// [`PeerConnection`].
type CreateOfferFunction = extern "C" fn(Dart_Handle) -> Dart_Handle;

/// Pointer to an extern function that returns newly created SDP answer of this
/// [`PeerConnection`].
type CreateAnswerFunction = extern "C" fn(Dart_Handle) -> Dart_Handle;

/// Pointer to an extern function that sets provided SDP offer as local
/// description of the provided [`PeerConnection`].
type SetLocalDescriptionFunction = extern "C" fn(
    Dart_Handle,
    ptr::NonNull<c_char>,
    ptr::NonNull<c_char>,
) -> Dart_Handle;

/// Pointer to an extern function that sets provided SDP offer as remote
/// description of the provided [`PeerConnection`].
type SetRemoteDescriptionFunction = extern "C" fn(
    Dart_Handle,
    ptr::NonNull<c_char>,
    ptr::NonNull<c_char>,
) -> Dart_Handle;

/// Stores pointer to the [`AddTransceiver`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut ADD_TRANSCEIVER_FUNCTION: Option<AddTransceiverFunction> = None;

/// Stores pointer to the [`ConnectionStateFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut CONNECTION_STATE_FUNCTION: Option<ConnectionStateFunction> = None;

/// Stores pointer to the [`AddIceCandidateFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut ADD_ICE_CANDIDATE_FUNCTION: Option<AddIceCandidateFunction> = None;

/// Stores pointer to the [`RestartIceFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut RESTART_ICE_FUNCTION: Option<RestartIceFunction> = None;

/// Stores pointer to the [`RollbackFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut ROLLBACK_FUNCTION: Option<RollbackFunction> = None;

/// Stores pointer to the [`GetTransceiverFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut GET_TRANSCEIVER_FUNCTION: Option<GetTransceiverFunction> = None;

/// Stores pointer to the [`GetTransceiverFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut GET_TRANSCEIVER_BY_MID_FUNCTION: Option<GetTransceiverByMid> = None;

/// Stores pointer to the [`SetLocalDescriptionFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut SET_LOCAL_DESCRIPTION_FUNCTION: Option<SetLocalDescriptionFunction> =
    None;

/// Stores pointer to the [`SetRemoteDescriptionFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut SET_REMOTE_DESCRIPTION_FUNCTION: Option<
    SetRemoteDescriptionFunction,
> = None;

/// Stores pointer to the [`OnTrackFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut ON_TRACK_FUNCTION: Option<OnTrackFunction> = None;

/// Stores pointer to the [`OnIceCandidateFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut ON_ICE_CANDIDATE_FUNCTION: Option<OnIceCandidateFunction> = None;

/// Stores pointer to the [`OnIceConnectionStateChangeFunction`] extern
/// function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut ON_ICE_CONNECTION_STATE_CHANGE_FUNCTION: Option<
    OnIceConnectionStateChangeFunction,
> = None;

/// Stores pointer to the [`OnConnectionStateChangeFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut ON_CONNECTION_STATE_CHANGE_FUNCTION: Option<
    OnConnectionStateChangeFunction,
> = None;

/// Stores pointer to the [`IceConnectionStateFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut ICE_CONNECTION_STATE_FUNCTION: Option<IceConnectionStateFunction> =
    None;

/// Stores pointer to the [`NewPeerFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut NEW_PEER: Option<NewPeerFunction> = None;

/// Stores pointer to the [`CreateOfferFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut CREATE_OFFER: Option<CreateOfferFunction> = None;

/// Stores pointer to the [`CreateAnswerFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut CREATE_ANSWER: Option<CreateAnswerFunction> = None;

/// Registers the provided [`CreateOfferFunction`] as [`CREATE_OFFER`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__create_offer(
    f: CreateOfferFunction,
) {
    CREATE_OFFER = Some(f);
}

/// Registers the provided [`CreateAnswerFunction`] as [`CREATE_ANSWER`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__create_answer(
    f: CreateAnswerFunction,
) {
    CREATE_ANSWER = Some(f);
}

/// Registers the provided [`IceConnectionStateFunction`] as
/// [`ICE_CONNECTION_STATE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__ice_connection_state(
    f: IceConnectionStateFunction,
) {
    ICE_CONNECTION_STATE_FUNCTION = Some(f);
}

/// Registers the provided [`ConnectionStateFunction`] as
/// [`CONNECTION_STATE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__connection_state(
    f: ConnectionStateFunction,
) {
    CONNECTION_STATE_FUNCTION = Some(f);
}

/// Registers the provided [`AddIceCandidateFunction`] as
/// [`ADD_ICE_CANDIDATE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__add_ice_candidate(
    f: AddIceCandidateFunction,
) {
    ADD_ICE_CANDIDATE_FUNCTION = Some(f);
}

/// Registers the provided [`RestartIceFunction`] as [`RESTART_ICE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__restart_ice(
    f: RestartIceFunction,
) {
    RESTART_ICE_FUNCTION = Some(f);
}

/// Registers the provided [`RollbackFunction`] as [`ROLLBACK_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__rollback(
    f: RollbackFunction,
) {
    ROLLBACK_FUNCTION = Some(f);
}

/// Registers the provided [`GetTransceiverFunction`] as
/// [`GET_TRANSCEIVER_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__get_transceiver(
    f: GetTransceiverFunction,
) {
    GET_TRANSCEIVER_FUNCTION = Some(f);
}

/// Registers the provided [`GetTransceiverByMid`] as
/// [`GET_TRANSCEIVER_BY_MID`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__get_transceiver_by_mid(
    f: GetTransceiverByMid,
) {
    GET_TRANSCEIVER_BY_MID_FUNCTION = Some(f);
}

/// Registers the provided [`SetLocalDescriptionFunction`] as
/// [`SET_LOCAL_DESCRIPTION_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__set_local_description(
    f: SetLocalDescriptionFunction,
) {
    SET_LOCAL_DESCRIPTION_FUNCTION = Some(f);
}

/// Registers the provided [`SetRemoteDescriptionFunction`] as
/// [`SET_REMOTE_DESCRIPTION_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__set_remote_description(
    f: SetRemoteDescriptionFunction,
) {
    SET_REMOTE_DESCRIPTION_FUNCTION = Some(f);
}

/// Registers the provided [`OnTrackFunction`] as [`ON_TRACK_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__on_track(
    f: OnTrackFunction,
) {
    ON_TRACK_FUNCTION = Some(f);
}

/// Registers the provided [`OnIceCandidateFunction`] as
/// [`ON_ICE_CANDIDATE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__on_ice_candidate(
    f: OnIceCandidateFunction,
) {
    ON_ICE_CANDIDATE_FUNCTION = Some(f);
}

/// Registers the provided [`OnIceConnectionStateChangeFunction`] as
/// [`ON_ICE_CONNECTION_STATE_CHANGE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__on_ice_connection_state_change(
    f: OnIceConnectionStateChangeFunction,
) {
    ON_ICE_CONNECTION_STATE_CHANGE_FUNCTION = Some(f);
}

/// Registers the provided [`OnConnectionStateChangeFunction`] as
/// [`ON_CONNECTION_STATE_CHANGE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__on_connection_state_change(
    f: OnConnectionStateChangeFunction,
) {
    ON_CONNECTION_STATE_CHANGE_FUNCTION = Some(f);
}

/// Registers the provided [`NewPeerFunction`] as [`NEW_PEER_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__new_peer(
    f: NewPeerFunction,
) {
    NEW_PEER = Some(f);
}

/// Registers the provided [`AddTransceiverFunction`] as
/// [`ADD_TRANSCEIVER_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RtcPeerConnection__add_transceiver(
    f: AddTransceiverFunction,
) {
    ADD_TRANSCEIVER_FUNCTION = Some(f);
}

/// Representation of [RTCPeerConnection][1].
///
/// [1]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection
#[derive(Clone, Debug)]
pub struct RtcPeerConnection {
    handle: DartHandle,
}

impl RtcPeerConnection {
    /// Instantiates a new [`RtcPeerConnection`].
    ///
    /// # Errors
    ///
    /// Errors with [`RtcPeerConnectionError::PeerCreationError`] if
    /// [`RtcPeerConnection`] creation fails.
    pub async fn new<I>(ice_servers: I, is_force_relayed: bool) -> Result<Self>
    where
        I: IntoIterator<Item = IceServer>,
    {
        let ice_servers = RtcIceServers::from(ice_servers);
        Ok(Self {
            handle: FutureFromDart::execute(unsafe {
                NEW_PEER.unwrap()(ice_servers.get_handle())
            })
            .await
            .map_err(RtcPeerConnectionError::PeerCreationError)
            .map_err(tracerr::wrap!())?,
        })
    }

    /// Returns [`RtcStats`] of this [`RtcPeerConnection`].
    #[allow(clippy::missing_errors_doc)]
    pub async fn get_stats(&self) -> Result<RtcStats> {
        // TODO: Correct implementation requires `flutter_webrtc`-side rework.
        Ok(RtcStats(Vec::new()))
    }

    /// Sets handler for a [RTCTrackEvent][1] (see [`ontrack` callback][2]).
    ///
    /// [1]: https://w3.org/TR/webrtc/#rtctrackevent
    /// [2]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection-ontrack
    pub fn on_track<F>(&self, f: Option<F>)
    where
        F: 'static + FnMut(MediaStreamTrack, Transceiver),
    {
        if let Some(mut f) = f {
            unsafe {
                ON_TRACK_FUNCTION.unwrap()(
                    self.handle.get(),
                    Callback::from_two_arg_fn_mut(
                        move |track: DartHandle, transceiver: DartHandle| {
                            f(
                                MediaStreamTrack::from(track),
                                Transceiver::from(transceiver),
                            );
                        },
                    )
                    .into_dart(),
                );
            };
        }
    }

    /// Sets handler for a [RTCPeerConnectionIceEvent][1] (see
    /// [`onicecandidate` callback][2]).
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-rtcpeerconnectioniceevent
    /// [2]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection-onicecandidate
    pub fn on_ice_candidate<F>(&self, f: Option<F>)
    where
        F: 'static + FnMut(IceCandidate),
    {
        if let Some(mut f) = f {
            unsafe {
                ON_ICE_CANDIDATE_FUNCTION.unwrap()(
                    self.handle.get(),
                    Callback::from_fn_mut(move |handle: DartHandle| {
                        let candidate = PlatformIceCandidate::from(handle);
                        f(IceCandidate {
                            candidate: candidate.candidate(),
                            sdp_m_line_index: candidate.sdp_m_line_index(),
                            sdp_mid: candidate.sdp_mid(),
                        });
                    })
                    .into_dart(),
                );
            }
        }
    }

    /// Returns [`IceConnectionState`] of this [`RtcPeerConnection`].
    #[must_use]
    pub fn ice_connection_state(&self) -> IceConnectionState {
        let ice_connection_state = unsafe {
            ICE_CONNECTION_STATE_FUNCTION.unwrap()(self.handle.get())
        };
        ice_connection_from_int(ice_connection_state)
    }

    /// Returns [`PeerConnectionState`] of this [`RtcPeerConnection`].
    ///
    /// Returns [`None`] if failed to parse a [`PeerConnectionState`].
    #[must_use]
    pub fn connection_state(&self) -> Option<PeerConnectionState> {
        let connection_state = Option::try_from(unsafe {
            *Box::from_raw(
                CONNECTION_STATE_FUNCTION.unwrap()(self.handle.get()).as_ptr(),
            )
        })
        .unwrap()?;
        Some(peer_connection_state_from_int(connection_state))
    }

    /// Sets handler for an [`iceconnectionstatechange`][1] event.
    ///
    /// [1]: https://w3.org/TR/webrtc/#event-iceconnectionstatechange
    pub fn on_ice_connection_state_change<F>(&self, f: Option<F>)
    where
        F: 'static + FnMut(IceConnectionState),
    {
        if let Some(mut f) = f {
            unsafe {
                ON_ICE_CONNECTION_STATE_CHANGE_FUNCTION.unwrap()(
                    self.handle.get(),
                    Callback::from_fn_mut(move |v| {
                        f(ice_connection_from_int(v));
                    })
                    .into_dart(),
                );
            }
        }
    }

    /// Sets handler for a [`connectionstatechange`][1] event.
    ///
    /// [1]: https://w3.org/TR/webrtc/#event-connectionstatechange
    pub fn on_connection_state_change<F>(&self, f: Option<F>)
    where
        F: 'static + FnMut(PeerConnectionState),
    {
        if let Some(mut f) = f {
            unsafe {
                ON_CONNECTION_STATE_CHANGE_FUNCTION.unwrap()(
                    self.handle.get(),
                    Callback::from_fn_mut(move |v| {
                        f(peer_connection_state_from_int(v));
                    })
                    .into_dart(),
                );
            }
        }
    }

    /// Adds remote [RTCPeerConnection][1]'s [ICE candidate][2] to this
    /// [`RtcPeerConnection`].
    ///
    /// # Errors
    ///
    /// With [`RtcPeerConnectionError::AddIceCandidateFailed`] if
    /// [RtcPeerConnection.addIceCandidate()][3] fails.
    ///
    /// [1]: https://w3.org/TR/webrtc/#rtcpeerconnection-interface
    /// [2]: https://tools.ietf.org/html/rfc5245#section-2
    /// [3]: https://w3.org/TR/webrtc/#dom-peerconnection-addicecandidate
    pub async fn add_ice_candidate(
        &self,
        candidate: &str,
        sdp_m_line_index: Option<u16>,
        sdp_mid: &Option<String>,
    ) -> Result<()> {
        unsafe {
            let fut = ADD_ICE_CANDIDATE_FUNCTION.unwrap()(
                self.handle.get(),
                PlatformIceCandidate::new(candidate, sdp_m_line_index, sdp_mid)
                    .handle(),
            );
            FutureFromDart::execute::<()>(fut).await.map_err(|e| {
                tracerr::new!(RtcPeerConnectionError::AddIceCandidateFailed(e))
            })?;
        };
        Ok(())
    }

    /// Marks [`RtcPeerConnection`] to trigger ICE restart.
    ///
    /// After this function returns, the offer returned by the next call to
    /// [`RtcPeerConnection::create_offer`] is automatically configured
    /// to trigger ICE restart.
    pub fn restart_ice(&self) {
        unsafe { RESTART_ICE_FUNCTION.unwrap()(self.handle.get()) };
    }

    /// Sets provided [SDP offer][`SdpType::Offer`] as local description.
    ///
    /// # Errors
    ///
    /// With [`RtcPeerConnectionError::SetLocalDescriptionFailed`] if
    /// [RtcPeerConnection.setLocalDescription()][1] fails.
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-peerconnection-setlocaldescription
    pub async fn set_offer(&self, offer: &str) -> Result<()> {
        self.set_local_description(RtcSdpType::Offer, offer.to_string())
            .await
            .map_err(tracerr::map_from_and_wrap!())
    }

    /// Sets the provided [SDP answer][`SdpType::Answer`] as local description.
    ///
    /// # Errors
    ///
    /// With [`RtcPeerConnectionError::SetLocalDescriptionFailed`] if
    /// [RtcPeerConnection.setLocalDescription()][1] fails.
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-peerconnection-setlocaldescription
    pub async fn set_answer(&self, answer: &str) -> Result<()> {
        self.set_local_description(RtcSdpType::Answer, answer.to_string())
            .await
            .map_err(tracerr::map_from_and_wrap!())
    }

    /// Obtains [SDP answer][`SdpType::Answer`] from the [`RtcPeerConnection`].
    ///
    /// Should be called whenever remote description has been changed.
    ///
    /// # Errors
    ///
    /// With [`RtcPeerConnectionError::CreateAnswerFailed`] if
    /// [RtcPeerConnection.createAnswer()][1] fails.
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection-createanswer
    pub async fn create_answer(&self) -> Result<String> {
        FutureFromDart::execute(unsafe {
            CREATE_ANSWER.unwrap()(self.handle.get())
        })
        .await
        .map_err(RtcPeerConnectionError::CreateAnswerFailed)
        .map_err(tracerr::wrap!())
    }

    /// Rollbacks the [`RtcPeerConnection`] to the previous stable state.
    ///
    /// # Errors
    ///
    /// With [`RtcPeerConnectionError::SetLocalDescriptionFailed`] if
    /// [RtcPeerConnection.setLocalDescription()][1] fails.
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-peerconnection-setlocaldescription
    pub async fn rollback(&self) -> Result<()> {
        FutureFromDart::execute(unsafe {
            ROLLBACK_FUNCTION.unwrap()(self.handle.get())
        })
        .await
        .map_err(RtcPeerConnectionError::SetLocalDescriptionFailed)
        .map_err(tracerr::wrap!())
        .map(drop)
    }

    /// Obtains [SDP offer][`SdpType::Offer`] from the [`RtcPeerConnection`].
    ///
    /// Should be called after local tracks changes, which require
    /// (re)negotiation.
    ///
    /// # Errors
    ///
    /// With [`RtcPeerConnectionError::CreateOfferFailed`] if
    /// [RtcPeerConnection.createOffer()][1] fails.
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection-createoffer
    pub async fn create_offer(&self) -> Result<String> {
        FutureFromDart::execute(unsafe {
            CREATE_OFFER.unwrap()(self.handle.get())
        })
        .await
        .map_err(RtcPeerConnectionError::CreateOfferFailed)
        .map_err(tracerr::wrap!())
    }

    /// Instructs the [`RtcPeerConnection`] to apply the supplied
    /// [SDP][`SdpType`] as the remote [offer][`SdpType::Offer`] or
    /// [answer][`SdpType::Answer`].
    ///
    /// Changes the local media state.
    ///
    /// # Errors
    ///
    /// With [`RtcPeerConnectionError::SetRemoteDescriptionFailed`] if
    /// [RTCPeerConnection.setRemoteDescription()][1] fails.
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-peerconnection-setremotedescription
    pub async fn set_remote_description(&self, sdp: SdpType) -> Result<()> {
        match sdp {
            SdpType::Offer(sdp) => unsafe {
                FutureFromDart::execute::<()>(SET_REMOTE_DESCRIPTION_FUNCTION
                    .unwrap()(
                    self.handle.get(),
                    string_into_c_str(RtcSdpType::Offer.to_string()),
                    string_into_c_str(sdp),
                ))
                .await
                .map_err(RtcPeerConnectionError::SetRemoteDescriptionFailed)
                .map_err(tracerr::wrap!())
            },
            SdpType::Answer(sdp) => unsafe {
                FutureFromDart::execute::<()>(SET_REMOTE_DESCRIPTION_FUNCTION
                    .unwrap()(
                    self.handle.get(),
                    string_into_c_str(RtcSdpType::Answer.to_string()),
                    string_into_c_str(sdp),
                ))
                .await
                .map_err(RtcPeerConnectionError::SetRemoteDescriptionFailed)
                .map_err(tracerr::wrap!())
            },
        }
    }

    /// Creates a new [`Transceiver`] (see [RTCRtpTransceiver][1]) and adds it
    /// to the [set of this RTCPeerConnection's transceivers][2].
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-rtcrtptransceiver
    /// [2]: https://w3.org/TR/webrtc/#transceivers-set
    pub fn add_transceiver(
        &self,
        kind: MediaKind,
        direction: TransceiverDirection,
    ) -> impl Future<Output = Transceiver> + 'static {
        unsafe {
            let handle = self.handle.get();
            async move {
                let trnsvr: DartHandle =
                    FutureFromDart::execute(ADD_TRANSCEIVER_FUNCTION.unwrap()(
                        handle,
                        kind as i64,
                        direction.into(),
                    ))
                    .await
                    .unwrap();
                Transceiver::from(trnsvr)
            }
        }
    }

    /// Returns [`Transceiver`] (see [RTCRtpTransceiver][1]) from a
    /// [set of this RTCPeerConnection's transceivers][2] by provided `mid`.
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-rtcrtptransceiver
    /// [2]: https://w3.org/TR/webrtc/#transceivers-set
    pub fn get_transceiver_by_mid(
        &self,
        mid: String,
    ) -> impl Future<Output = Option<Transceiver>> + 'static {
        unsafe {
            let handle = self.handle.get();
            async move {
                let transceiver: Option<DartHandle> =
                    FutureFromDart::execute(GET_TRANSCEIVER_BY_MID_FUNCTION
                        .unwrap()(
                        handle,
                        string_into_c_str(mid.to_string()),
                    ))
                    .await
                    .unwrap();
                transceiver.map(Transceiver::from)
            }
        }
    }

    /// Sets local description to the provided [`RtcSdpType`].
    async fn set_local_description(
        &self,
        sdp_type: RtcSdpType,
        sdp: String,
    ) -> Result<()> {
        unsafe {
            FutureFromDart::execute(SET_LOCAL_DESCRIPTION_FUNCTION.unwrap()(
                self.handle.get(),
                string_into_c_str(sdp_type.to_string()),
                string_into_c_str(sdp),
            ))
            .await
            .map_err(RtcPeerConnectionError::SetLocalDescriptionFailed)
            .map_err(tracerr::wrap!())
        }
    }
}

/// Representation of a Dart SDP type.
#[derive(Display)]
pub enum RtcSdpType {
    /// Description is an initial proposal in an offer/answer exchange.
    #[display(fmt = "offer")]
    Offer,

    /// Description is a definitive choice in an offer/answer exchange.
    #[display(fmt = "answer")]
    Answer,
}
