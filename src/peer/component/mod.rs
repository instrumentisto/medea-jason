//! Implementation of a [`Component`].

mod ice_candidates;
mod local_sdp;
mod tracks_repository;
mod watchers;

use std::{cell::Cell, collections::HashSet, rc::Rc};

use futures::{StreamExt as _, TryFutureExt as _, future::LocalBoxFuture};
pub use local_sdp::DESCRIPTION_APPROVE_TIMEOUT;
use medea_client_api_proto::{
    self as proto, IceCandidate, IceServer, NegotiationRole, PeerId as Id,
    TrackId,
};
use medea_reactive::{AllProcessed, ObservableCell, ProgressableCell};
use proto::{ConnectionMode, MemberId};
use tracerr::Traced;

use self::{
    ice_candidates::IceCandidates, local_sdp::LocalSdp,
    tracks_repository::TracksRepository,
};
use crate::{
    media::LocalTracksConstraints,
    peer::{
        LocalStreamUpdateCriteria, PeerConnection, UpdateLocalStreamError,
        media::{receiver, sender},
    },
    utils::{AsProtoState, SynchronizableState, Updatable, component},
};

/// Possible synchronization phases of [`Component`]'s state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyncPhase {
    /// State is desynced and should be synced on RPC reconnection.
    Desynced,

    /// State syncs with a Media Server state.
    Syncing,

    /// State is synced.
    Synced,
}

/// Possible negotiation phases of a [`Component`].
///
/// ```ignore
///           +--------+
///           |        |
/// +-------->+ Stable +<----------+
/// |         |        |           |
/// |         +---+----+           |
/// |             |                |
/// |             v                |
/// |      +------+-------+        |
/// |      |              |        |
/// |      | WaitLocalSdp +<----+  |
/// |      |              |     |  |
/// |      +------+-------+     |  |
/// |             |             |  |
/// |             v             |  |
/// |  +----------+----------+  |  |
/// |  |                     |  |  |
/// +--+ WaitLocalSdpApprove +--+  |
///    |                     |     |
///    +----------+----------+     |
///               |                |
///               v                |
///       +-------+-------+        |
///       |               |        |
///       | WaitRemoteSdp |        |
///       |               |        |
///       +-------+-------+        |
///               |                |
///               |                |
///               +----------------+
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NegotiationPhase {
    /// [`Component`] is new or negotiation is completed.
    Stable,

    /// [`Component`] waits for a local SDP offer generating.
    WaitLocalSdp,

    /// [`Component`] waits for a local SDP being approved by server.
    WaitLocalSdpApprove,

    /// [`Component`] waits for a remote SDP offer.
    WaitRemoteSdp,
}

/// State of a [`Component`].
#[derive(Debug)]
pub struct State {
    /// ID of this [`Component`].
    id: Id,

    /// Indicator whether this `Peer` is working in a [P2P mesh] or [SFU] mode.
    ///
    /// [P2P mesh]: https://webrtcglossary.com/mesh
    /// [SFU]: https://webrtcglossary.com/sfu
    connection_mode: ConnectionMode,

    /// All [`sender::State`]s of this [`Component`].
    senders: TracksRepository<sender::State>,

    /// All [`receiver::State`]s of this [`Component`].
    receivers: TracksRepository<receiver::State>,

    /// Indicator whether this [`Component`] should relay all media through a
    /// TURN server forcibly.
    force_relay: bool,

    /// List of [`IceServer`]s which this [`Component`] should use.
    ice_servers: Vec<IceServer>,

    /// Current [`NegotiationRole`] of this [`Component`].
    negotiation_role: ProgressableCell<Option<NegotiationRole>>,

    /// [`NegotiationPhase`] of this [`Component`].
    negotiation_phase: ObservableCell<NegotiationPhase>,

    /// Local session description of this [`Component`].
    local_sdp: LocalSdp,

    /// Remote session description of this [`Component`].
    remote_sdp: ProgressableCell<Option<String>>,

    /// Indicates whether ICE restart should be performed.
    restart_ice: Cell<bool>,

