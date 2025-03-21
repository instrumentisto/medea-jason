//! Wrapper around [RTCPeerConnection][1].
//!
//! [1]: https://w3.org/TR/webrtc#dom-rtcpeerconnection

use std::rc::Rc;

use derive_more::with_trait::Display;
use medea_client_api_proto::{
    IceConnectionState, IceServer, PeerConnectionState, stats::RtcStat,
};
use medea_macro::dart_bridge;
use tracerr::Traced;

use super::{
    ice_candidate::{
        IceCandidate as PlatformIceCandidate,
        IceCandidateError as PlatformIceCandidateError,
    },
    media_track::MediaStreamTrack,
    transceiver::TransceiverInit,
    utils::string_into_c_str,
};
use crate::{
    media::MediaKind,
    platform::{
        IceCandidate, IceCandidateError, RtcPeerConnectionError, RtcStats,
        RtcStatsError, SdpType,
        dart::{
            ice_server::RtcIceServers,
            transceiver::Transceiver,
            utils::{
                callback::Callback, dart_future::FutureFromDart,
                handle::DartHandle, ice_connection_from_int,
                peer_connection_state_from_int,
            },
        },
    },
};

type RtcPeerConnectionResult<T> = Result<T, Traced<RtcPeerConnectionError>>;

#[dart_bridge("flutter/lib/src/native/platform/peer_connection.g.dart")]
mod peer_connection {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::platform::Error;

    extern "C" {
        /// Returns [`IceConnectionState`] of the provided [`PeerConnection`].
        pub fn ice_connection_state(peer: Dart_Handle) -> Result<i32, Error>;

        /// Sets the provided callback to a [`connectionstatechange`][1] event
        /// of the provided [`PeerConnection`].
        ///
        /// [1]: https://w3.org/TR/webrtc#event-connectionstatechange
        pub fn on_connection_state_change(
            peer: Dart_Handle,
            cb: Dart_Handle,
        ) -> Result<(), Error>;

        /// Returns a [`ConnectionState`] of the provided [`PeerConnection`].
        pub fn connection_state(peer: Dart_Handle) -> Result<i32, Error>;

        /// Requests an ICE candidate gathering redoing on both ends of the
        /// connection.
        pub fn restart_ice(peer: Dart_Handle) -> Result<(), Error>;

        /// Rollbacks SDP offer of the provided [`PeerConnection`].
        pub fn rollback(peer: Dart_Handle) -> Result<Dart_Handle, Error>;

        /// Returns JSON encoded [`Vec`] of [`RtcStats`] from the provided
        /// [`PeerConnection`].
        pub fn get_stats(peer: Dart_Handle) -> Result<Dart_Handle, Error>;

        /// Sets `onTrack` callback of the provided [`PeerConnection`].
        pub fn on_track(
            peer: Dart_Handle,
            cb: Dart_Handle,
        ) -> Result<(), Error>;

        /// Sets `onIceCandidate` callback of the provided [`PeerConnection`].
        pub fn on_ice_candidate(
            peer: Dart_Handle,
            cb: Dart_Handle,
        ) -> Result<(), Error>;

        /// Sets `onIceCandidateError` callback of the provided
        /// [`PeerConnection`].
        pub fn on_ice_candidate_error(
            peer: Dart_Handle,
            cb: Dart_Handle,
        ) -> Result<(), Error>;

        /// Looks ups [`Transceiver`] in the provided [`PeerConnection`] by the
        /// provided [`String`].
        pub fn get_transceiver_by_mid(
            peer: Dart_Handle,
            mid: ptr::NonNull<c_char>,
        ) -> Result<Dart_Handle, Error>;

        /// Adds the provided [`IceCandidate`] to the provided
        /// [`PeerConnection`].
        pub fn add_ice_candidate(
            peer: Dart_Handle,
            candidate: Dart_Handle,
        ) -> Result<Dart_Handle, Error>;

        /// Sets a callback for an [`iceconnectionstatechange`][1] event of the
        /// provided [`PeerConnection`].
        pub fn on_ice_connection_state_change(
            peer: Dart_Handle,
            cb: Dart_Handle,
        ) -> Result<(), Error>;

        /// Returns a [`Dart_Handle`] to a newly created [`PeerConnection`].
        pub fn new_peer(
            ice_servers: Dart_Handle,
            is_force_relayed: bool,
        ) -> Result<Dart_Handle, Error>;

        /// Creates a new [`Transceiver`] in the provided [`PeerConnection`].
        pub fn add_transceiver(
            peer: Dart_Handle,
            kind: i64,
            init: Dart_Handle,
        ) -> Result<Dart_Handle, Error>;

        /// Returns newly created SDP offer of the provided [`PeerConnection`].
        pub fn create_offer(peer: Dart_Handle) -> Result<Dart_Handle, Error>;

        /// Returns a newly created SDP answer of the provided
        /// [`PeerConnection`].
        pub fn create_answer(peer: Dart_Handle) -> Result<Dart_Handle, Error>;

        /// Sets the provided SDP offer as a local description of the provided
        /// [`PeerConnection`].
        pub fn set_local_description(
            peer: Dart_Handle,
            ty: ptr::NonNull<c_char>,
            offer: ptr::NonNull<c_char>,
        ) -> Result<Dart_Handle, Error>;

        /// Sets the provided SDP offer as a remote description of the provided
        /// [`PeerConnection`].
        pub fn set_remote_description(
            peer: Dart_Handle,
            ty: ptr::NonNull<c_char>,
            offer: ptr::NonNull<c_char>,
        ) -> Result<Dart_Handle, Error>;

        /// Closes the provided [`PeerConnection`].
        pub fn close(peer: Dart_Handle) -> Result<(), Error>;
    }
}

