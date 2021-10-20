use std::{future::Future, os::raw::c_char, ptr};

use dart_sys::Dart_Handle;
use derive_more::Display;
use medea_client_api_proto::{
    IceConnectionState, IceServer, PeerConnectionState,
};
use tracerr::Traced;

use crate::{
    api::dart::string_into_c_str,
    media::MediaKind,
    platform::{
        dart::{
            transceiver::Transceiver,
            utils::{
                callback_listener::Callback, handle::DartHandle,
                ice_connection_from_int, peer_connection_state_from_int,
            },
        },
        IceCandidate, RtcPeerConnectionError, RtcStats, SdpType,
        TransceiverDirection,
    },
};

use super::{
    ice_candidate::IceCandidate as PlatformIceCandidate,
    media_track::MediaStreamTrack, utils::dart_future::DartFutureResolver,
};
use crate::{
    api::dart::DartValueArg,
    platform::dart::utils::{
        callback_listener::TwoArgCallback,
        dart_future::FallibleDartFutureResolver,
    },
};
use std::convert::TryFrom;

/// Representation of the Dart SDP type.
// FIXME (evdokimovs): Migrate to the casting to i32 approach instead of
// to_string
#[derive(Display)]
pub enum RtcSdpType {
    /// The description is the initial proposal in an offer/answer exchange.
    #[display(fmt = "offer")]
    Offer,

    /// The description is the definitive choice in an offer/answer exchange
    #[display(fmt = "answer")]
    Answer,

    /// The description rolls back to offer/answer state to the last stable
    /// state.
    // FIXME (evdokimovs): Remove dead_code ignore when rollback will be
    //                     implemented.
    #[allow(dead_code)]
    #[display(fmt = "rollback")]
    Rollback,
}

type Result<T> = std::result::Result<T, Traced<RtcPeerConnectionError>>;

/// Pointer to an extern function that returns [`IceConnectionState`] of the
/// provided [`PeerConnection`].
type IceConnectionStateFunction = extern "C" fn(Dart_Handle) -> i32;

/// Pointer to an extern function that sets provided callback to the
/// `PeerConnection.on_connection_state_change`.
type OnConnectionStateChangeFunction = extern "C" fn(Dart_Handle, Dart_Handle);

/// Pointer to an extern function that returns [`ConnectionState`] of the
/// provided [`PeerConnection`].
type ConnectionStateFunction =
    extern "C" fn(Dart_Handle) -> DartValueArg<Option<i64>>;

/// Pointer to an extern function that request that ICE candidate gathering be
/// redone on both ends of the connection.
type RestartIceFunction = extern "C" fn(Dart_Handle);

/// Pointer to an extern function that rollbacks SDP offer of the provided
/// [`PeerConnection`].
type RollbackFunction = extern "C" fn(Dart_Handle) -> Dart_Handle;

/// Pointer to an extern function that sets `onTrack` callback of the provided
/// [`PeerConnection`].
type OnTrackFunction = extern "C" fn(Dart_Handle, Dart_Handle);

/// Pointer to an extern function that sets `onIceCandidate` callback of the
/// provided [`PeerConnection`].
type OnIceCandidateFunction = extern "C" fn(Dart_Handle, Dart_Handle);

/// Pointer to an extern function that lookups transceiver in provided
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
type NewPeerFunction = extern "C" fn() -> Dart_Handle;

