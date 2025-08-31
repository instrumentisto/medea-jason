//! [`Component`] for `MediaTrack` with a `Recv` direction.

use std::{iter, rc::Rc};

use futures::StreamExt as _;
use medea_client_api_proto as proto;
use medea_client_api_proto::{
    MediaSourceKind, MediaType, MemberId, TrackId, TrackPatchEvent,
};
use medea_macro::watchers;
use medea_reactive::{
    AllProcessed, Guarded, ObservableCell, Processed, ProgressableCell,
    when_all_processed,
};
use proto::ConnectionMode;

use super::Receiver;
use crate::{
    media::{LocalTracksConstraints, MediaDirection, MediaKind},
    peer::{
        MediaExchangeState, MediaExchangeStateController,
        MediaStateControllable, MuteStateController, TransceiverSide,
        component::SyncPhase,
        media::{InTransition as _, transitable_state::media_exchange_state},
    },
    utils::{AsProtoState, SynchronizableState, Updatable, component},
};

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
    /// [`Transceiver`]: crate::platform::Transceiver
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

    /// Indicator whether this [`Receiver`] is working in a [P2P mesh] or [SFU]
    /// mode.
    ///
    /// [P2P mesh]: https://webrtcglossary.com/mesh
    /// [SFU]: https://webrtcglossary.com/sfu
    connection_mode: ConnectionMode,

    /// [`SyncPhase`] of the [`Component`].
    sync_phase: ObservableCell<SyncPhase>,
}

impl AsProtoState for State {
    type Output = proto::state::Receiver;