    /// All [`IceCandidate`]s of this [`Component`].
    ice_candidates: IceCandidates,

    /// Indicator whether [`State::update_local_stream`] method should be
    /// called if some [`sender`] wants to update a local stream.
    maybe_update_local_stream: ObservableCell<bool>,

    /// Indicator whether there is some information about tracks to provide
    /// into [`Connections`].
    ///
    /// [`Connections`]: crate::connection::Connections
    maybe_update_connections:
        ObservableCell<Option<(TrackId, HashSet<MemberId>)>>,

    /// [`SyncPhase`] of this [`Component`].
    sync_phase: ObservableCell<SyncPhase>,
}

impl State {
    /// Creates a new [`State`] with the provided data.
    #[must_use]
    pub fn new(
        id: Id,
        ice_servers: Vec<IceServer>,
        force_relay: bool,
        negotiation_role: Option<NegotiationRole>,
        connection_mode: ConnectionMode,
    ) -> Self {
        Self {
            id,
            connection_mode,
            senders: TracksRepository::new(),
            receivers: TracksRepository::new(),
            ice_servers,
            force_relay,
            remote_sdp: ProgressableCell::new(None),
            local_sdp: LocalSdp::new(),
            negotiation_role: ProgressableCell::new(negotiation_role),
            negotiation_phase: ObservableCell::new(NegotiationPhase::Stable),
            restart_ice: Cell::new(false),
            ice_candidates: IceCandidates::new(),
            maybe_update_local_stream: ObservableCell::new(false),
            maybe_update_connections: ObservableCell::new(None),
            sync_phase: ObservableCell::new(SyncPhase::Synced),
        }
    }

    /// Returns [`ConnectionMode`] of this [`State`].
    #[must_use]
    pub const fn connection_mode(&self) -> ConnectionMode {
        self.connection_mode
    }

    /// Returns [`Id`] of this [`State`].
    #[must_use]
    pub const fn id(&self) -> Id {
        self.id
    }

    /// Returns all [`IceServer`]s of this [`State`].
    #[must_use]
    pub const fn ice_servers(&self) -> &Vec<IceServer> {
        &self.ice_servers
    }

    /// Indicates whether [`PeerConnection`] should be relayed forcibly.
    #[must_use]
    pub const fn force_relay(&self) -> bool {
        self.force_relay
    }

    /// Inserts a new [`sender::State`] into this [`State`].
    pub fn insert_sender(&self, track_id: TrackId, sender: Rc<sender::State>) {
        self.senders.insert(track_id, sender);
    }

    /// Inserts a new [`receiver::State`] into this [`State`].
    pub fn insert_receiver(
        &self,
        track_id: TrackId,
        receiver: Rc<receiver::State>,
    ) {
        self.receivers.insert(track_id, receiver);
    }

    /// Returns [`Rc`] to the [`sender::State`] with the provided [`TrackId`].
    #[must_use]
    pub fn get_sender(&self, track_id: TrackId) -> Option<Rc<sender::State>> {
        self.senders.get(track_id)
    }

    /// Returns [`Rc`] to the [`receiver::State`] with the provided [`TrackId`].
    #[must_use]
    pub fn get_receiver(
        &self,
        track_id: TrackId,
    ) -> Option<Rc<receiver::State>> {
        self.receivers.get(track_id)
    }

    /// Returns all the send [`TrackId`]s of the peer.
    pub fn get_send_tracks(&self) -> Vec<TrackId> {
        self.senders.ids()
    }

    /// Returns all the receive [`TrackId`]s of the peer.
    pub fn get_recv_tracks(&self) -> Vec<TrackId> {
        self.receivers.ids()
    }

    /// Returns all the [`TrackId`]s (sand and receiver) of the peer.
    pub fn get_tracks(&self) -> Vec<TrackId> {
        self.get_send_tracks()
            .into_iter()
            .chain(self.get_recv_tracks())
            .collect()
    }

