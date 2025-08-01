//! Implementation of a [`Component`] watchers.

use std::{collections::HashSet, rc::Rc};

use derive_more::with_trait::{Display, From};
use futures::{StreamExt as _, future};
use medea_client_api_proto::{
    IceCandidate, MemberId, NegotiationRole, TrackId,
};
use medea_macro::watchers;
use medea_reactive::Guarded;
use tracerr::Traced;

use super::{Component, PeerConnection, State};
use crate::{
    peer::{
        GetMidsError, PeerEvent, RtcPeerConnectionError,
        component::{NegotiationPhase, SyncPhase},
        media::{receiver, sender},
    },
    utils::{Updatable as _, transpose_guarded},
};

/// Errors occurring in watchers of a [`Component`].
#[derive(Clone, Debug, Display, From)]
enum PeerWatcherError {
    /// Errors occurred in platform's [RTCPeerConnection][1].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
    RtcPeerConnection(RtcPeerConnectionError),

    /// Failed to acquire a list of `mid`s from a [`PeerConnection`].
    GetMids(GetMidsError),

    /// Failed to create a [`Sender`].
    ///
    /// [`Sender`]: sender::Sender
    SenderCreateFailed(sender::CreateError),
}

// TODO: Don't force spawned watchers to return `Result`.
#[watchers]
impl Component {
    /// Watcher for the [`State::ice_candidates`] push update.
    ///
    /// Calls [`PeerConnection::add_ice_candidate()`] with the pushed
    /// [`IceCandidate`].
    #[watch(self.ice_candidates.on_add())]
    async fn ice_candidate_added(
        peer: Rc<PeerConnection>,
        _: Rc<State>,
        candidate: IceCandidate,
    ) -> Result<(), Traced<RtcPeerConnectionError>> {
        peer.add_ice_candidate(
            candidate.candidate,
            candidate.sdp_m_line_index,
            candidate.sdp_mid,
        )
        .await
        .map_err(tracerr::map_from_and_wrap!())
    }

    /// Watcher for the [`State::remote_sdp`] update.
    ///
    /// Calls [`PeerConnection::set_remote_answer()`] with a new value if the
    /// current [`NegotiationRole`] is an [`Offerer`].
    ///
    /// Calls [`PeerConnection::set_remote_offer()`] with a new value if the
    /// current [`NegotiationRole`] is an [`Answerer`].
    ///
    /// [`Answerer`]: NegotiationRole::Answerer
    /// [`Offerer`]: NegotiationRole::Offerer
    #[watch(self.remote_sdp.subscribe().filter_map(transpose_guarded))]
    async fn remote_sdp_changed(
        peer: Rc<PeerConnection>,
        state: Rc<State>,
        description: Guarded<String>,
    ) -> Result<(), Traced<RtcPeerConnectionError>> {
        let (description, _guard) = description.into_parts();
        if let Some(role) = state.negotiation_role.get() {
            match role {
                NegotiationRole::Offerer => {
                    peer.set_remote_answer(description)
                        .await
                        .map_err(tracerr::map_from_and_wrap!())?;
                    peer.media_connections.sync_receivers().await;
                    state.negotiation_phase.set(NegotiationPhase::Stable);
                    state.negotiation_role.set(None);
                }
                NegotiationRole::Answerer(_) => {
                    peer.set_remote_offer(description)
                        .await
                        .map_err(tracerr::map_from_and_wrap!())?;
                    peer.media_connections.sync_receivers().await;
                }
            }
        }
        Ok(())
    }

    /// Watcher for the [`State::senders`] remove update.
    ///
    /// Removes a [`sender::Component`] from the [`PeerConnection`].
    #[watch(self.senders.on_remove())]
    fn sender_removed(
        peer: &PeerConnection,
        _: &State,
        val: Guarded<(TrackId, Rc<sender::State>)>,
    ) {
        let ((track_id, _), _guard) = val.into_parts();
        peer.remove_track(track_id);
        peer.connections.remove_track(&track_id);
    }

    /// Watcher for the [`State::receivers`] remove update.
    ///
    /// Removes a [`receiver::Component`] from the [`PeerConnection`].
    #[watch(self.receivers.on_remove())]
    fn receiver_removed(
        peer: &PeerConnection,
        _: &State,
        val: Guarded<(TrackId, Rc<receiver::State>)>,
    ) {
        let ((track_id, _), _guard) = val.into_parts();
        peer.remove_track(track_id);
        peer.connections.remove_track(&track_id);
    }