type AddTransceiverFunction =
    extern "C" fn(Dart_Handle, i32, i32) -> Dart_Handle;

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
    /// Instantiates new [`RtcPeerConnection`].
    pub async fn new<I>(
        _ice_servers: I,
        _is_force_relayed: bool,
    ) -> Result<Self>
    where
        I: IntoIterator<Item = IceServer>,
    {
        Ok(Self {
            handle: FallibleDartFutureResolver::execute(unsafe {
                NEW_PEER.unwrap()()
            })
            .await
            .unwrap(),
        })
    }

    /// Returns [`RtcStats`] of this [`RtcPeerConnection`].
    pub async fn get_stats(&self) -> Result<RtcStats> {
        // TODO: Implement RTCStats
        Ok(RtcStats(Vec::new()))
    }

    /// Sets handler for a [`RtcTrackEvent`] (see [RTCTrackEvent][1] and
    /// [`ontrack` callback][2]).
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
                    TwoArgCallback::callback(
                        move |track: DartHandle, transceiver: DartHandle| {
                            f(
                                MediaStreamTrack::from(track),
                                Transceiver::from(transceiver),
                            );
                        },
                    ),
                );
            };
        }
    }

    /// Sets handler for a [`RtcPeerConnectionIceEvent`] (see
    /// [RTCPeerConnectionIceEvent][1] and [`onicecandidate` callback][2]).
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
                    Callback::callback(move |handle: DartHandle| {
                        let candidate = PlatformIceCandidate::from(handle);
                        f(IceCandidate {
                            candidate: candidate.candidate(),
                            sdp_m_line_index: candidate.sdp_m_line_index(),
                            sdp_mid: candidate.sdp_mid(),
                        });
                    }),
                );
            }
        }
    }

    /// Returns [`RtcIceConnectionState`] of this [`RtcPeerConnection`].
    #[must_use]
    pub fn ice_connection_state(&self) -> IceConnectionState {
        unsafe {
            let ice_connection_state =
                ICE_CONNECTION_STATE_FUNCTION.unwrap()(self.handle.get());
            ice_connection_from_int(ice_connection_state)
        }
    }

    /// Returns [`PeerConnectionState`] of this [`RtcPeerConnection`].
    ///
    /// Returns [`None`] if failed to parse a [`PeerConnectionState`].
    #[must_use]
    pub fn connection_state(&self) -> Option<PeerConnectionState> {
        unsafe {
            let connection_state: i64 = Option::try_from(
                CONNECTION_STATE_FUNCTION.unwrap()(self.handle.get()),
            )
            .unwrap()?;
            Some(peer_connection_state_from_int(connection_state as i32))
        }
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
                    Callback::callback(move |v: i64| {
                        f(ice_connection_from_int(v as i32));
                    }),
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
                    Callback::callback(move |v: i64| {
                        f(peer_connection_state_from_int(v as i32));
                    }),
                );
            }
        }
    }

    /// Adds remote [RTCPeerConnection][1]'s [ICE candidate][2] to this
    /// [`RtcPeerConnection`].
    ///
    /// [1]: https://w3.org/TR/webrtc/#rtcpeerconnection-interface
    /// [2]: https://tools.ietf.org/html/rfc5245#section-2
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
            FallibleDartFutureResolver::execute::<()>(fut)
                .await
                .map_err(|e| {
                    tracerr::new!(
                        RtcPeerConnectionError::AddIceCandidateFailed(e)
                    )
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
    pub async fn set_offer(&self, offer: &str) -> Result<()> {
        self.set_local_description(RtcSdpType::Offer, offer.to_string())
            .await
            .map_err(tracerr::map_from_and_wrap!())
    }

    /// Sets provided [SDP answer][`SdpType::Answer`] as local description.
    pub async fn set_answer(&self, answer: &str) -> Result<()> {
        self.set_local_description(RtcSdpType::Answer, answer.to_string())
            .await
            .map_err(tracerr::map_from_and_wrap!())
    }

    /// Sets local description to the provided one [`RtcSdpType`].
    async fn set_local_description(
        &self,
        sdp_type: RtcSdpType,
        sdp: String,
    ) -> Result<()> {
        unsafe {
            FallibleDartFutureResolver::execute(SET_LOCAL_DESCRIPTION_FUNCTION
                .unwrap()(
                self.handle.get(),
                string_into_c_str(sdp_type.to_string()),
                string_into_c_str(sdp),
            ))
            .await
            .map_err::<Traced<RtcPeerConnectionError>, _>(|e| {
                tracerr::new!(
                    RtcPeerConnectionError::SetLocalDescriptionFailed(e.into())
                )
            })?;
        }
        Ok(())
    }

    /// Instructs the underlying [RTCPeerConnection][`SysRtcPeerConnection`]
    /// to apply the supplied [SDP][`SdpType`] as the remote
    /// [offer][`SdpType::Offer`] or [answer][`SdpType::Answer`].
    ///
    /// Changes the local media state.
    pub async fn set_remote_description(&self, sdp: SdpType) -> Result<()> {
        match sdp {
            SdpType::Offer(sdp) => unsafe {
                FallibleDartFutureResolver::execute::<()>(
                    SET_REMOTE_DESCRIPTION_FUNCTION.unwrap()(
                        self.handle.get(),
                        string_into_c_str(RtcSdpType::Offer.to_string()),
                        string_into_c_str(sdp),
                    ),
                )
                .await
                .map_err(|e| {
                    tracerr::new!(
                        RtcPeerConnectionError::SetRemoteDescriptionFailed(e)
                    )
                })?;
            },
            SdpType::Answer(sdp) => unsafe {
                FallibleDartFutureResolver::execute::<()>(
                    SET_REMOTE_DESCRIPTION_FUNCTION.unwrap()(
                        self.handle.get(),
                        string_into_c_str(RtcSdpType::Answer.to_string()),
                        string_into_c_str(sdp),
                    ),
                )
                .await
                .map_err(|e| {
                    tracerr::new!(
                        RtcPeerConnectionError::SetRemoteDescriptionFailed(e)
                    )
                })?;
            },
        }
        Ok(())
    }

    /// Obtains [SDP answer][`SdpType::Answer`] from the underlying
    /// [RTCPeerConnection][`SysRtcPeerConnection`].
    ///
    /// Should be called whenever remote description has been changed.
    pub async fn create_answer(&self) -> Result<String> {
        unsafe {
            Ok(DartFutureResolver::execute(CREATE_ANSWER.unwrap()(
                self.handle.get(),
            ))
            .await)
        }
    }

    /// Obtains [SDP offer][`SdpType::Offer`] from the underlying
    /// [RTCPeerConnection][`SysRtcPeerConnection`].
    ///
    /// Should be called after local tracks changes, which require
    /// (re)negotiation.
    pub async fn create_offer(&self) -> Result<String> {
        unsafe {
            Ok(DartFutureResolver::execute(CREATE_OFFER.unwrap()(
                self.handle.get(),
            ))
            .await)
        }
    }

    /// Rollbacks the underlying [RTCPeerConnection][`SysRtcPeerConnection`] to
    /// the previous stable state.
    pub async fn rollback(&self) -> Result<()> {
        todo!("See todo below")
        // TODO: Use set_offer/create_offer function
        // unsafe { StdResult::<(),
        // Error>::from(ROLLBACK_FUNCTION.unwrap()(self.handle.get())).
        // map_err(|e| tracerr::new!(RtcPeerConnectionError::)) }
    }

    /// Creates new [`RtcRtpTransceiver`] (see [RTCRtpTransceiver][1])
    /// and adds it to the [set of this RTCPeerConnection's transceivers][2].
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
                let trnsvr: DartHandle = FallibleDartFutureResolver::execute(
                    ADD_TRANSCEIVER_FUNCTION.unwrap()(
                        handle,
                        kind as i32,
                        direction.into(),
                    ),
                )
                .await
                .unwrap();
                Transceiver::from(trnsvr)
            }
        }
    }

    /// Returns [`RtcRtpTransceiver`] (see [RTCRtpTransceiver][1]) from a
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
                    FallibleDartFutureResolver::execute(
                        GET_TRANSCEIVER_BY_MID_FUNCTION.unwrap()(
                            handle,
                            string_into_c_str(mid.to_string()),
                        ),
                    )
                    .await
                    .unwrap();
                transceiver.map(|h: DartHandle| Transceiver::from(h))
            }
        }
    }
}
