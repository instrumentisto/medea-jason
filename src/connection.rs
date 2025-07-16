//! [`Connection`] with a specific remote `Member`.

use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    rc::{Rc, Weak},
};

use derive_more::with_trait::{Display, From};
use futures::{
    FutureExt as _, StreamExt as _, future, future::LocalBoxFuture,
    stream::LocalBoxStream,
};
use medea_client_api_proto::{
    self as proto, ConnectionQualityScore, MemberId, PeerConnectionState,
    TrackId,
};
use tracerr::Traced;

use crate::{
    api,
    media::{MediaKind, MediaSourceKind, RecvConstraints, track::remote},
    peer::{
        MediaState, MediaStateControllable as _, ProhibitedStateError,
        TransceiverSide as _, media_exchange_state, receiver,
    },
    platform,
    utils::{Caused, TaskHandle},
};

/// Errors occurring when changing media state of [`Sender`]s and [`Receiver`]s.
///
/// [`Sender`]: crate::peer::media::Sender
/// [`Receiver`]: crate::peer::media::Receiver
#[derive(Caused, Clone, Copy, Debug, Display, From)]
#[cause(error = platform::Error)]
pub enum ChangeMediaStateError {
    /// [`ConnectionHandle`]'s [`Weak`] pointer is detached.
    #[display("`ConnectionHandle` is in detached state")]
    Detached,

    /// [`MediaState`] of a [`Sender`] transits to an opposite of the requested
    /// one.
    ///
    /// [`Sender`]: crate::peer::media::Sender
    #[display(
        "`MediaState` transits to opposite ({_0}) of the requested \
         `MediaExchangeState`"
    )]
    TransitionIntoOppositeState(MediaState),

    /// Requested state transition is not allowed by [`Sender`]'s settings.
    ///
    /// [`Sender`]: crate::peer::media::Sender
    ProhibitedState(ProhibitedStateError),
}

/// Alias of [`Result`]s related to [`MediaState`] update functions.
type ChangeMediaStateResult = Result<(), Traced<ChangeMediaStateError>>;

/// Service which manages [`Connection`]s with remote `Member`s.
#[derive(Debug)]
pub struct Connections {
    /// [`TrackId`] to remote [`MemberId`].
    tracks_to_members: RefCell<HashMap<TrackId, HashSet<MemberId>>>,

    /// Remote [`MemberId`] to [`TrackId`].
    members_to_tracks: RefCell<HashMap<MemberId, HashSet<TrackId>>>,

    /// Remote [`MemberId`] to [`Connection`] with that `Member`.
    members_to_conns: RefCell<HashMap<MemberId, Connection>>,

    /// Global constraints to the [`remote::Track`]s of the Jason.
    room_recv_constraints: Rc<RecvConstraints>,

    /// Callback invoked on remote `Member` media arrival.
    on_new_connection: platform::Callback<api::ConnectionHandle>,
}

impl Connections {
    /// Creates new [`Connections`].
    pub fn new(room_recv_constraints: Rc<RecvConstraints>) -> Self {
        Self {
            tracks_to_members: RefCell::default(),
            members_to_tracks: RefCell::default(),
            members_to_conns: RefCell::default(),
            room_recv_constraints,
            on_new_connection: platform::Callback::default(),
        }
    }

    /// Sets callback, which will be invoked when new [`Connection`] is
    /// established.
    pub fn on_new_connection(
        &self,
        f: platform::Function<api::ConnectionHandle>,
    ) {
        self.on_new_connection.set_func(f);
    }