    /// Watcher for the [`State::senders`] insert update.
    ///
    /// Waits until [`receiver::Component`]s creation is finished.
    ///
    /// Waits for a remote SDP offer apply if the current [`NegotiationRole`] is
    /// an [`Answerer`].
    ///
    /// Creates a new [`sender::Component`], creates a new [`Connection`] with
    /// all [`sender::State::receivers`] by calling a
    /// [`Connections::create_connection()`][1].
    ///
    /// [`Answerer`]: NegotiationRole::Answerer
    /// [`Connection`]: crate::connection::Connection
    /// [1]: crate::connection::Connections::create_connection
    #[watch(self.senders.on_insert())]
    async fn sender_added(
        peer: Rc<PeerConnection>,
        state: Rc<State>,
        val: Guarded<(TrackId, Rc<sender::State>)>,
    ) -> Result<(), Traced<PeerWatcherError>> {
        let mut wait_futs = vec![
            state.negotiation_role.when_all_processed().into(),
            state.when_all_receivers_processed().into(),
        ];
        if matches!(
            state.negotiation_role.get(),
            Some(NegotiationRole::Answerer(_))
        ) {
            wait_futs.push(state.remote_sdp.when_all_processed().into());
        }
        medea_reactive::when_all_processed(wait_futs).await;

        let ((track_id, new_sender), _guard) = val.into_parts();
        drop(peer.connections.update_connections(
            &track_id,
            new_sender.receivers().into_iter().collect(),
            state.connection_mode,
        ));
        let sender = sender::Sender::new(
            &new_sender,
            &peer.media_connections,
            peer.send_constraints.clone(),
            peer.track_events_sender.clone(),
        )
        .await
        .inspect_err(|e| {
            drop(peer.peer_events_sender.unbounded_send(
                PeerEvent::FailedLocalMedia {
                    error: tracerr::map_from(e.clone()),
                },
            ));
        })
        .map_err(tracerr::map_from_and_wrap!())?;
        peer.media_connections
            .insert_sender(sender::Component::new(sender, new_sender));
        Ok(())
    }

    /// Watcher for the [`State::receivers`] insert update.
    ///
    /// Creates a new [`receiver::Component`], creates a new [`Connection`] with
    /// a [`receiver::State::sender_id`] by calling a
    /// [`Connections::create_connection()`][1].
    ///
    /// [`Connection`]: crate::connection::Connections
    /// [1]: crate::connection::Connections::create_connection
    #[watch(self.receivers.on_insert())]
    async fn receiver_added(
        peer: Rc<PeerConnection>,
        state: Rc<State>,
        val: Guarded<(TrackId, Rc<receiver::State>)>,
    ) {
        let ((track_id, rcvr_state), _guard) = val.into_parts();
        let conns = peer.connections.update_connections(
            &track_id,
            HashSet::from([rcvr_state.sender_id().clone()]),
            state.connection_mode,
        );
        let receiver = receiver::Receiver::new(
            &rcvr_state,
            &peer.media_connections,
            peer.track_events_sender.clone(),
            &peer.recv_constraints,
            state.connection_mode,
        )
        .await;
        peer.media_connections.insert_receiver(receiver::Component::new(
            Rc::new(receiver),
            Rc::clone(&rcvr_state),
        ));
        for conn in conns {
            conn.add_receiver(Rc::clone(&rcvr_state));
        }
    }