    /// Sets [`NegotiationRole`] of this [`State`] to the provided one.
    pub async fn set_negotiation_role(
        &self,
        negotiation_role: NegotiationRole,
    ) {
        _ = self
            .negotiation_role
            .subscribe()
            .any(async |val| val.is_none())
            .await;
        self.negotiation_role.set(Some(negotiation_role));
    }

    /// Sets [`State::restart_ice`] to `true`.
    pub fn restart_ice(&self) {
        self.restart_ice.set(true);
    }

    /// Removes [`sender::State`] or [`receiver::State`] with the provided
    /// [`TrackId`].
    pub fn remove_track(&self, track_id: TrackId) {
        if !self.receivers.remove(track_id) {
            _ = self.senders.remove(track_id);
        }
    }

    /// Sets remote SDP offer to the provided value.
    pub fn set_remote_sdp(&self, sdp: String) {
        self.remote_sdp.set(Some(sdp));
    }

    /// Adds [`IceCandidate`] for the [`State`].
    pub fn add_ice_candidate(&self, ice_candidate: IceCandidate) {
        self.ice_candidates.add(ice_candidate);
    }

    /// Marks current local SDP as approved by server.
    pub fn apply_local_sdp(&self, sdp: String) {
        self.local_sdp.approved_set(sdp);
    }

    /// Stops all timeouts of the [`State`].
    ///
    /// Stops local SDP rollback timeout.
    pub fn stop_timeouts(&self) {
        self.local_sdp.stop_timeout();
    }

    /// Resumes all timeouts of the [`State`].
    ///
    /// Resumes local SDP rollback timeout.
    pub fn resume_timeouts(&self) {
        self.local_sdp.resume_timeout();
    }

    /// Returns [`Future`] resolving once
    /// [getUserMedia()][1]/[getDisplayMedia()][2] request for the provided
    /// [`TrackId`]s is resolved.
    ///
    /// [`Result`] returned by this [`Future`] will be the same as the result of
    /// the [getUserMedia()][1]/[getDisplayMedia()][2] request.
    ///
    /// Returns last known [getUserMedia()][1]/[getDisplayMedia()][2] request's
    /// [`Result`], if currently no such requests are running for the provided
    /// [`TrackId`]s.
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    /// [2]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
    pub fn local_stream_update_result(
        &self,
        tracks_ids: HashSet<TrackId>,
    ) -> LocalBoxFuture<'static, Result<(), Traced<UpdateLocalStreamError>>>
    {
        Box::pin(
            self.senders
                .local_stream_update_result(tracks_ids)
                .map_err(tracerr::map_from_and_wrap!()),
        )
    }

