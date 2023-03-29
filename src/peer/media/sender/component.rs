//! [`Component`] for `MediaTrack` with a `Send` direction.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use futures::{future::LocalBoxFuture, StreamExt as _};
use medea_client_api_proto::{
    self as proto, MediaDirection, MediaSourceKind, MediaType, MemberId,
    TrackId, TrackPatchEvent,
};
use medea_macro::watchers;
use medea_reactive::{AllProcessed, Guarded, ObservableCell, ProgressableCell};
use proto::ConnectionMode;
use tracerr::Traced;

use crate::{
    media::{LocalTracksConstraints, MediaKind, TrackConstraints, VideoSource},
    peer::{
        component::SyncState,
        media::{
            media_exchange_state, mute_state, InTransition, MediaExchangeState,
            MuteState, ProhibitedStateError,
        },
        MediaExchangeStateController, MediaState, MediaStateControllable,
        MuteStateController, TransceiverSide, UpdateLocalStreamError,
    },
    utils::{component, AsProtoState, SynchronizableState, Updatable},
};

use super::Sender;

/// State of a [`Sender`]'s [`local::Track`].
///
/// [`PartialEq`] implementation of this state ignores
/// [`LocalTrackState::Failed`] content.
///
/// [`local::Track`]: crate::media::track::local::Track
#[derive(Debug, Clone)]
enum LocalTrackState {
    /// Indicates that [`Sender`] is new, or [`local::Track`] is set.
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    Stable,

    /// Indicates that [`Sender`] needs a new [`local::Track`].
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    NeedUpdate,

    /// Indicates that new [`local::Track`] getting is failed.
    ///
    /// Contains an [`UpdateLocalStreamError`] with which
    /// [getUserMedia()][1]/[getDisplayMedia()][2] request failed.
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    /// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
    Failed(Traced<UpdateLocalStreamError>),
}

impl PartialEq for LocalTrackState {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::NeedUpdate => matches!(other, Self::NeedUpdate),
            Self::Stable => matches!(other, Self::Stable),
            Self::Failed(_) => matches!(other, Self::Failed(_)),
        }
    }
}

/// Component responsible for the [`Sender`] enabling/disabling and
/// muting/unmuting.
pub type Component = component::Component<State, Sender>;

/// State of the [`Component`].
#[derive(Debug)]
pub struct State {
    /// ID of the [`Sender`]'s [`local::Track`].
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    id: TrackId,

    /// [MID] of the [`Sender`]'s [`Transceiver`].
    ///
    /// [`Transceiver`]: crate::platform::Transceiver
    /// [MID]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    mid: Option<String>,

    /// [`MediaType`] of the [`Sender`]'s [`local::Track`].
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    media_type: MediaType,

    /// IDs of the members the [`Sender`]'s [`local::Track`] is received by.
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    receivers: RefCell<Vec<MemberId>>,

    /// Indicator whether the [`Sender`]'s [`local::Track`] is enabled
    /// individually.
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    enabled_individual: Rc<MediaExchangeStateController>,

    /// Indicator whether the [`Sender`]'s [`local::Track`] is muted.
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    mute_state: Rc<MuteStateController>,

    /// Indicator whether the [`Sender`]'s [`local::Track`] is enabled
    /// generally.
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    enabled_general: ProgressableCell<media_exchange_state::Stable>,

    /// Current general [`MediaDirection`] of this [`Sender`].
    media_direction: Cell<MediaDirection>,

    /// [MediaStreamConstraints][1] of the [`Sender`]'s [`local::Track`].
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    send_constraints: LocalTracksConstraints,

    /// Indicator whether this [`Sender`] is working in a [P2P mesh] or [SFU]
    /// mode.
    ///
    /// [P2P mesh]: https://bloggeek.me/webrtcglossary/mesh
    /// [SFU]: https://webrtcglossary.com/sfu
    connection_mode: ConnectionMode,

    /// State of the [`Sender`]'s [`local::Track`].
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    local_track_state: ObservableCell<LocalTrackState>,

    /// Synchronization state of the [`Component`].
    sync_state: ObservableCell<SyncState>,
}

impl AsProtoState for State {
    type Output = proto::state::Sender;