    /// Adds or updates information about related [`Track`]s with the provided
    /// [`TrackId`] and [`MemberId`]s. Then [`Connections`] decides to create or
    /// to delete [`Connection`]s.
    ///
    /// Returns [`Connection`]s associated with the provided [`MemberId`]s.
    ///
    /// [`Track`]: medea_client_api_proto::Track
    #[must_use]
    pub fn update_connections(
        &self,
        track_id: &TrackId,
        partner_members: HashSet<MemberId>,
    ) -> Vec<Connection> {
        if let Some(partners) =
            self.tracks_to_members.borrow_mut().get_mut(track_id)
        {
            let mut connections = self.members_to_conns.borrow_mut();
            let mut members_to_tracks = self.members_to_tracks.borrow_mut();

            // No changes.
            if partners == &partner_members {
                return partners
                    .iter()
                    .filter_map(|partner| {
                        _ = members_to_tracks
                            .get_mut(partner)
                            .map(|tracks| tracks.insert(*track_id));
                        connections.get(partner).cloned()
                    })
                    .collect();
            }

            // Adding new.
            let added: Vec<_> =
                partner_members.difference(partners).cloned().collect();
            for mid in added {
                _ = members_to_tracks
                    .entry(mid.clone())
                    .or_default()
                    .insert(*track_id);

                if !connections.contains_key(&mid) {
                    let connection = Connection::new(
                        mid.clone(),
                        &self.room_recv_constraints,
                    );
                    self.on_new_connection.call1(connection.new_handle());
                    drop(connections.insert(mid.clone(), connection));
                }
                _ = partners.insert(mid);
            }

            // Deleting.
            partners.retain(|partner| {
                let to_remove = !partner_members.contains(partner);

                if to_remove {
                    if let Some(tracks) = members_to_tracks.get_mut(partner) {
                        _ = tracks.remove(track_id);

                        if tracks.is_empty() {
                            _ = connections
                                .remove(partner)
                                .map(|conn| conn.0.on_close.call0());
                        }
                    }
                }

                !to_remove
            });

            return partner_members
                .into_iter()
                .filter_map(|partner| connections.get(&partner).cloned())
                .collect();
        }

        self.add_connections(*track_id, &partner_members)
    }

    /// Adds information about related [`Track`]s with the provided [`TrackId`]
    /// and [`MemberId`]s, and creates [`Connection`]s.
    ///
    /// Returns [`Connection`]s associated with the provided [`MemberId`]s.
    ///
    /// [`Track`]: medea_client_api_proto::Track
    #[must_use]
    fn add_connections(
        &self,
        track_id: TrackId,
        partner_members: &HashSet<MemberId>,
    ) -> Vec<Connection> {
        let mut connections = self.members_to_conns.borrow_mut();

        #[expect(clippy::iter_over_hash_type, reason = "order doesn't matter")]
        for partner in partner_members {
            _ = self
                .members_to_tracks
                .borrow_mut()
                .entry(partner.clone())
                .or_default()
                .insert(track_id);
            if !connections.contains_key(partner) {
                let connection = Connection::new(
                    partner.clone(),
                    &self.room_recv_constraints,
                );
                self.on_new_connection.call1(connection.new_handle());
                drop(connections.insert(partner.clone(), connection));
            }
        }

        drop(
            self.tracks_to_members.borrow_mut().insert(
                track_id,
                partner_members.clone().into_iter().collect(),
            ),
        );

        partner_members
            .iter()
            .filter_map(|p| connections.get(p).cloned())
            .collect()
    }

    /// Removes information about [`Track`] with the provided [`TrackId`]. Then
    /// [`Connections`] can decide to delete the related [`Connection`].
    ///
    /// [`Track`]: medea_client_api_proto::Track
    pub fn remove_track(&self, track_id: &TrackId) {
        let mut tracks = self.tracks_to_members.borrow_mut();

        if let Some(partners) = tracks.remove(track_id) {
            #[expect(clippy::iter_over_hash_type, reason = "doesn't matter")]
            for p in partners {
                if let Some(member_tracks) =
                    self.members_to_tracks.borrow_mut().get_mut(&p)
                {
                    _ = member_tracks.remove(track_id);

                    if member_tracks.is_empty() {
                        _ = self
                            .members_to_conns
                            .borrow_mut()
                            .remove(&p)
                            .map(|conn| conn.0.on_close.call0());
                    }
                }
            }
        }
    }

    /// Lookups a [`Connection`] by the provided remote [`MemberId`].
    #[must_use]
    pub fn get(&self, remote_member_id: &MemberId) -> Option<Connection> {
        self.members_to_conns.borrow().get(remote_member_id).cloned()
    }