    /// Returns [`Future`] resolving when all [`sender::State`]'s and
    /// [`receiver::State`]'s updates will be applied.
    pub fn when_all_updated(&self) -> AllProcessed<'static> {
        medea_reactive::when_all_processed(vec![
            self.senders.when_updated().into(),
            self.receivers.when_updated().into(),
        ])
    }

    /// Updates a local `MediaStream` based on a
    /// [`sender::State::is_local_stream_update_needed`].
    ///
    /// Resets a [`sender::State`] local stream update when it's updated.
    async fn update_local_stream(
        &self,
        peer: &Rc<PeerConnection>,
    ) -> Result<(), Traced<UpdateLocalStreamError>> {
        let mut criteria = LocalStreamUpdateCriteria::empty();
        let senders = self.senders.get_outdated();
        for s in &senders {
            criteria.add(s.media_kind(), s.media_source());
        }
        let res = peer
            .update_local_stream(criteria)
            .await
            .map_err(tracerr::map_from_and_wrap!())
            .map(drop);
        for s in senders {
            if let Err(err) = res.clone() {
                s.failed_local_stream_update(err);
            } else {
                s.local_stream_updated();
            }
        }
        res
    }

    /// Inserts the provided [`proto::Track`] to this [`State`].
    pub fn insert_track(
        &self,
        track: &proto::Track,
        send_constraints: LocalTracksConstraints,
    ) {
        match &track.direction {
            proto::Direction::Send { receivers, mid } => {
                self.senders.insert(
                    track.id,
                    Rc::new(sender::State::new(
                        track.id,
                        mid.clone(),
                        track.media_type.clone(),
                        track.media_direction,
                        track.muted,
                        receivers.clone(),
                        send_constraints,
                        self.connection_mode,
                    )),
                );
            }
            proto::Direction::Recv { sender, mid } => {
                self.receivers.insert(
                    track.id,
                    Rc::new(receiver::State::new(
                        track.id,
                        mid.clone(),
                        track.media_type.clone(),
                        track.media_direction,
                        track.muted,
                        sender.clone(),
                        self.connection_mode,
                    )),
                );
            }
        }
    }

    /// Returns [`Future`] resolving once all senders inserts and removes are
    /// processed.
    pub fn when_all_senders_processed(&self) -> AllProcessed<'static> {
        self.senders.when_all_processed()
    }

    /// Returns [`Future`] resolving once all [`State::receivers`]' inserts and
    /// removes are processed.
    fn when_all_receivers_processed(&self) -> AllProcessed<'static> {
        self.receivers.when_all_processed()
    }

    /// Patches [`sender::State`] or [`receiver::State`] with the provided
    /// [`proto::TrackPatchEvent`].
    ///
    /// Schedules a local stream update.
    pub async fn patch_track(&self, patch: proto::TrackPatchEvent) {
        if let Some(receivers) = &patch.receivers {
            _ = self.maybe_update_connections.when_eq(None).await;
            self.maybe_update_connections
                .set(Some((patch.id, receivers.clone().into_iter().collect())));
        }

        if let Some(sender) = self.get_sender(patch.id) {
            sender.update(patch);
            _ = self.maybe_update_local_stream.when_eq(false).await;
            self.maybe_update_local_stream.set(true);
        } else if let Some(receiver) = self.get_receiver(patch.id) {
            receiver.update(&patch);
        } else {
            log::warn!("Cannot apply patch to `Track`: {}", patch.id.0);
        }
    }

    /// Returns the current SDP offer of this [`State`].
    #[must_use]
    pub fn current_sdp_offer(&self) -> Option<String> {
        self.local_sdp.current()
    }
}

/// Component responsible for a [`PeerConnection`] updating.
pub type Component = component::Component<State, PeerConnection>;

impl AsProtoState for State {
    type Output = proto::state::Peer;

    fn as_proto(&self) -> Self::Output {
        Self::Output {
            id: self.id,
            connection_mode: self.connection_mode,
            senders: self.senders.as_proto(),
            receivers: self.receivers.as_proto(),
            ice_candidates: self.ice_candidates.as_proto(),
            force_relay: self.force_relay,
            ice_servers: self.ice_servers.clone(),
            negotiation_role: self.negotiation_role.get(),
            local_sdp: self.local_sdp.current(),
            remote_sdp: self.remote_sdp.get(),
            restart_ice: self.restart_ice.get(),
        }
    }
}

impl SynchronizableState for State {
    type Input = proto::state::Peer;

    fn from_proto(
        input: Self::Input,
        send_constraints: &LocalTracksConstraints,
    ) -> Self {
        let state = Self::new(
            input.id,
            input.ice_servers,
            input.force_relay,
            input.negotiation_role,
            input.connection_mode,
        );

        #[expect(clippy::iter_over_hash_type, reason = "order doesn't matter")]
        for (id, sender) in input.senders {
            if !sender.receivers.is_empty() {
                state.senders.insert(
                    id,
                    Rc::new(sender::State::from_proto(
                        sender,
                        send_constraints,
                    )),
                );
            }
        }
        #[expect(clippy::iter_over_hash_type, reason = "order doesn't matter")]
        for (id, receiver) in input.receivers {
            state.receivers.insert(
                id,
                Rc::new(receiver::State::from_proto(
                    receiver,
                    send_constraints,
                )),
            );
        }
        #[expect(clippy::iter_over_hash_type, reason = "order doesn't matter")]
        for ice_candidate in input.ice_candidates {
            state.ice_candidates.add(ice_candidate);
        }

        state
    }