    fn as_proto(&self) -> Self::Output {
        Self::Output {
            id: self.id,
            connection_mode: self.connection_mode,
            mid: self.mid.clone(),
            media_type: self.media_type,
            receivers: self.receivers.borrow().clone(),
            media_direction: self.media_direction.get(),
            muted: self.mute_state.muted(),
        }
    }
}

impl SynchronizableState for State {
    type Input = proto::state::Sender;

    fn from_proto(
        input: Self::Input,
        send_constraints: &LocalTracksConstraints,
    ) -> Self {
        Self {
            id: input.id,
            mid: input.mid,
            media_type: input.media_type,
            receivers: RefCell::new(input.receivers),
            mute_state: MuteStateController::new(mute_state::Stable::from(
                input.muted,
            )),
            enabled_individual: MediaExchangeStateController::new(
                media_exchange_state::Stable::from(
                    input.media_direction.is_send_enabled(),
                ),
            ),
            enabled_general: ProgressableCell::new(
                media_exchange_state::Stable::from(
                    input.media_direction.is_enabled_general(),
                ),
            ),
            media_direction: Cell::new(input.media_direction),
            send_constraints: send_constraints.clone(),
            connection_mode: input.connection_mode,
            local_track_state: ObservableCell::new(LocalTrackState::Stable),
            sync_state: ObservableCell::new(SyncState::Synced),
        }
    }

    fn apply(&self, input: Self::Input, _: &LocalTracksConstraints) {
        let new_media_exchange_state = media_exchange_state::Stable::from(
            input.media_direction.is_send_enabled(),
        );
        let current_media_exchange_state = match self.enabled_individual.state()
        {
            MediaExchangeState::Transition(transition) => {
                transition.into_inner()
            }
            MediaExchangeState::Stable(stable) => stable,
        };
        if current_media_exchange_state != new_media_exchange_state {
            self.enabled_individual.update(new_media_exchange_state);
        }

        let new_mute_state = mute_state::Stable::from(input.muted);
        let current_mute_state = match self.mute_state.state() {
            MuteState::Stable(stable) => stable,
            MuteState::Transition(transition) => transition.into_inner(),
        };
        if current_mute_state != new_mute_state {
            self.mute_state.update(new_mute_state);
        }

        let new_general_media_exchange_state =
            media_exchange_state::Stable::from(
                input.media_direction.is_enabled_general(),
            );
        self.enabled_general.set(new_general_media_exchange_state);

        self.sync_state.set(SyncState::Synced);
    }
}

impl Updatable for State {
    /// Returns [`Future`] resolving once [`media_exchange_state`] and
    /// [`mute_state`] are stabilized.
    ///
    /// [`Future`]: std::future::Future
    fn when_stabilized(&self) -> AllProcessed<'static> {
        medea_reactive::when_all_processed(vec![
            Rc::clone(&self.enabled_individual).when_stabilized().into(),
            Rc::clone(&self.mute_state).when_stabilized().into(),
        ])
    }

    /// Returns [`Future`] resolving once a [`State`] update is applied onto the
    /// [`Sender`].
    ///
    /// [`Future`]: std::future::Future
    fn when_updated(&self) -> AllProcessed<'static> {
        medea_reactive::when_all_processed(vec![
            self.enabled_individual.when_processed().into(),
            self.mute_state.when_processed().into(),
            self.enabled_general.when_all_processed().into(),
        ])
    }

    /// Notifies [`State`] about a RPC connection loss.
    fn connection_lost(&self) {
        self.sync_state.set(SyncState::Desynced);
    }

    /// Notifies [`State`] about a RPC connection restore.
    fn connection_recovered(&self) {
        self.sync_state.set(SyncState::Syncing);
    }
}

impl From<&State> for proto::state::Sender {
    fn from(state: &State) -> Self {
        Self {
            id: state.id,
            connection_mode: state.connection_mode,
            mid: state.mid.clone(),
            media_type: state.media_type,
            receivers: state.receivers.borrow().clone(),
            media_direction: state.media_direction.get(),
            muted: state.mute_state.muted(),
        }
    }
}

