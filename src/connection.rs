//! [`Connection`] with a specific remote `Member`.

use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    future::Future,
    rc::{Rc, Weak},
};

use derive_more::{Display, From};
use futures::{
    future, future::LocalBoxFuture, stream::LocalBoxStream, FutureExt as _,
    StreamExt as _,
};
use medea_client_api_proto::{ConnectionQualityScore, MemberId, TrackId};
use tracerr::Traced;

use crate::{
    api,
    media::{track::remote, MediaKind, MediaSourceKind, RecvConstraints},
    peer::{
        media_exchange_state, receiver, MediaState, MediaStateControllable,
        ProhibitedStateError, TransceiverSide,
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
    #[display(fmt = "ConnectionHandle is in detached state")]
    Detached,

    /// [`MediaState`] of a [`Sender`] transits to an opposite of the requested
    /// one.
    ///
    /// [`Sender`]: crate::peer::media::Sender
    #[display(
        fmt = "MediaState transits to opposite ({}) of the \
               requested MediaExchangeState",
        _0
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
    tracks: RefCell<HashMap<TrackId, HashSet<MemberId>>>,

    /// Remote [`MemberId`] to [`TrackId`].
    members_to_tracks: RefCell<HashMap<MemberId, HashSet<TrackId>>>,

    /// Remote [`MemberId`] to [`Connection`] with that `Member`.
    connections: RefCell<HashMap<MemberId, Connection>>,

    /// Global constraints to the [`remote::Track`]s of the Jason.
    room_recv_constraints: Rc<RecvConstraints>,

    /// Callback invoked on remote `Member` media arrival.
    #[cfg_attr(not(target_family = "wasm"), allow(unused_qualifications))]
    on_new_connection: platform::Callback<api::ConnectionHandle>,
}

impl Connections {
    /// Creates new [`Connections`].
    pub fn new(room_recv_constraints: Rc<RecvConstraints>) -> Self {
        Self {
            tracks: RefCell::default(),
            members_to_tracks: RefCell::default(),
            connections: RefCell::default(),
            room_recv_constraints,
            on_new_connection: platform::Callback::default(),
        }
    }

    /// Sets callback, which will be invoked when new [`Connection`] is
    /// established.
    #[cfg_attr(not(target_family = "wasm"), allow(unused_qualifications))]
    pub fn on_new_connection(
        &self,
        f: platform::Function<api::ConnectionHandle>,
    ) {
        self.on_new_connection.set_func(f);
    }

    /// Adds or updates information about related [`Track`]s with provided
    /// [`TrackId`] and [`MemberId`]s. Then [`Connections`] decides to create or
    /// to delete [`Connection`]s.
    ///
    /// Returns [`Connection`]s associated with provided [`MemberId`]s.
    ///
    /// [`Track`]: medea_client_api_proto::Track
    pub fn update_connections(
        &self,
        track_id: &TrackId,
        partner_members: HashSet<MemberId>,
    ) -> Vec<Connection> {
        if let Some(partners) = self.tracks.borrow_mut().get_mut(track_id) {
            let mut connections = self.connections.borrow_mut();
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

        self.add_connections(*track_id, partner_members)
    }

    /// Adds information about related [`Track`]s with provided [`TrackId`] and
    /// [`MemberId`]s, creates [`Connection`]s.
    ///
    /// Returns [`Connection`]s associated with provided [`MemberId`]s.
    ///
    /// [`Track`]: medea_client_api_proto::Track
    fn add_connections(
        &self,
        track_id: TrackId,
        partner_members: HashSet<MemberId>,
    ) -> Vec<Connection> {
        let connections = partner_members
            .iter()
            .map(|partner| {
                _ = self
                    .members_to_tracks
                    .borrow_mut()
                    .entry(partner.clone())
                    .or_default()
                    .insert(track_id);
                self.connections
                    .borrow_mut()
                    .entry(partner.clone())
                    .or_insert_with(|| {
                        let connection = Connection::new(
                            partner.clone(),
                            &self.room_recv_constraints,
                        );
                        self.on_new_connection.call1(connection.new_handle());
                        drop(
                            self.connections
                                .borrow_mut()
                                .insert(partner.clone(), connection.clone()),
                        );
                        connection
                    })
                    .clone()
            })
            .collect();

        drop(self.tracks.borrow_mut().insert(track_id, partner_members));

        connections
    }

    /// Removes information about [`Track`] with provided [`TrackId`]. Then
    /// [`Connections`] can decides to delete related [`Connection`].
    ///
    /// [`Track`]: medea_client_api_proto::Track
    pub fn remove_track(&self, track_id: &TrackId) {
        let mut tracks = self.tracks.borrow_mut();

        if let Some(partners) = tracks.remove(track_id) {
            for p in partners {
                if let Some(member_tracks) =
                    self.members_to_tracks.borrow_mut().get_mut(&p)
                {
                    _ = member_tracks.remove(track_id);

                    if member_tracks.is_empty() {
                        _ = self
                            .connections
                            .borrow_mut()
                            .remove(&p)
                            .map(|conn| conn.0.on_close.call0());
                    }
                }
            }
        }
    }

    /// Lookups [`Connection`] by the given remote [`MemberId`].
    pub fn get(&self, remote_member_id: &MemberId) -> Option<Connection> {
        self.connections.borrow().get(remote_member_id).cloned()
    }
}

/// Error of [`ConnectionHandle`]'s [`Weak`] pointer being detached.
#[derive(Caused, Clone, Copy, Debug, Display)]
#[cause(error = platform::Error)]
#[display(fmt = "`ConnectionHandle` is in detached state")]
pub struct HandleDetachedError;

/// External handler to a [`Connection`] with a remote `Member`.
///
/// Actually, represents a [`Weak`]-based handle to `InnerConnection`.
#[derive(Debug)]
pub struct ConnectionHandle(Weak<InnerConnection>);

/// Actual data of a connection with a specific remote `Member`.
///
/// Shared between external [`ConnectionHandle`] and Rust side [`Connection`].
#[derive(Debug)]
struct InnerConnection {
    /// Remote `Member` ID.
    remote_id: MemberId,

    /// Current [`ConnectionQualityScore`] of this [`Connection`].
    quality_score: Cell<Option<ConnectionQualityScore>>,

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
            let source_filter = source_kind
                .map_or(true, |skind| skind == r.source_kind().into());

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
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
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
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
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
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
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
    ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
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
            inner
                .change_media_state(desired_state, kind, source_kind)
                .await
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
                    room_recv_constraints.on_audio_enabled_change(),
                    MediaKind::Audio,
                    MediaSourceKind::Device,
                ),
            ],
            remote_id,
            quality_score: Cell::default(),
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
            MediaKind::Audio => self.0.recv_constraints.is_audio_enabled(),
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

    /// Updates [`ConnectionQualityScore`] of this [`Connection`].
    pub fn update_quality_score(&self, score: ConnectionQualityScore) {
        if self.0.quality_score.replace(Some(score)) != Some(score) {
            #[allow(clippy::as_conversions)]
            self.0.on_quality_score_update.call1(score as u8);
        }
    }
}