    fn as_proto(&self) -> Self::Output {
        Self::Output {
            id: self.id,
            connection_mode: self.connection_mode,
            mid: self.mid.clone(),
            media_type: self.media_type.clone(),
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
            connection_mode: input.connection_mode,
            sync_phase: ObservableCell::new(SyncPhase::Synced),
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

        self.sync_phase.set(SyncPhase::Synced);
    }
}

impl Updatable for State {
    /// Returns [`Future`] resolving once [`media_exchange_state`] is
    /// stabilized.
    fn when_stabilized(&self) -> AllProcessed<'static> {
        let controller = Rc::clone(&self.enabled_individual);
        when_all_processed(iter::once(
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
    fn when_updated(&self) -> AllProcessed<'static> {
        when_all_processed(vec![
            self.enabled_individual.when_processed().into(),
            self.enabled_general.when_all_processed().into(),
        ])
    }

    /// Notifies [`State`] about a RPC connection loss.
    fn connection_lost(&self) {
        self.sync_phase.set(SyncPhase::Desynced);
    }

    /// Notifies [`State`] about a RPC connection restore.
    fn connection_recovered(&self) {
        self.sync_phase.set(SyncPhase::Syncing);
    }
}

impl From<&State> for proto::state::Receiver {
    fn from(from: &State) -> Self {
        Self {
            id: from.id,
            connection_mode: from.connection_mode,
            mid: from.mid.clone(),
            media_type: from.media_type.clone(),
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
        media_direction: medea_client_api_proto::MediaDirection,
        muted: bool,
        sender: MemberId,
        connection_mode: ConnectionMode,
    ) -> Self {
        Self {
            id,
            mid,
            media_type,
            sender_id: sender,
            enabled_individual: MediaExchangeStateController::new(
                media_direction.is_recv_enabled().into(),
            ),
            enabled_general: ProgressableCell::new(
                media_direction.is_enabled_general().into(),
            ),
            muted: ObservableCell::new(muted),
            sync_phase: ObservableCell::new(SyncPhase::Synced),
            connection_mode,
            media_direction: ObservableCell::new(media_direction.into()),
        }
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
    pub const fn media_type(&self) -> &MediaType {
        &self.media_type
    }

    /// Returns current [`MemberId`] of the `Member` from which this
    /// [`State`] should receive media data.
    #[must_use]
    pub const fn sender_id(&self) -> &MemberId {
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
            self.enabled_general.set(direction.is_enabled_general().into());

            self.enabled_individual.update(direction.is_recv_enabled().into());
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
    /// [`RECV`] direction from the [`Transceiver`] of the [`Receiver`].
    ///
    /// [`RECV`]: crate::platform::TransceiverDirection::RECV
    /// [`Transceiver`]: crate::platform::Transceiver
    #[watch(self.enabled_general.subscribe())]
    async fn general_media_exchange_state_changed(
        receiver: Rc<Receiver>,
        st: Rc<State>,
        state: Guarded<media_exchange_state::Stable>,
    ) {
        let (state, _guard) = state.into_parts();
        receiver
            .enabled_general
            .set(state == media_exchange_state::Stable::Enabled);
        if (st.connection_mode, state)
            == (ConnectionMode::Mesh, media_exchange_state::Stable::Disabled)
        {
            let sub_recv = {
                receiver
                    .transceiver
                    .borrow()
                    .as_ref()
                    .map(|trnscvr| trnscvr.set_recv(false))
            };
            if let Some(fut) = sub_recv {
                fut.await;
            }
        } else {
            let add_recv = receiver
                .transceiver
                .borrow()
                .as_ref()
                .map(|trnscvr| trnscvr.set_recv(true));
            if let Some(fut) = add_recv {
                fut.await;
            }
        }
        receiver.maybe_notify_track().await;
    }

    /// Watcher for [`media_exchange_state::Stable`] media exchange state
    /// updates.
    ///
    /// Updates [`Receiver::enabled_individual`] to the new state.
    #[watch(self.enabled_individual.subscribe_stable())]
    fn enabled_individual_stable_state_changed(
        receiver: &Receiver,
        _: &State,
        state: media_exchange_state::Stable,
    ) {
        receiver
            .enabled_individual
            .set(state == media_exchange_state::Stable::Enabled);
    }

    /// Watcher for media exchange state [`media_exchange_state::Transition`]
    /// updates.
    ///
    /// Sends [`TrackEvent::MediaExchangeIntention`][1] with the provided
    /// [`media_exchange_state`].
    ///
    /// [1]: crate::peer::TrackEvent::MediaExchangeIntention
    #[watch(self.enabled_individual.subscribe_transition())]
    fn enabled_individual_transition_started(
        receiver: &Receiver,
        _: &State,
        state: media_exchange_state::Transition,
    ) {
        receiver.send_media_exchange_state_intention(state);
    }

    /// Watcher for the mute state updates.
    ///
    /// Propagates command to the associated [`Receiver`] and updates its media
    /// track (if any).
    #[watch(self.muted.subscribe())]
    fn mute_state_changed(receiver: &Receiver, _: &State, muted: bool) {
        receiver.muted.set(muted);
        if let Some(track) = receiver.track.borrow().as_ref() {
            track.set_muted(muted);
        }
    }

    /// Stops transition timeouts on [`SyncPhase::Desynced`].
    ///
    /// Sends media state intentions and resets transition timeouts on
    /// [`SyncPhase::Synced`].
    #[watch(self.sync_phase.subscribe().skip(1))]
    fn sync_phase_watcher(
        receiver: &Receiver,
        state: &State,
        sync_phase: SyncPhase,
    ) {
        match sync_phase {
            SyncPhase::Synced => {
                if let MediaExchangeState::Transition(transition) =
                    state.enabled_individual.state()
                {
                    receiver.send_media_exchange_state_intention(transition);
                }
                state.enabled_individual.reset_transition_timeout();
            }
            SyncPhase::Desynced => {
                state.enabled_individual.stop_transition_timeout();
            }
            SyncPhase::Syncing => (),
        }
    }

    /// Updates [`MediaDirection`] of the provided [`Receiver`].
    #[watch(self.media_direction.subscribe())]
    fn direction_watcher(
        receiver: &Receiver,
        _: &State,
        direction: MediaDirection,
    ) {
        receiver.set_media_direction(direction);
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
            MediaType::Audio(audio) => audio.source_kind,
            MediaType::Video(video) => video.source_kind,
        }
    }

    fn is_transitable(&self) -> bool {
        true
    }
}

#[cfg(feature = "mockable")]
// TODO: Try remove on next Rust version upgrade.
#[expect(clippy::allow_attributes, reason = "`#[expect]` is not considered")]
#[allow(clippy::multiple_inherent_impl, reason = "feature gated")]
impl State {
    /// Stabilizes the [`MediaExchangeState`] of this [`State`].
    pub fn stabilize(&self) {
        if let MediaExchangeState::Transition(transition) =
            self.enabled_individual.state()
        {
            self.enabled_individual.update(transition.intended());
            self.enabled_general.set(transition.intended());
        }
    }

    /// Sets the [`State::sync_phase`] to a [`SyncPhase::Synced`].
    pub fn synced(&self) {
        self.sync_phase.set(SyncPhase::Synced);
    }
}