impl State {
    /// Creates new [`State`] with the provided data.
    #[must_use]
    pub fn new(
        id: TrackId,
        mid: Option<String>,
        media_type: MediaType,
        media_direction: MediaDirection,
        receivers: Vec<MemberId>,
        send_constraints: LocalTracksConstraints,
        connection_mode: ConnectionMode,
    ) -> Self {
        Self {
            id,
            mid,
            media_type,
            receivers: RefCell::new(receivers),
            enabled_individual: MediaExchangeStateController::new(
                media_exchange_state::Stable::from(
                    media_direction.is_send_enabled(),
                ),
            ),
            enabled_general: ProgressableCell::new(
                media_exchange_state::Stable::from(
                    media_direction.is_enabled_general(),
                ),
            ),
            media_direction: Cell::new(media_direction),
            mute_state: MuteStateController::new(mute_state::Stable::from(
                false,
            )),
            sync_state: ObservableCell::new(SyncState::Synced),
            send_constraints,
            connection_mode,
            local_track_state: ObservableCell::new(LocalTrackState::Stable),
        }
    }

    /// Indicates whether this [`Sender`]'s media exchange state is in
    /// [`media_exchange_state::Stable::Enabled`].
    #[must_use]
    pub fn enabled(&self) -> bool {
        self.enabled_individual.enabled()
    }

    /// Returns [`TrackId`] of this [`State`].
    #[must_use]
    pub const fn id(&self) -> TrackId {
        self.id
    }

    /// Returns current `mid` of this [`State`].
    #[must_use]
    pub fn mid(&self) -> Option<&str> {
        self.mid.as_deref()
    }

    /// Returns current [`MediaType`] of this [`State`].
    #[must_use]
    pub const fn media_type(&self) -> MediaType {
        self.media_type
    }

    /// Returns current [`MemberId`]s of the `Member`s that this [`State`]
    /// should send media data to.
    #[must_use]
    pub fn receivers(&self) -> Vec<MemberId> {
        self.receivers.borrow().clone()
    }

    /// Returns current individual media exchange state of this [`State`].
    #[must_use]
    pub fn is_enabled_individual(&self) -> bool {
        self.enabled_individual.enabled()
    }

    /// Returns current general media exchange state of this [`State`].
    #[must_use]
    pub fn is_enabled_general(&self) -> bool {
        self.enabled_general.get() == media_exchange_state::Stable::Enabled
    }

    /// Returns current mute state of this [`State`].
    #[must_use]
    pub fn is_muted(&self) -> bool {
        self.mute_state.muted()
    }

    /// Returns [`Future`] which will be resolved once
    /// [getUserMedia()][1]/[getDisplayMedia()][2] request for this [`State`] is
    /// resolved.
    ///
    /// [`Future`]: std::future::Future
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    /// [2]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
    pub fn local_stream_update_result(
        &self,
    ) -> LocalBoxFuture<'static, Result<(), Traced<UpdateLocalStreamError>>>
    {
        let mut local_track_state_rx = self.local_track_state.subscribe();
        Box::pin(async move {
            while let Some(s) = local_track_state_rx.next().await {
                match s {
                    LocalTrackState::Stable => return Ok(()),
                    LocalTrackState::Failed(err) => {
                        return Err(tracerr::new!(err))
                    }
                    LocalTrackState::NeedUpdate => (),
                }
            }

            Ok(())
        })
    }

    /// Updates this [`State`] with the provided [`TrackPatchEvent`].
    pub fn update(&self, track_patch: TrackPatchEvent) {
        if track_patch.id != self.id {
            return;
        }
        if let Some(direction) = track_patch.media_direction {
            self.media_direction.set(direction);
            self.enabled_general
                .set((direction.is_enabled_general()).into());

            self.enabled_individual
                .update(direction.is_send_enabled().into());
        }
        if let Some(muted) = track_patch.muted {
            self.mute_state.update(mute_state::Stable::from(muted));
        }
        if let Some(receivers) = track_patch.receivers {
            *self.receivers.borrow_mut() = receivers;
        }
    }

    /// Indicates whether local `MediaStream` update needed for this [`State`].
    #[must_use]
    pub fn is_local_stream_update_needed(&self) -> bool {
        matches!(self.local_track_state.get(), LocalTrackState::NeedUpdate)
    }

    /// Marks an inner `local_track_state` of this [`State`] as failed with the
    /// provided `error`.
    pub fn failed_local_stream_update(
        &self,
        error: Traced<UpdateLocalStreamError>,
    ) {
        self.local_track_state.set(LocalTrackState::Failed(error));
    }

