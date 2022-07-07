//! [`Component`] for `MediaTrack` with a `Recv` direction.

use std::{convert::Infallible, rc::Rc};

use futures::StreamExt as _;
use medea_client_api_proto as proto;
use medea_client_api_proto::{
    MediaSourceKind, MediaType, MemberId, TrackId, TrackPatchEvent,
};
use medea_macro::watchers;
use medea_reactive::{
    when_all_processed, AllProcessed, Guarded, ObservableCell, Processed,
    ProgressableCell,
};

use crate::{
    media::{LocalTracksConstraints, MediaDirection, MediaKind},
    peer::{
        component::SyncState,
        media::{transitable_state::media_exchange_state, InTransition},
        MediaExchangeState, MediaExchangeStateController,
        MediaStateControllable, MuteStateController, TransceiverSide,
    },
    platform,
    utils::{component, AsProtoState, SynchronizableState, Updatable},
};

use super::Receiver;

/// Component responsible for the [`Receiver`] enabling/disabling and
/// muting/unmuting.
pub type Component = component::Component<State, Receiver>;

/// State of the [`Component`].
#[derive(Debug)]
pub struct State {
    /// ID of the [`Receiver`]'s [`remote::Track`].
    ///
    /// [`remote::Track`]: crate::media::track::remote::Track
    id: TrackId,

    /// [MID] of the [`Receiver`]'s [`Transceiver`].
    ///
    /// [`Transceiver`]: platform::Transceiver
    /// [MID]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    mid: Option<String>,

    /// [`MediaType`] of the [`Receiver`]'s [`remote::Track`].
    ///
    /// [`remote::Track`]: crate::media::track::remote::Track
    media_type: MediaType,

    /// ID of the member sending the [`Receiver`]'s [`remote::Track`].
    ///
    /// [`remote::Track`]: crate::media::track::remote::Track
    sender_id: MemberId,

    /// Indicator whether the [`Receiver`]'s [`remote::Track`] is enabled
    /// individually.
    ///
    /// [`remote::Track`]: crate::media::track::remote::Track
    enabled_individual: Rc<MediaExchangeStateController>,

    /// Indicator whether the [`Receiver`]'s [`remote::Track`] is enabled
    /// generally.
    ///
    /// [`remote::Track`]: crate::media::track::remote::Track
    enabled_general: ProgressableCell<media_exchange_state::Stable>,

    /// Current general [`MediaDirection`] of this [`Receiver`].
    media_direction: ObservableCell<MediaDirection>,

    /// Indicator whether the [`Receiver`]'s [`remote::Track`] is muted.
    ///
    /// [`remote::Track`]: crate::media::track::remote::Track
    muted: ObservableCell<bool>,

    /// Synchronization state of the [`Component`].
    sync_state: ObservableCell<SyncState>,
}

impl AsProtoState for State {
    type Output = proto::state::Receiver;

    fn as_proto(&self) -> Self::Output {
        Self::Output {
            id: self.id,
            mid: self.mid.clone(),
            media_type: self.media_type,
            sender_id: self.sender_id.clone(),
            muted: false,
            media_direction: self.media_direction().into(),
        }
    }
}

impl SynchronizableState for State {
    type Input = proto::state::Receiver;

    fn from_proto(input: Self::Input, _: &LocalTracksConstraints) -> Self {
        Self {
            id: input.id,
            mid: input.mid,
            media_type: input.media_type,
            sender_id: input.sender_id,
            enabled_individual: MediaExchangeStateController::new(
                media_exchange_state::Stable::from(
                    input.media_direction.is_recv_enabled(),
                ),
            ),
            enabled_general: ProgressableCell::new(
                media_exchange_state::Stable::from(
                    input.media_direction.is_enabled_general(),
                ),
            ),
            muted: ObservableCell::new(input.muted),
            media_direction: ObservableCell::new(input.media_direction.into()),
            sync_state: ObservableCell::new(SyncState::Synced),
        }
    }

    fn apply(&self, input: Self::Input, _: &LocalTracksConstraints) {
        let new_media_exchange_state = media_exchange_state::Stable::from(
            input.media_direction.is_recv_enabled(),
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

        self.enabled_general.set(media_exchange_state::Stable::from(
            input.media_direction.is_enabled_general(),
        ));
        self.media_direction.set(input.media_direction.into());

        self.sync_state.set(SyncState::Synced);
    }
}

impl Updatable for State {
    /// Returns [`Future`] resolving once [`media_exchange_state`] is
    /// stabilized.
    ///
    /// [`Future`]: std::future::Future
    fn when_stabilized(&self) -> AllProcessed<'static> {
        let controller = Rc::clone(&self.enabled_individual);
        when_all_processed(std::iter::once(
            Processed::new(Box::new(move || {
                let controller = Rc::clone(&controller);
                Box::pin(async move {
                    controller.when_stabilized().await;
                })
            }))
            .into(),
        ))
    }