/// Representation of [RTCPeerConnection][1].
///
/// [1]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
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
    pub async fn new<I>(
        ice_servers: I,
        is_force_relayed: bool,
    ) -> RtcPeerConnectionResult<Self>
    where
        I: IntoIterator<Item = IceServer>,
    {
        let ice_servers = RtcIceServers::from(ice_servers);
        let fut = unsafe {
            peer_connection::new_peer(
                ice_servers.get_handle(),
                is_force_relayed,
            )
        }
        .unwrap();
        Ok(Self {
            handle: unsafe { FutureFromDart::execute(fut) }
                .await
                .map_err(RtcPeerConnectionError::PeerCreationError)
                .map_err(tracerr::wrap!())?,
        })
    }

    /// Returns [`RtcStats`] of this [`RtcPeerConnection`].
    // TODO: Needs refactoring.
    #[expect(clippy::missing_errors_doc, reason = "needs refactoring")]
    pub async fn get_stats(&self) -> RtcPeerConnectionResult<RtcStats> {
        let fut =
            unsafe { peer_connection::get_stats(self.handle.get()) }.unwrap();
        let stats_json: String = unsafe { FutureFromDart::execute(fut) }
            .await
            .map_err(RtcStatsError::Platform)
            .map_err(tracerr::from_and_wrap!())?;
        let rtc_stats: Vec<RtcStat> = serde_json::from_str(&stats_json)
            .map_err(Rc::new)
            .map_err(RtcStatsError::ParseError)
            .map_err(tracerr::from_and_wrap!())?;
        Ok(RtcStats(rtc_stats))
    }

    /// Sets `handler` for a [RTCTrackEvent][1] (see [`ontrack` callback][2]).
    ///
    /// [1]: https://w3.org/TR/webrtc#rtctrackevent
    /// [2]: https://w3.org/TR/webrtc#dom-rtcpeerconnection-ontrack
    pub fn on_track<F>(&self, handler: Option<F>)
    where
        F: 'static + FnMut(MediaStreamTrack, Transceiver),
    {
        if let Some(mut h) = handler {
            unsafe {
                peer_connection::on_track(
                    self.handle.get(),
                    Callback::from_two_arg_fn_mut(
                        move |track: DartHandle, transceiver: DartHandle| {
                            h(
                                MediaStreamTrack::new(track, None),
                                Transceiver::from(transceiver),
                            );
                        },
                    )
                    .into_dart(),
                )
            }
            .unwrap();
        }
    }

    /// Sets `handler` for a [RTCPeerConnectionIceEvent][1] (see
    /// [`onicecandidate` callback][2]).
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcpeerconnectioniceevent
    /// [2]: https://w3.org/TR/webrtc#dom-rtcpeerconnection-onicecandidate
    pub fn on_ice_candidate<F>(&self, handler: Option<F>)
    where
        F: 'static + FnMut(IceCandidate),
    {
        if let Some(mut h) = handler {
            unsafe {
                peer_connection::on_ice_candidate(
                    self.handle.get(),
                    Callback::from_fn_mut(move |handle: DartHandle| {
                        let candidate = PlatformIceCandidate::from(handle);
                        // Empty `candidate.candidate()` means that all the ICE
                        // transports have finished gathering candidates.
                        //
                        // Doesn't need to be delivered onward to the remote
                        // peer.
                        if !candidate.candidate().is_empty() {
                            h(IceCandidate {
                                candidate: candidate.candidate(),
                                sdp_m_line_index: candidate.sdp_m_line_index(),
                                sdp_mid: candidate.sdp_mid(),
                            });
                        }
                    })
                    .into_dart(),
                )
            }
            .unwrap();
        }
    }

    /// Sets `handler` for an [RTCPeerConnectionIceEvent][1] (see the
    /// [`onicecandidateerror` callback][2]).
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcpeerconnectioniceevent
    /// [2]: https://w3.org/TR/webrtc#dom-rtcpeerconnection-onicecandidateerror
    pub fn on_ice_candidate_error<F>(&self, handler: Option<F>)
    where
        F: 'static + FnMut(IceCandidateError),
    {
        if let Some(mut h) = handler {
            unsafe {
                peer_connection::on_ice_candidate_error(
                    self.handle.get(),
                    Callback::from_fn_mut(move |handle: DartHandle| {
                        let candidate = PlatformIceCandidateError::from(handle);
                        h(IceCandidateError {
                            address: Some(candidate.address()),
                            port: Some(candidate.port()),
                            url: candidate.url(),
                            error_code: candidate.error_code(),
                            error_text: candidate.error_text(),
                        });
                    })
                    .into_dart(),
                )
            }
            .unwrap();
        }
    }

    /// Returns [`IceConnectionState`] of this [`RtcPeerConnection`].
    #[must_use]
    pub fn ice_connection_state(&self) -> IceConnectionState {
        let ice_connection_state =
            unsafe { peer_connection::ice_connection_state(self.handle.get()) }
                .unwrap();
        ice_connection_from_int(ice_connection_state)
    }

    /// Returns [`PeerConnectionState`] of this [`RtcPeerConnection`].
    #[must_use]
    pub fn connection_state(&self) -> PeerConnectionState {
        let conn_state =
            unsafe { peer_connection::ice_connection_state(self.handle.get()) }
                .unwrap();
        peer_connection_state_from_int(conn_state)
    }

    /// Sets `handler` for an [`iceconnectionstatechange`][1] event.
    ///
    /// [1]: https://w3.org/TR/webrtc#event-iceconnectionstatechange
    pub fn on_ice_connection_state_change<F>(&self, handler: Option<F>)
    where
        F: 'static + FnMut(IceConnectionState),
    {
        if let Some(mut h) = handler {
            unsafe {
                peer_connection::on_ice_connection_state_change(
                    self.handle.get(),
                    Callback::from_fn_mut(move |v| {
                        h(ice_connection_from_int(v));
                    })
                    .into_dart(),
                )
            }
            .unwrap();
        }
    }

    /// Sets `handler` for a [`connectionstatechange`][1] event.
    ///
    /// [1]: https://w3.org/TR/webrtc#event-connectionstatechange
    pub fn on_connection_state_change<F>(&self, handler: Option<F>)
    where
        F: 'static + FnMut(PeerConnectionState),
    {
        if let Some(mut h) = handler {
            unsafe {
                peer_connection::on_connection_state_change(
                    self.handle.get(),
                    Callback::from_fn_mut(move |v| {
                        h(peer_connection_state_from_int(v));
                    })
                    .into_dart(),
                )
            }
            .unwrap();
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
    /// [1]: https://w3.org/TR/webrtc#rtcpeerconnection-interface
    /// [2]: https://tools.ietf.org/html/rfc5245#section-2
    /// [3]: https://w3.org/TR/webrtc#dom-peerconnection-addicecandidate
    pub async fn add_ice_candidate(
        &self,
        candidate: &str,
        sdp_m_line_index: Option<u16>,
        sdp_mid: &Option<String>,
    ) -> RtcPeerConnectionResult<()> {
        let fut = unsafe {
            peer_connection::add_ice_candidate(
                self.handle.get(),
                PlatformIceCandidate::new(candidate, sdp_m_line_index, sdp_mid)
                    .handle(),
            )
        }
        .unwrap();
        unsafe { FutureFromDart::execute::<()>(fut) }.await.map_err(|e| {
            tracerr::new!(RtcPeerConnectionError::AddIceCandidateFailed(e))
        })?;
        Ok(())
    }

    /// Marks [`RtcPeerConnection`] to trigger ICE restart.
    ///
    /// After this function returns, the offer returned by the next call to
    /// [`RtcPeerConnection::create_offer`] is automatically configured
    /// to trigger ICE restart.
    pub fn restart_ice(&self) {
        unsafe { peer_connection::restart_ice(self.handle.get()) }.unwrap();
    }

    /// Sets provided [SDP offer][`SdpType::Offer`] as local description.
    ///
    /// # Errors
    ///
    /// With [`RtcPeerConnectionError::SetLocalDescriptionFailed`] if
    /// [RtcPeerConnection.setLocalDescription()][1] fails.
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-peerconnection-setlocaldescription
    pub async fn set_offer(&self, offer: &str) -> RtcPeerConnectionResult<()> {
        self.set_local_description(RtcSdpType::Offer, offer.into())
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
    /// [1]: https://w3.org/TR/webrtc#dom-peerconnection-setlocaldescription
    pub async fn set_answer(
        &self,
        answer: &str,
    ) -> RtcPeerConnectionResult<()> {
        self.set_local_description(RtcSdpType::Answer, answer.into())
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
    /// [1]: https://w3.org/TR/webrtc#dom-rtcpeerconnection-createanswer
    pub async fn create_answer(&self) -> RtcPeerConnectionResult<String> {
        let fut = unsafe { peer_connection::create_answer(self.handle.get()) }
            .unwrap();
        unsafe { FutureFromDart::execute(fut) }
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
    /// [1]: https://w3.org/TR/webrtc#dom-peerconnection-setlocaldescription
    pub async fn rollback(&self) -> RtcPeerConnectionResult<()> {
        let fut =
            unsafe { peer_connection::rollback(self.handle.get()) }.unwrap();
        unsafe { FutureFromDart::execute(fut) }
            .await
            .map_err(RtcPeerConnectionError::SetLocalDescriptionFailed)
            .map_err(tracerr::wrap!())
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
    /// [1]: https://w3.org/TR/webrtc#dom-rtcpeerconnection-createoffer
    pub async fn create_offer(&self) -> RtcPeerConnectionResult<String> {
        let fut = unsafe { peer_connection::create_offer(self.handle.get()) }
            .unwrap();
        unsafe { FutureFromDart::execute(fut) }
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
    /// [1]: https://w3.org/TR/webrtc#dom-peerconnection-setremotedescription
    pub async fn set_remote_description(
        &self,
        sdp: SdpType,
    ) -> RtcPeerConnectionResult<()> {
        let fut = match sdp {
            SdpType::Offer(sdp) => unsafe {
                peer_connection::set_remote_description(
                    self.handle.get(),
                    string_into_c_str(RtcSdpType::Offer.to_string()),
                    string_into_c_str(sdp),
                )
            }
            .unwrap(),
            SdpType::Answer(sdp) => unsafe {
                peer_connection::set_remote_description(
                    self.handle.get(),
                    string_into_c_str(RtcSdpType::Answer.to_string()),
                    string_into_c_str(sdp),
                )
            }
            .unwrap(),
        };
        unsafe { FutureFromDart::execute::<()>(fut) }
            .await
            .map_err(RtcPeerConnectionError::SetRemoteDescriptionFailed)
            .map_err(tracerr::wrap!())
    }

    /// Creates a new [`Transceiver`] (see [RTCRtpTransceiver][1]) and adds it
    /// to the [set of this RTCPeerConnection's transceivers][2].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver
    /// [2]: https://w3.org/TR/webrtc#transceivers-set
    pub async fn add_transceiver(
        &self,
        kind: MediaKind,
        init: TransceiverInit,
    ) -> Transceiver {
        let fut = unsafe {
            peer_connection::add_transceiver(
                self.handle.get(),
                kind as i64,
                init.handle(),
            )
        }
        .unwrap();
        let trnsvr: DartHandle =
            unsafe { FutureFromDart::execute(fut) }.await.unwrap();
        Transceiver::from(trnsvr)
    }

    /// Returns [`Transceiver`] (see [RTCRtpTransceiver][1]) from a
    /// [set of this RTCPeerConnection's transceivers][2] by provided `mid`.
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver
    /// [2]: https://w3.org/TR/webrtc#transceivers-set
    pub fn get_transceiver_by_mid(
        &self,
        mid: String,
    ) -> impl Future<Output = Option<Transceiver>> + 'static + use<> {
        let handle = self.handle.get();
        async move {
            let fut = unsafe {
                peer_connection::get_transceiver_by_mid(
                    handle,
                    string_into_c_str(mid),
                )
            }
            .unwrap();
            let transceiver: Option<DartHandle> =
                unsafe { FutureFromDart::execute(fut) }.await.unwrap();
            transceiver.map(Transceiver::from)
        }
    }

    /// Sets local description to the provided [`RtcSdpType`].
    async fn set_local_description(
        &self,
        sdp_type: RtcSdpType,
        sdp: String,
    ) -> RtcPeerConnectionResult<()> {
        let fut = unsafe {
            peer_connection::set_local_description(
                self.handle.get(),
                string_into_c_str(sdp_type.to_string()),
                string_into_c_str(sdp),
            )
        }
        .unwrap();
        unsafe { FutureFromDart::execute(fut) }
            .await
            .map_err(RtcPeerConnectionError::SetLocalDescriptionFailed)
            .map_err(tracerr::wrap!())
    }
}

impl Drop for RtcPeerConnection {
    fn drop(&mut self) {
        unsafe { peer_connection::close(self.handle.get()) }.unwrap();
    }
}

/// Representation of a Dart SDP type.
#[derive(Display)]
pub enum RtcSdpType {
    /// Description is an initial proposal in an offer/answer exchange.
    #[display("offer")]
    Offer,

    /// Description is a definitive choice in an offer/answer exchange.
    #[display("answer")]
    Answer,
}