    /// Marks an inner `local_track_state` of this [`State`] as stable.
    pub fn local_stream_updated(&self) {
        self.local_track_state.set(LocalTrackState::Stable);
    }

    /// Returns [`MediaKind`] of this [`State`].
    #[must_use]
    pub const fn media_kind(&self) -> MediaKind {
        match &self.media_type {
            MediaType::Audio(_) => MediaKind::Audio,
            MediaType::Video(_) => MediaKind::Video,
        }
    }

    /// Returns [`MediaSourceKind`] of this [`State`].
    #[must_use]
    pub const fn media_source(&self) -> MediaSourceKind {
        match &self.media_type {
            MediaType::Audio(_) => MediaSourceKind::Device,
            MediaType::Video(video) => video.source_kind,
        }
    }
}

#[watchers]
impl Component {
    /// Watcher for media exchange state [`media_exchange_state::Transition`]
    /// updates.
    ///
    /// Sends [`TrackEvent::MediaExchangeIntention`][1] with the provided
    /// [`media_exchange_state`].
    ///
    /// [1]: crate::peer::TrackEvent::MediaExchangeIntention
    #[watch(self.enabled_individual.subscribe_transition())]
    fn enabled_individual_transition_started(
        sender: &Sender,
        _: &State,
        new_state: media_exchange_state::Transition,
    ) {
        sender.send_media_exchange_state_intention(new_state);
    }

    /// Watcher for mute state [`mute_state::Transition`] updates.
    ///
    /// Sends [`TrackEvent::MuteUpdateIntention`][1] with the provided
    /// [`mute_state`].
    ///
    /// [1]: crate::peer::TrackEvent::MuteUpdateIntention
    #[watch(self.mute_state.subscribe_transition())]
    fn mute_state_transition_watcher(
        sender: &Sender,
        _: &State,
        new_state: mute_state::Transition,
    ) {
        sender.send_mute_state_intention(new_state);
    }

    /// Watcher for the [`State::enabled_general`] update.
    ///
    /// Updates [`Sender`]'s general media exchange state. Adds or removes
    /// [`SEND`] direction from the [`Transceiver`] of this [`Sender`].
    ///
    /// [`SEND`]: crate::platform::TransceiverDirection::SEND
    /// [`Transceiver`]: crate::platform::Transceiver
    #[watch(self.enabled_general.subscribe())]
    async fn enabled_general_state_changed(
        sender: Rc<Sender>,
        state: Rc<State>,
        new_state: Guarded<media_exchange_state::Stable>,
    ) {
        let (new_state, _guard) = new_state.into_parts();
        sender
            .enabled_general
            .set(new_state == media_exchange_state::Stable::Enabled);

        if state.connection_mode == ConnectionMode::Sfu {
            sender.transceiver.set_send(true).await;
        } else {
            match new_state {
                media_exchange_state::Stable::Enabled => {
                    if sender.enabled_in_cons() {
                        sender.transceiver.set_send(true).await;
                    }
                }
                media_exchange_state::Stable::Disabled => {
                    sender.transceiver.set_send(false).await;
                }
            }
        }
    }

    /// Watcher for [`media_exchange_state::Stable`] media exchange state
    /// updates.
    ///
    /// Updates [`Sender::enabled_individual`] to the `new_state`.
    ///
    /// Removes `MediaTrack` from [`platform::Transceiver`] if `new_state` is
    /// [`media_exchange_state::Stable::Disabled`].
    ///
    /// Marks [`State::local_track_state`] as [`LocalTrackState::NeedUpdate`] if
    /// `new_state` is [`media_exchange_state::Stable::Enabled`].
    #[watch(self.enabled_individual.subscribe_stable())]
    async fn enabled_individual_stable_state_changed(
        sender: Rc<Sender>,
        state: Rc<State>,
        new_state: media_exchange_state::Stable,
    ) {
        sender
            .enabled_individual
            .set(new_state == media_exchange_state::Stable::Enabled);
        match new_state {
            media_exchange_state::Stable::Enabled => {
                state.local_track_state.set(LocalTrackState::NeedUpdate);
            }
            media_exchange_state::Stable::Disabled => {
                sender.remove_track().await;
            }
        }
    }