    /// Iterates over all the [`Connection`]s of the provided [`TrackId`].
    pub fn iter_by_track(
        &self,
        track_id: &TrackId,
    ) -> impl Iterator<Item = Connection> + use<'_> {
        self.tracks_to_members
            .borrow()
            .get(track_id)
            .cloned()
            .into_iter()
            .flat_map(|member_ids| {
                member_ids.into_iter().filter_map(|member_id| {
                    self.members_to_conns.borrow().get(&member_id).cloned()
                })
            })
    }

    /// Updates this [`Connection`] with the provided [`proto::state::Room`].
    pub fn apply(&self, new_state: &proto::state::Room) {
        #[expect(clippy::iter_over_hash_type, reason = "order doesn't matter")]
        for peer in new_state.peers.values() {
            for (track_id, sender) in &peer.senders {
                if let Some(partners) =
                    self.tracks_to_members.borrow().get(track_id)
                {
                    for member in partners {
                        if let Some(member_tracks) =
                            self.members_to_tracks.borrow_mut().get_mut(member)
                        {
                            if !sender.receivers.contains(member) {
                                _ = member_tracks.remove(track_id);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Error of [`ConnectionHandle`]'s [`Weak`] pointer being detached.
#[derive(Caused, Clone, Copy, Debug, Display)]
#[cause(error = platform::Error)]
#[display("`ConnectionHandle` is in detached state")]
pub struct HandleDetachedError;

/// External handler to a [`Connection`] with a remote `Member`.
///
/// Actually, represents a [`Weak`]-based handle to `InnerConnection`.
#[derive(Clone, Debug)]
pub struct ConnectionHandle(Weak<InnerConnection>);

/// Estimated [`Connection`]'s quality on the client side only.
#[derive(Clone, Copy, Debug, Display, Eq, From, Ord, PartialEq, PartialOrd)]
pub enum ClientConnectionQualityScore {
    /// [`Connection`] is lost.
    Disconnected,

    /// [`Connection`] is established and scored.
    Connected(ConnectionQualityScore),
}

impl ClientConnectionQualityScore {
    /// Converts this [`ClientConnectionQualityScore`] into a [`u8`] number.
    #[must_use]
    pub const fn into_u8(self) -> u8 {
        match self {
            Self::Disconnected => 0,
            // TODO: Replace with derive?
            #[expect(clippy::as_conversions, reason = "needs refactoring")]
            Self::Connected(score) => score as u8,
        }
    }
}

/// Actual data of a connection with a specific remote `Member`.
///
/// Shared between external [`ConnectionHandle`] and Rust side [`Connection`].
#[derive(Debug)]
struct InnerConnection {
    /// Remote `Member` ID.
    remote_id: MemberId,

    /// Current [`ConnectionQualityScore`] of this [`Connection`].
    quality_score: Cell<Option<ConnectionQualityScore>>,

    /// Current [`ClientConnectionQualityScore`] of this [`Connection`].
    client_quality_score: Cell<Option<ClientConnectionQualityScore>>,

    /// Current [`PeerConnectionState`] of this [`Connection`].
    peer_state: Cell<Option<PeerConnectionState>>,

    /// Callback invoked when a [`remote::Track`] is received.
    on_remote_track_added: platform::Callback<api::RemoteMediaTrack>,

    /// Individual [`RecvConstraints`] of this [`Connection`].
    recv_constraints: Rc<RecvConstraints>,

    /// All [`receiver::State`]s related to this [`InnerConnection`].
    receivers: RefCell<Vec<Rc<receiver::State>>>,

    /// Callback invoked when a [`ConnectionQualityScore`] is updated.
    on_quality_score_update: platform::Callback<u8>,

    /// Callback invoked when this [`Connection`] is closed.
    on_close: platform::Callback<()>,

    /// [`TaskHandle`]s for the spawned changes listeners of this
    /// [`Connection`].
    _task_handles: Vec<TaskHandle>,
}

impl InnerConnection {
    /// Changes [`MediaState`] of the provided [`MediaKind`] to the provided
    /// [`MediaState`].
    ///
    /// # Errors
    ///
    /// With [`ChangeMediaStateError::TransitionIntoOppositeState`] if this
    /// function with opposite [`MediaState`] was called or a media server
    /// didn't approve this state transition.
    async fn change_media_state(
        &self,
        desired_state: MediaState,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
    ) -> ChangeMediaStateResult {
        let receivers = self.receivers.borrow().clone();
        let mut change_tasks = Vec::new();
        for r in receivers {
            let source_filter =
                source_kind.is_none_or(|skind| skind == r.source_kind().into());

            if r.is_subscription_needed(desired_state)
                && r.kind() == kind
                && source_filter
            {
                r.media_state_transition_to(desired_state)
                    .map_err(tracerr::map_from_and_wrap!())?;
                change_tasks.push(r.when_media_state_stable(desired_state));
            }
        }

        drop(
            future::try_join_all(change_tasks)
                .await
                .map_err(tracerr::from_and_wrap!())?,
        );

        if let MediaState::MediaExchange(desired_state) = desired_state {
            self.recv_constraints.set_enabled(
                desired_state == media_exchange_state::Stable::Enabled,
                kind,
                source_kind.map(Into::into),
            );
        }

        Ok(())
    }
}

impl ConnectionHandle {
    /// Sets callback, invoked when this `Connection` will close.
    ///
    /// # Errors
    ///
    /// See [`HandleDetachedError`] for details.
    pub fn on_close(
        &self,
        f: platform::Function<()>,
    ) -> Result<(), Traced<HandleDetachedError>> {
        self.0
            .upgrade()
            .ok_or_else(|| tracerr::new!(HandleDetachedError))
            .map(|inner| inner.on_close.set_func(f))
    }

    /// Returns remote `Member` ID.
    ///
    /// # Errors
    ///
    /// See [`HandleDetachedError`] for details.
    pub fn get_remote_member_id(
        &self,
    ) -> Result<String, Traced<HandleDetachedError>> {
        self.0
            .upgrade()
            .ok_or_else(|| tracerr::new!(HandleDetachedError))
            .map(|inner| inner.remote_id.0.clone())
    }

    /// Sets callback, invoked when a new [`remote::Track`] is added to this
    /// [`Connection`].
    ///
    /// # Errors
    ///
    /// See [`HandleDetachedError`] for details.
    pub fn on_remote_track_added(
        &self,
        f: platform::Function<api::RemoteMediaTrack>,
    ) -> Result<(), Traced<HandleDetachedError>> {
        self.0
            .upgrade()
            .ok_or_else(|| tracerr::new!(HandleDetachedError))
            .map(|inner| inner.on_remote_track_added.set_func(f))
    }

    /// Sets callback, invoked when a connection quality score is updated by
    /// a server.
    ///
    /// # Errors
    ///
    /// See [`HandleDetachedError`] for details.
    pub fn on_quality_score_update(
        &self,
        f: platform::Function<u8>,
    ) -> Result<(), Traced<HandleDetachedError>> {
        self.0
            .upgrade()
            .ok_or_else(|| tracerr::new!(HandleDetachedError))
            .map(|inner| inner.on_quality_score_update.set_func(f))
    }

    /// Enables inbound video in this [`Connection`].
    ///
    /// # Errors
    ///
    /// With [`ChangeMediaStateError::Detached`] if an inner [`Weak`] pointer
    /// upgrade fails.
    ///
    /// With [`ChangeMediaStateError::TransitionIntoOppositeState`] if
    /// [`ConnectionHandle::disable_remote_video()`] was called while enabling
    /// or a media server didn't approve this state transition.
    pub fn enable_remote_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static + use<> {
        self.change_media_state(
            media_exchange_state::Stable::Enabled.into(),
            MediaKind::Video,
            source_kind,
        )
    }

    /// Disables inbound video in this [`Connection`].
    ///
    /// # Errors
    ///
    /// With [`ChangeMediaStateError::Detached`] if an inner [`Weak`] pointer
    /// upgrade fails.
    ///
    /// With [`ChangeMediaStateError::TransitionIntoOppositeState`] if
    /// [`ConnectionHandle::enable_remote_video()`] was called while disabling
    /// or a media server didn't approve this state transition.
    pub fn disable_remote_video(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static + use<> {
        self.change_media_state(
            media_exchange_state::Stable::Disabled.into(),
            MediaKind::Video,
            source_kind,
        )
    }

    /// Enables inbound audio in this [`Connection`].
    ///
    /// # Errors
    ///
    /// With [`ChangeMediaStateError::Detached`] if an inner [`Weak`] pointer
    /// upgrade fails.
    ///
    /// With [`ChangeMediaStateError::TransitionIntoOppositeState`] if
    /// [`ConnectionHandle::disable_remote_audio()`] was called while enabling
    /// or a media server didn't approve this state transition.
    pub fn enable_remote_audio(
        &self,
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static + use<> {
        self.change_media_state(
            media_exchange_state::Stable::Enabled.into(),
            MediaKind::Audio,
            None,
        )
    }

    /// Disables inbound audio in this [`Connection`].
    ///
    /// # Errors
    ///
    /// With [`ChangeMediaStateError::Detached`] if an inner [`Weak`] pointer
    /// upgrade fails.
    ///
    /// With [`ChangeMediaStateError::TransitionIntoOppositeState`] if
    /// [`ConnectionHandle::enable_remote_audio()`] was called while disabling
    /// or a media server didn't approve this state transition.
    pub fn disable_remote_audio(
        &self,
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static + use<> {
        self.change_media_state(
            media_exchange_state::Stable::Disabled.into(),
            MediaKind::Audio,
            None,
        )
    }

    /// Changes [`MediaState`] of the provided [`MediaKind`], [`TrackDirection`]
    /// and [`MediaSourceKind`] to the provided [`MediaState`].
    ///
    /// Helper function for all the exported enable/disable receive audio/video
    /// methods.
    fn change_media_state(
        &self,
        desired_state: MediaState,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
    ) -> LocalBoxFuture<'static, ChangeMediaStateResult> {
        let inner = self
            .0
            .upgrade()
            .ok_or_else(|| tracerr::new!(ChangeMediaStateError::Detached));
        let inner = match inner {
            Ok(inner) => inner,
            Err(e) => return Box::pin(future::err(e)),
        };

        Box::pin(async move {
            inner.change_media_state(desired_state, kind, source_kind).await
        })
    }
}

/// Connection with a specific remote `Member`, that is used on Rust side.
#[derive(Clone, Debug)]
pub struct Connection(Rc<InnerConnection>);

impl Connection {
    /// Instantiates a new [`Connection`] for the given `Member`.
    ///
    /// Based on the provided [`RecvConstraints`] individual [`RecvConstraints`]
    /// of this [`Connection`] will automatically synchronize.
    #[must_use]
    pub fn new(
        remote_id: MemberId,
        room_recv_constraints: &Rc<RecvConstraints>,
    ) -> Self {
        // Clone initial incoming media constraints.
        let recv_constraints = Rc::new(room_recv_constraints.as_ref().clone());

        Self(Rc::new(InnerConnection {
            _task_handles: vec![
                Self::spawn_constraints_synchronizer(
                    Rc::clone(&recv_constraints),
                    room_recv_constraints.on_video_device_enabled_change(),
                    MediaKind::Video,
                    MediaSourceKind::Device,
                ),
                Self::spawn_constraints_synchronizer(
                    Rc::clone(&recv_constraints),
                    room_recv_constraints.on_video_display_enabled_change(),
                    MediaKind::Video,
                    MediaSourceKind::Display,
                ),
                Self::spawn_constraints_synchronizer(
                    Rc::clone(&recv_constraints),
                    room_recv_constraints.on_audio_device_enabled_change(),
                    MediaKind::Audio,
                    MediaSourceKind::Device,
                ),
                Self::spawn_constraints_synchronizer(
                    Rc::clone(&recv_constraints),
                    room_recv_constraints.on_audio_display_enabled_change(),
                    MediaKind::Audio,
                    MediaSourceKind::Display,
                ),
            ],
            remote_id,
            quality_score: Cell::default(),
            client_quality_score: Cell::default(),
            peer_state: Cell::default(),
            on_quality_score_update: platform::Callback::default(),
            recv_constraints,
            on_close: platform::Callback::default(),
            on_remote_track_added: platform::Callback::default(),
            receivers: RefCell::default(),
        }))
    }

    /// Spawns synchronizer for the individual [`RecvConstraints`].
    ///
    /// When global [`RecvConstraints`] updated, then all individual
    /// [`RecvConstraints`] are going to the same state.
    ///
    /// Returns [`TaskHandle`] for the spawned changes listener.
    fn spawn_constraints_synchronizer(
        recv_constraints: Rc<RecvConstraints>,
        mut changes_stream: LocalBoxStream<'static, bool>,
        kind: MediaKind,
        source_kind: MediaSourceKind,
    ) -> TaskHandle {
        let (fut, abort) = future::abortable(async move {
            while let Some(is_enabled) = changes_stream.next().await {
                recv_constraints.set_enabled(
                    is_enabled,
                    kind,
                    Some(source_kind.into()),
                );
            }
        });
        platform::spawn(fut.map(drop));

        TaskHandle::from(abort)
    }

    /// Stores provided [`receiver::State`] in this [`Connection`].
    ///
    /// Updates [`MediaExchangeState`] of the provided [`receiver::State`] based
    /// on the current individual [`RecvConstraints`] of this [`Connection`].
    ///
    /// [`MediaExchangeState`]: crate::peer::MediaExchangeState
    pub fn add_receiver(&self, receiver: Rc<receiver::State>) {
        let enabled_in_cons = match &receiver.kind() {
            MediaKind::Audio => {
                self.0.recv_constraints.is_audio_device_enabled()
                    || self.0.recv_constraints.is_audio_display_enabled()
            }
            MediaKind::Video => {
                self.0.recv_constraints.is_video_device_enabled()
                    || self.0.recv_constraints.is_video_display_enabled()
            }
        };
        receiver
            .media_exchange_state_controller()
            .transition_to(enabled_in_cons.into());

        self.0.receivers.borrow_mut().push(receiver);
    }

    /// Invokes `on_remote_track_added` callback with the provided
    /// [`remote::Track`].
    pub fn add_remote_track(&self, track: remote::Track) {
        self.0.on_remote_track_added.call1(track);
    }

    /// Creates a new external handle to this [`Connection`].
    #[must_use]
    pub fn new_handle(&self) -> ConnectionHandle {
        ConnectionHandle(Rc::downgrade(&self.0))
    }

    /// Updates the [`ConnectionQualityScore`] of this [`Connection`].
    pub fn update_quality_score(&self, score: ConnectionQualityScore) {
        if self.0.quality_score.replace(Some(score)) == Some(score) {
            return;
        }

        self.refresh_client_conn_quality_score();
    }

    /// Updates the [`PeerConnectionState`] of this [`Connection`].
    pub fn update_peer_state(&self, state: PeerConnectionState) {
        if self.0.peer_state.replace(Some(state)) == Some(state) {
            return;
        }

        self.refresh_client_conn_quality_score();
    }

    /// Refreshes the [`ClientConnectionQualityScore`] of this [`Connection`].
    fn refresh_client_conn_quality_score(&self) {
        use PeerConnectionState as S;

        let state = self.0.peer_state.get();
        let quality_score = self.0.quality_score.get();
        let score = match (state, quality_score) {
            (Some(S::Connected), Some(quality_score)) => quality_score.into(),
            (Some(S::Disconnected | S::Failed | S::Closed), _) => {
                ClientConnectionQualityScore::Disconnected
            }
            (Some(S::Connecting | S::New) | None, _)
            | (Some(S::Connected), None) => return,
        };

        let is_score_changed =
            self.0.client_quality_score.replace(Some(score)) != Some(score);
        if is_score_changed {
            self.0.on_quality_score_update.call1(score.into_u8());
        }
    }
}