    /// Watcher for the [`State::connections`] insert update.
    ///
    /// Creates a new [`Connection`] for the given [`PeerConnection`].
    #[watch(
        self.maybe_update_connections.subscribe().filter_map(future::ready)
    )]
    fn maybe_update_connections(
        peer: &PeerConnection,
        state: &State,
        val: (TrackId, HashSet<MemberId>),
    ) {
        drop(peer.connections.update_connections(
            &val.0,
            val.1,
            state.connection_mode,
        ));

        state.maybe_update_connections.set(None);
    }

    /// Watcher for the [`State::local_sdp`] updates.
    ///
    /// Sets [`PeerConnection`]'s SDP offer to the provided one and sends
    /// a [`PeerEvent::NewSdpOffer`] if [`NegotiationRole`] is
    /// [`NegotiationRole::Offerer`].
    ///
    /// Sets [`PeerConnection`]'s SDP answer to the provided one and sends
    /// a [`PeerEvent::NewSdpAnswer`] if [`NegotiationRole`] is
    /// [`NegotiationRole::Answerer`].
    ///
    /// Rollbacks [`PeerConnection`] to a stable state if [`PeerConnection`] is
    /// marked for rollback and [`NegotiationRole`] is [`Some`].
    ///
    /// [`Answerer`]: NegotiationRole::Answerer
    /// [`Offerer`]: NegotiationRole::Offerer
    #[watch(self.local_sdp.subscribe().filter_map(future::ready))]
    async fn local_sdp_changed(
        peer: Rc<PeerConnection>,
        state: Rc<State>,
        sdp: String,
    ) -> Result<(), Traced<PeerWatcherError>> {
        _ = state.sync_phase.when_eq(SyncPhase::Synced).await;
        if let Some(role) = state.negotiation_role.get() {
            if state.local_sdp.is_rollback() {
                // TODO: Temporary fix that allows us to ignore rollback
                //       since it won't work anyway.
                if state.negotiation_phase.get() != NegotiationPhase::Stable {
                    peer.peer
                        .rollback()
                        .await
                        .map_err(tracerr::map_from_and_wrap!())?;
                }
                if state.local_sdp.is_restart_needed() {
                    state.negotiation_phase.set(NegotiationPhase::WaitLocalSdp);
                } else {
                    state.negotiation_phase.set(NegotiationPhase::Stable);
                    state.negotiation_role.set(None);
                }
            } else {
                match role {
                    NegotiationRole::Offerer => {
                        peer.peer
                            .set_offer(&sdp)
                            .await
                            .map_err(tracerr::map_from_and_wrap!())?;
                        peer.media_connections.sync_receivers().await;
                        let mids = peer
                            .get_mids()
                            .map_err(tracerr::map_from_and_wrap!())?;
                        _ = peer
                            .peer_events_sender
                            .unbounded_send(PeerEvent::NewSdpOffer {
                                peer_id: peer.id(),
                                sdp_offer: sdp,
                                transceivers_statuses: peer
                                    .get_transceivers_statuses()
                                    .await,
                                mids,
                            })
                            .ok();
                        state
                            .negotiation_phase
                            .set(NegotiationPhase::WaitLocalSdpApprove);
                    }
                    NegotiationRole::Answerer(_) => {
                        peer.peer
                            .set_answer(&sdp)
                            .await
                            .map_err(tracerr::map_from_and_wrap!())?;
                        peer.media_connections.sync_receivers().await;
                        _ = peer
                            .peer_events_sender
                            .unbounded_send(PeerEvent::NewSdpAnswer {
                                peer_id: peer.id(),
                                sdp_answer: sdp,
                                transceivers_statuses: peer
                                    .get_transceivers_statuses()
                                    .await,
                            })
                            .ok();
                        state
                            .negotiation_phase
                            .set(NegotiationPhase::WaitLocalSdpApprove);
                    }
                }
            }
        }
        Ok(())
    }

    /// Watcher for the SDP offer approving by server.
    ///
    /// If the current [`NegotiationRole`] is an [`NegotiationRole::Offerer`]
    /// then [`NegotiationPhase`] will transit to a [`WaitRemoteSdp`].
    ///
    /// If the current [`NegotiationRole`] is an [`NegotiationRole::Answerer`]
    /// then [`NegotiationPhase`] will transit to a [`Stable`].
    ///
    /// [`Offerer`]: NegotiationRole::Offerer
    /// [`Stable`]: NegotiationPhase::Stable
    /// [`WaitRemoteSdp`]: NegotiationPhase::WaitRemoteSdp
    #[watch(self.local_sdp.on_approve().skip(1))]
    fn local_sdp_approved(_: &PeerConnection, state: &State, (): ()) {
        if let Some(negotiation_role) = state.negotiation_role.get() {
            match negotiation_role {
                NegotiationRole::Offerer => {
                    state
                        .negotiation_phase
                        .set(NegotiationPhase::WaitRemoteSdp);
                }
                NegotiationRole::Answerer(_) => {
                    state.negotiation_phase.set(NegotiationPhase::Stable);
                    state.negotiation_role.set(None);
                }
            }
        }
    }

    /// Watcher for the [`NegotiationPhase`] change.
    ///
    /// Resets [`NegotiationRole`] to [`None`] on a
    /// [`NegotiationPhase::Stable`].
    ///
    /// Creates and sets local SDP offer on a
    /// [`NegotiationPhase::WaitLocalSdp`].
    #[watch(self.negotiation_phase.subscribe().skip(1))]
    async fn negotiation_phase_changed(
        peer: Rc<PeerConnection>,
        state: Rc<State>,
        negotiation_state: NegotiationPhase,
    ) -> Result<(), Traced<RtcPeerConnectionError>> {
        medea_reactive::when_all_processed(vec![
            state.when_all_updated().into(),
            state.when_all_senders_processed().into(),
            state.when_all_receivers_processed().into(),
            state.remote_sdp.when_all_processed().into(),
        ])
        .await;

        match negotiation_state {
            NegotiationPhase::WaitLocalSdp => {
                if let Some(negotiation_role) = state.negotiation_role.get() {
                    match negotiation_role {
                        NegotiationRole::Offerer => {
                            if state.restart_ice.take() {
                                peer.restart_ice();
                            }
                            let sdp_offer = peer
                                .peer
                                .create_offer()
                                .await
                                .map_err(tracerr::map_from_and_wrap!())?;
                            state.local_sdp.unapproved_set(sdp_offer);
                        }
                        NegotiationRole::Answerer(_) => {
                            let sdp_answer = peer
                                .peer
                                .create_answer()
                                .await
                                .map_err(tracerr::map_from_and_wrap!())?;
                            state.local_sdp.unapproved_set(sdp_answer);
                        }
                    }
                }
            }
            NegotiationPhase::Stable
            | NegotiationPhase::WaitLocalSdpApprove
            | NegotiationPhase::WaitRemoteSdp => (),
        }
        Ok(())
    }

    /// Watcher for the [`State::negotiation_role`] updates.
    ///
    /// Waits for [`sender::Component`]s' and [`receiver::Component`]s'
    /// creation/update, updates local `MediaStream` (if required) and
    /// renegotiates [`PeerConnection`].
    #[watch(self.negotiation_role.subscribe().filter_map(transpose_guarded))]
    async fn negotiation_role_changed(
        _: Rc<PeerConnection>,
        state: Rc<State>,
        role: Guarded<NegotiationRole>,
    ) {
        let (role, guard) = role.into_parts();
        match role {
            NegotiationRole::Offerer => {
                drop(guard);
                medea_reactive::when_all_processed(vec![
                    state.when_all_senders_processed().into(),
                    state.when_all_receivers_processed().into(),
                ])
                .await;

                medea_reactive::when_all_processed(vec![
                    state.senders.when_stabilized().into(),
                    state.receivers.when_stabilized().into(),
                    state.when_all_updated().into(),
                ])
                .await;
            }
            NegotiationRole::Answerer(remote_sdp) => {
                state.set_remote_sdp(remote_sdp);
                drop(guard);

                medea_reactive::when_all_processed(vec![
                    state.receivers.when_updated().into(),
                    state.senders.when_all_processed().into(),
                    state.remote_sdp.when_all_processed().into(),
                    state.senders.when_updated().into(),
                ])
                .await;

                medea_reactive::when_all_processed(vec![
                    state.senders.when_stabilized().into(),
                    state.receivers.when_stabilized().into(),
                ])
                .await;
            }
        }

        state.maybe_update_local_stream.set(true);
        _ = state.maybe_update_local_stream.when_eq(false).await;

        state.negotiation_phase.set(NegotiationPhase::WaitLocalSdp);
    }

    /// Watcher for the [`State::sync_phase`] updates.
    ///
    /// Sends [`PeerConnection`]'s connection state and ICE connection state to
    /// the server.
    #[watch(self.sync_phase.subscribe().skip(1))]
    fn sync_phase_changed(
        peer: &PeerConnection,
        _: &State,
        sync_phase: SyncPhase,
    ) {
        if sync_phase == SyncPhase::Synced {
            peer.send_current_connection_states();
        }
    }

    /// Watcher for the [`State::maybe_update_local_stream`] `true` updates.
    ///
    /// Waits for [`State::senders`] update and calls
    /// [`State::update_local_stream()`].
    #[watch(
        self.maybe_update_local_stream.subscribe().filter(|v| future::ready(*v))
    )]
    async fn maybe_local_stream_update_needed(
        peer: Rc<PeerConnection>,
        state: Rc<State>,
        _: bool,
    ) {
        state.senders.when_updated().await;
        drop(state.update_local_stream(&peer).await);

        state.maybe_update_local_stream.set(false);
    }
}