    /// Returns [`Future`] resolving once [`State`] update will be applied onto
    /// the [`Receiver`].
    ///
    /// [`Future`]: std::future::Future
    fn when_updated(&self) -> AllProcessed<'static> {
        when_all_processed(vec![
            self.enabled_individual.when_processed().into(),
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

impl From<&State> for proto::state::Receiver {
    fn from(from: &State) -> Self {
        Self {
            id: from.id,
            mid: from.mid.clone(),
            media_type: from.media_type,
            sender_id: from.sender_id.clone(),
            media_direction: from.media_direction().into(),
            muted: false,
        }
    }
}

impl State {
    /// Returns [`State`] with a provided data.
    #[must_use]
    pub fn new(
        id: TrackId,
        mid: Option<String>,
        media_type: MediaType,
        sender: MemberId,
    ) -> Self {
        Self {
            id,
            mid,
            media_type,
            sender_id: sender,
            enabled_individual: MediaExchangeStateController::new(
                media_exchange_state::Stable::Enabled,
            ),
            enabled_general: ProgressableCell::new(
                media_exchange_state::Stable::Enabled,
            ),
            muted: ObservableCell::new(false),
            sync_state: ObservableCell::new(SyncState::Synced),
            media_direction: ObservableCell::new(MediaDirection::SendRecv),
        }
    }

    /// Returns [`TrackId`] of this [`State`].
    #[must_use]
    pub fn id(&self) -> TrackId {
        self.id
    }

    /// Returns current `mid` of this [`State`].
    #[must_use]
    pub fn mid(&self) -> Option<&str> {
        self.mid.as_deref()
    }

    /// Returns current [`MediaType`] of this [`State`].
    #[must_use]
    pub fn media_type(&self) -> MediaType {
        self.media_type
    }

    /// Returns current [`MemberId`] of the `Member` from which this
    /// [`State`] should receive media data.
    #[must_use]
    pub fn sender_id(&self) -> &MemberId {
        &self.sender_id
    }

    /// Returns current individual media exchange state of this [`State`].
    #[must_use]
    pub fn enabled_individual(&self) -> bool {
        self.enabled_individual.enabled()
    }

    /// Returns current general media exchange state of this [`State`].
    #[must_use]
    pub fn enabled_general(&self) -> bool {
        self.enabled_general.get() == media_exchange_state::Stable::Enabled
    }

    /// Returns current mute state of this [`State`].
    #[must_use]
    pub fn muted(&self) -> bool {
        self.muted.get()
    }

    /// Returns the current general [`MediaDirection`] of this [`State`].
    #[must_use]
    pub fn media_direction(&self) -> MediaDirection {
        self.media_direction.get()
    }

    /// Updates this [`State`] with the provided [`TrackPatchEvent`].
    pub fn update(&self, track_patch: &TrackPatchEvent) {
        if self.id != track_patch.id {
            return;
        }
        if let Some(direction) = track_patch.media_direction {
            self.enabled_general
                .set(direction.is_enabled_general().into());

            self.enabled_individual
                .update(direction.is_recv_enabled().into());
        }
        if let Some(muted) = track_patch.muted {
            self.muted.set(muted);
        }
        if let Some(direction) = track_patch.media_direction {
            self.media_direction.set(direction.into());
        }
    }
}

#[watchers]
impl Component {
    /// Watcher for the [`State::enabled_general`] updates.
    ///
    /// Updates [`Receiver`]'s general media exchange state. Adds or removes
    /// [`TransceiverDirection::RECV`] from the [`platform::Transceiver`] of the
    /// [`Receiver`].
    ///
    /// [`TransceiverDirection::RECV`]: platform::TransceiverDirection::RECV
    #[watch(self.enabled_general.subscribe())]
    async fn general_media_exchange_state_changed(
        receiver: Rc<Receiver>,
        _: Rc<State>,
        state: Guarded<media_exchange_state::Stable>,
    ) -> Result<(), Infallible> {
        let (state, _guard) = state.into_parts();
        receiver
            .enabled_general
            .set(state == media_exchange_state::Stable::Enabled);
        match state {
            media_exchange_state::Stable::Disabled => {
                let sub_direction = {
                    receiver
                        .transceiver
                        .borrow()
                        .as_ref()
                        .map(|trnscvr| trnscvr.set_recv(false))
                };
                if let Some(fut) = sub_direction {
                    fut.await;
                }
            }
            media_exchange_state::Stable::Enabled => {
                let add_recv = receiver
                    .transceiver
                    .borrow()
                    .as_ref()
                    .map(|trnscvr| trnscvr.set_recv(true));

                if let Some(fut) = add_recv {
                    fut.await;
                }
            }
        }
        receiver.maybe_notify_track().await;

        Ok(())
    }

    /// Watcher for [`media_exchange_state::Stable`] media exchange state
    /// updates.
    ///
    /// Updates [`Receiver::enabled_individual`] to the new state.
    #[watch(self.enabled_individual.subscribe_stable())]
    async fn enabled_individual_stable_state_changed(
        receiver: Rc<Receiver>,
        _: Rc<State>,
        state: media_exchange_state::Stable,
    ) -> Result<(), Infallible> {
        receiver
            .enabled_individual
            .set(state == media_exchange_state::Stable::Enabled);
        Ok(())
    }

    /// Watcher for media exchange state [`media_exchange_state::Transition`]
    /// updates.
    ///
    /// Sends [`TrackEvent::MediaExchangeIntention`][1] with the provided
    /// [`media_exchange_state`].
    ///
    /// [1]: crate::peer::TrackEvent::MediaExchangeIntention
    #[watch(self.enabled_individual.subscribe_transition())]
    async fn enabled_individual_transition_started(
        receiver: Rc<Receiver>,
        _: Rc<State>,
        state: media_exchange_state::Transition,
    ) -> Result<(), Infallible> {
        receiver.send_media_exchange_state_intention(state);
        Ok(())
    }

    /// Watcher for the mute state updates.
    ///
    /// Propagates command to the associated [`Receiver`] and updates its media
    /// track (if any).
    #[watch(self.muted.subscribe())]
    async fn mute_state_changed(
        receiver: Rc<Receiver>,
        _: Rc<State>,
        muted: bool,
    ) -> Result<(), Infallible> {
        receiver.muted.set(muted);
        if let Some(track) = receiver.track.borrow().as_ref() {
            track.set_muted(muted);
        }
        Ok(())
    }

    /// Stops transition timeouts on [`SyncState::Desynced`].
    ///
    /// Sends media state intentions and resets transition timeouts on
    /// [`SyncState::Synced`].
    #[watch(self.sync_state.subscribe().skip(1))]
    async fn sync_state_watcher(
        receiver: Rc<Receiver>,
        state: Rc<State>,
        sync_state: SyncState,
    ) -> Result<(), Infallible> {
        match sync_state {
            SyncState::Synced => {
                if let MediaExchangeState::Transition(transition) =
                    state.enabled_individual.state()
                {
                    receiver.send_media_exchange_state_intention(transition);
                }
                state.enabled_individual.reset_transition_timeout();
            }
            SyncState::Desynced => {
                state.enabled_individual.stop_transition_timeout();
            }
            SyncState::Syncing => (),
        }
        Ok(())
    }

    /// Updates [`MediaDirection`] of the provided [`Receiver`].
    #[watch(self.media_direction.subscribe())]
    async fn direction_watcher(
        receiver: Rc<Receiver>,
        _: Rc<State>,
        direction: MediaDirection,
    ) -> Result<(), Infallible> {
        receiver.set_media_direction(direction);

        Ok(())
    }
}

impl MediaStateControllable for State {
    fn media_exchange_state_controller(
        &self,
    ) -> Rc<MediaExchangeStateController> {
        Rc::clone(&self.enabled_individual)
    }

    fn mute_state_controller(&self) -> Rc<MuteStateController> {
        // Receivers can be muted, but currently they are muted directly by
        // server events.
        //
        // There is no point to provide an external API for muting receivers,
        // since the muting is pipelined after demuxing and decoding, so it
        // won't reduce incoming traffic or CPU usage. Therefore receivers
        // muting don't require `MuteStateController`'s state management.
        //
        // Removing this `unreachable!()` would require abstracting
        // `MuteStateController` to some trait and creating some dummy
        // implementation. Not worth it atm.
        unreachable!("Receivers muting is not implemented");
    }
}

impl TransceiverSide for State {
    fn track_id(&self) -> TrackId {
        self.id
    }

    fn kind(&self) -> MediaKind {
        match &self.media_type {
            MediaType::Audio(_) => MediaKind::Audio,
            MediaType::Video(_) => MediaKind::Video,
        }
    }

    fn source_kind(&self) -> MediaSourceKind {
        match &self.media_type {
            MediaType::Audio(_) => MediaSourceKind::Device,
            MediaType::Video(video) => video.source_kind,
        }
    }

    fn is_transitable(&self) -> bool {
        true
    }
}

#[cfg(feature = "mockable")]
impl State {
    /// Stabilizes [`MediaExchangeState`] of this [`State`].
    pub fn stabilize(&self) {
        if let crate::peer::MediaExchangeState::Transition(transition) =
            self.enabled_individual.state()
        {
            self.enabled_individual.update(transition.intended());
            self.enabled_general.set(transition.intended());
        }
    }

    /// Sets the [`State::sync_state`] to a [`SyncState::Synced`].
    pub fn synced(&self) {
        self.sync_state.set(SyncState::Synced);
    }
}