    /// Watcher for the [`mute_state::Stable`] updates.
    ///
    /// Updates [`Sender`]'s mute state.
    ///
    /// Updates [`Sender`]'s [`platform::Transceiver`] `MediaTrack.enabled`
    /// property.
    #[watch(self.mute_state.subscribe_stable())]
    fn mute_state_stable_watcher(
        sender: &Sender,
        _: &State,
        new_state: mute_state::Stable,
    ) {
        sender.muted.set(new_state == mute_state::Stable::Muted);
        if let Some(track) = sender.track.borrow().as_ref() {
            match new_state {
                mute_state::Stable::Muted => {
                    track.set_enabled(false);
                }
                mute_state::Stable::Unmuted => {
                    track.set_enabled(true);
                }
            }
        }
    }

    /// Stops transition timeouts on a [`SyncState::Desynced`].
    ///
    /// Sends media state intentions and resets transition timeouts on a
    /// [`SyncState::Synced`].
    #[watch(self.sync_state.subscribe().skip(1))]
    fn sync_state_watcher(
        sender: &Sender,
        state: &State,
        sync_state: SyncState,
    ) {
        match sync_state {
            SyncState::Synced => {
                if let MediaExchangeState::Transition(transition) =
                    state.enabled_individual.state()
                {
                    sender.send_media_exchange_state_intention(transition);
                }
                if let MuteState::Transition(transition) =
                    state.mute_state.state()
                {
                    sender.send_mute_state_intention(transition);
                }
                state.enabled_individual.reset_transition_timeout();
                state.mute_state.reset_transition_timeout();
            }
            SyncState::Desynced => {
                state.enabled_individual.stop_transition_timeout();
                state.mute_state.stop_transition_timeout();
            }
            SyncState::Syncing => (),
        }
    }

    /// Disables media exchange on a local track acquisition error.
    #[allow(clippy::needless_pass_by_value)]
    #[watch(self.local_track_state.subscribe())]
    fn local_track_state_changed(
        _: &Sender,
        state: &State,
        new_state: LocalTrackState,
    ) -> Result<(), Traced<ProhibitedStateError>> {
        if matches!(new_state, LocalTrackState::Failed(_)) {
            state.media_state_transition_to(MediaState::MediaExchange(
                media_exchange_state::Stable::Disabled,
            ))?;
        }
        Ok(())
    }
}

impl TransceiverSide for State {
    fn track_id(&self) -> TrackId {
        self.id
    }

    fn kind(&self) -> MediaKind {
        self.media_kind()
    }

    fn source_kind(&self) -> MediaSourceKind {
        self.media_source()
    }

    fn is_transitable(&self) -> bool {
        let caps = TrackConstraints::from(self.media_type);
        match &caps {
            TrackConstraints::Video(VideoSource::Device(_)) => {
                self.send_constraints.inner().get_device_video().is_some()
            }
            TrackConstraints::Video(VideoSource::Display(_)) => {
                self.send_constraints.inner().get_display_video().is_some()
            }
            TrackConstraints::Audio(_) => true,
        }
    }
}

impl MediaStateControllable for State {
    fn media_exchange_state_controller(
        &self,
    ) -> Rc<MediaExchangeStateController> {
        Rc::clone(&self.enabled_individual)
    }

    fn mute_state_controller(&self) -> Rc<MuteStateController> {
        Rc::clone(&self.mute_state)
    }

    fn media_state_transition_to(
        &self,
        desired_state: MediaState,
    ) -> Result<(), Traced<ProhibitedStateError>> {
        if self.media_type.required()
            && matches!(
                desired_state,
                MediaState::Mute(mute_state::Stable::Muted)
                    | MediaState::MediaExchange(
                        media_exchange_state::Stable::Disabled
                    )
            )
        {
            Err(tracerr::new!(
                ProhibitedStateError::CannotDisableRequiredSender
            ))
        } else {
            match desired_state {
                MediaState::MediaExchange(desired_state) => {
                    self.media_exchange_state_controller()
                        .transition_to(desired_state);
                }
                MediaState::Mute(desired_state) => {
                    self.mute_state_controller().transition_to(desired_state);
                }
            }
            Ok(())
        }
    }
}

#[cfg(feature = "mockable")]
#[allow(clippy::multiple_inherent_impl)]
impl State {
    /// Sets the [`State::sync_state`] to a [`SyncState::Synced`].
    pub fn synced(&self) {
        self.sync_state.set(SyncState::Synced);
    }
}