    fn apply(&self, input: Self::Input, send_cons: &LocalTracksConstraints) {
        if input.negotiation_role.is_some() {
            self.negotiation_role.set(input.negotiation_role);
        }
        if input.restart_ice {
            self.restart_ice.set(true);
        }
        if let Some(sdp_offer) = input.local_sdp {
            self.local_sdp.approved_set(sdp_offer);
        } else {
            self.negotiation_phase.set(NegotiationPhase::WaitLocalSdp);
        }
        self.remote_sdp.set(input.remote_sdp);
        self.ice_candidates.apply(input.ice_candidates, send_cons);
        self.senders.apply(input.senders, send_cons);
        self.receivers.apply(input.receivers, send_cons);

        self.sync_phase.set(SyncPhase::Synced);
    }
}

impl Updatable for State {
    fn when_stabilized(&self) -> AllProcessed<'static> {
        medea_reactive::when_all_processed(vec![
            self.senders.when_stabilized().into(),
            self.receivers.when_stabilized().into(),
        ])
    }

    fn when_updated(&self) -> AllProcessed<'static> {
        medea_reactive::when_all_processed(vec![
            self.receivers.when_updated().into(),
            self.senders.when_updated().into(),
        ])
    }

    fn connection_lost(&self) {
        self.sync_phase.set(SyncPhase::Desynced);
        self.senders.connection_lost();
        self.receivers.connection_lost();
    }

    fn connection_recovered(&self) {
        self.sync_phase.set(SyncPhase::Syncing);
        self.senders.connection_recovered();
        self.receivers.connection_recovered();
    }
}

#[cfg(feature = "mockable")]
// TODO: Try remove on next Rust version upgrade.
#[expect(clippy::allow_attributes, reason = "`#[expect]` is not considered")]
#[allow(clippy::multiple_inherent_impl, reason = "feature gated")]
impl State {
    /// Waits for a [`State::remote_sdp`] change to be applied.
    pub async fn when_remote_sdp_processed(&self) {
        self.remote_sdp.when_all_processed().await;
    }

    /// Resets a [`NegotiationRole`] of this [`State`] to [`None`].
    pub fn reset_negotiation_role(&self) {
        self.negotiation_phase.set(NegotiationPhase::Stable);
        self.negotiation_role.set(None);
    }

    /// Returns the current [`NegotiationRole`] of this [`State`].
    #[must_use]
    pub fn negotiation_role(&self) -> Option<NegotiationRole> {
        self.negotiation_role.get()
    }

    /// Returns a [`Future`] resolving once local SDP approve is needed.
    pub fn when_local_sdp_approve_needed(
        &self,
    ) -> impl Future<Output = ()> + use<> {
        use futures::FutureExt as _;

        self.negotiation_phase
            .when_eq(NegotiationPhase::WaitLocalSdpApprove)
            .map(drop)
    }

    /// Stabilizes all [`receiver::State`]s of this [`State`].
    pub fn stabilize_all(&self) {
        self.receivers.stabilize_all();
    }

    /// Waits until a [`State::local_sdp`] is resolved and returns its new
    /// value.
    pub async fn when_local_sdp_updated(&self) -> Option<String> {
        use futures::StreamExt as _;

        self.local_sdp.subscribe().skip(1).next().await.flatten()
    }

    /// Waits until all [`State::senders`]' and [`State::receivers`]' inserts
    /// are processed.
    pub async fn when_all_tracks_created(&self) {
        medea_reactive::when_all_processed(vec![
            self.senders.when_insert_processed().into(),
            self.receivers.when_insert_processed().into(),
        ])
        .await;
    }

    /// Sets [`State::sync_phase`] to the [`SyncPhase::Synced`].
    pub fn synced(&self) {
        self.senders.synced();
        self.receivers.synced();
        self.sync_phase.set(SyncPhase::Synced);
    }
}
