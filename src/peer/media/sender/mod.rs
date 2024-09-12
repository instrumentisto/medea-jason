//! Implementation of the `MediaTrack` with a `Send` direction.

mod component;

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use derive_more::{Display, From};
use futures::channel::mpsc;
use medea_client_api_proto::TrackId;
use tracerr::Traced;

use crate::{
    media::{track::local, LocalTracksConstraints, TrackConstraints},
    peer::TrackEvent,
    platform,
    utils::Caused,
};

use super::{
    media_exchange_state, mute_state, MediaConnections, MediaStateControllable,
};

#[doc(inline)]
pub use self::component::{Component, State};

/// Errors occurring when creating a new [`Sender`].
#[derive(Caused, Clone, Debug, Display)]
#[cause(error = platform::Error)]
pub enum CreateError {
    /// [`Sender`] cannot be disabled because it's marked as `required`.
    #[display(
        "`MediaExchangeState` of `Sender` cannot transit to \
         disabled state, because this `Sender` is required"
    )]
    CannotDisableRequiredSender,

    /// Could not find a [`platform::Transceiver`] by `mid`.
    #[display("Unable to find Transceiver with mid: {_0}")]
    TransceiverNotFound(String),
}

/// Error occuring in [`RTCRtpSender.replaceTrack()`][1] method.
///
/// [1]: https://w3.org/TR/webrtc#dom-rtcrtpsender-replacetrack
#[derive(Caused, Clone, Debug, Display, From)]
#[cause(error = platform::Error)]
#[display("`MediaManagerHandle` is in detached state")]
pub struct InsertTrackError(platform::Error);

/// Representation of a [`local::Track`] that is being sent to some remote peer.
#[derive(Debug)]
pub struct Sender {
    /// ID of this [`local::Track`].
    track_id: TrackId,

    /// Constraints of this [`local::Track`].
    caps: TrackConstraints,

    /// [`Transceiver`] associated with this [`local::Track`].
    ///
    /// [`Transceiver`]: platform::Transceiver
    transceiver: platform::Transceiver,

    /// [`local::Track`] that this [`Sender`] is transmitting to the remote.
    track: RefCell<Option<Rc<local::Track>>>,

    /// Indicator whether this [`local::Track`] is muted.
    muted: Cell<bool>,

    /// Indicator whether this [`local::Track`] is enabled individually.
    enabled_individual: Cell<bool>,

    /// Indicator whether this [`local::Track`] is enabled generally.
    enabled_general: Cell<bool>,

    /// [MediaStreamConstraints][1] of this [`local::Track`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    send_constraints: LocalTracksConstraints,

    /// Channel for sending [`TrackEvent`]s to the actual [`local::Track`].
    track_events_sender: mpsc::UnboundedSender<TrackEvent>,
}

impl Sender {
    /// Creates a new [`platform::Transceiver`] if the provided `mid` is
    /// [`None`], otherwise retrieves an existing [`platform::Transceiver`] via
    /// the provided `mid` from the provided [`MediaConnections`].
    ///
    /// # Errors
    ///
    /// With a [`CreateError::TransceiverNotFound`] if [`State`] has [`Some`]
    /// [`mid`], but this [`mid`] isn't found in the [`MediaConnections`].
    ///
    /// With a [`CreateError::CannotDisableRequiredSender`] if the provided
    /// [`LocalTracksConstraints`] are configured to disable this [`Sender`],
    /// but it cannot be disabled according to the provide [`State`].
    ///
    /// [`mid`]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    pub async fn new(
        state: &State,
        media_connections: &MediaConnections,
        send_constraints: LocalTracksConstraints,
        track_events_sender: mpsc::UnboundedSender<TrackEvent>,
    ) -> Result<Rc<Self>, Traced<CreateError>> {
        let enabled_in_cons = send_constraints.enabled(state.media_type());
        let muted_in_cons = send_constraints.muted(state.media_type());
        let media_disabled = state.is_muted()
            || !state.is_enabled_individual()
            || !enabled_in_cons
            || muted_in_cons;
        if state.media_type().required() && media_disabled {
            return Err(tracerr::new!(
                CreateError::CannotDisableRequiredSender
            ));
        }

        let caps = TrackConstraints::from(state.media_type().clone());
        let transceiver = match state.mid() {
            // Try to find rcvr transceiver that can be used as sendrecv.
            None => {
                let transceiver = media_connections
                    .0
                    .borrow()
                    .receivers
                    .values()
                    .find(|rcvr| {
                        rcvr.caps().media_kind() == caps.media_kind()
                            && rcvr.caps().media_source_kind()
                                == caps.media_source_kind()
                    })
                    .and_then(|rcvr| rcvr.transceiver());
                if let Some(trcv) = transceiver {
                    trcv
                } else {
                    let add_transceiver =
                        media_connections.0.borrow().add_transceiver(
                            state.media_type().clone(),
                            platform::TransceiverDirection::INACTIVE,
                        );
                    add_transceiver.await
                }
            }
            Some(mid) => {
                let get_transceiver = media_connections
                    .0
                    .borrow()
                    .get_transceiver_by_mid(mid.into());
                get_transceiver
                    .await
                    .ok_or_else(|| CreateError::TransceiverNotFound(mid.into()))
                    .map_err(tracerr::wrap!())?
            }
        };

        let this = Rc::new(Self {
            track_id: state.id(),
            caps,
            transceiver,
            enabled_general: Cell::new(state.is_enabled_general()),
            enabled_individual: Cell::new(state.is_enabled_individual()),
            muted: Cell::new(state.is_muted()),
            track_events_sender,
            send_constraints,
            track: RefCell::new(None),
        });

        state
            .media_exchange_state_controller()
            .transition_to(media_exchange_state::Stable::from(enabled_in_cons));
        if muted_in_cons {
            state
                .mute_state_controller()
                .transition_to(mute_state::Stable::from(muted_in_cons));
        }

        Ok(this)
    }

    /// Returns [`TrackConstraints`] of this [`Sender`].
    pub const fn caps(&self) -> &TrackConstraints {
        &self.caps
    }

    /// Indicates whether this [`Sender`] is publishing media traffic.
    pub async fn is_publishing(&self) -> bool {
        self.transceiver
            .has_direction(platform::TransceiverDirection::SEND)
            .await
    }

    /// Drops [`local::Track`] used by this [`Sender`]. Sets track used by
    /// sending side of inner transceiver to [`None`].
    ///
    /// # Panics
    ///
    /// If [replaceTrack()][2] call fails. This might happen if an underlying
    /// [RTCRtpSender][1] is stopped. [replaceTrack()][2] with `null` track
    /// should never fail for any other reason.
    ///
    /// [1]: https://w3c.github.io/webrtc-pc/#dom-rtcrtpsender
    /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpsender-replacetrack
    pub async fn remove_track(&self) {
        drop(self.track.take());
        drop(self.transceiver.set_send_track(None).await);
    }

    /// Indicates whether this [`Sender`] has [`local::Track`].
    #[must_use]
    pub fn has_track(&self) -> bool {
        self.track.borrow().is_some()
    }

    /// Inserts provided [`local::Track`] into provided [`Sender`]s
    /// transceiver. No-op if provided track already being used by this
    /// [`Sender`].
    pub(super) async fn insert_track(
        self: Rc<Self>,
        new_track: Rc<local::Track>,
    ) -> Result<(), Traced<InsertTrackError>> {
        // no-op if we try to insert same track
        if let Some(current_track) = self.track.borrow().as_ref() {
            if new_track.id() == current_track.id() {
                return Ok(());
            }
        }

        let new_track = Rc::new(new_track.fork().await);
        new_track.set_enabled(!self.muted.get());
        self.transceiver
            .set_send_track(Some(&new_track))
            .await
            .map_err(InsertTrackError::from)
            .map_err(tracerr::wrap!())?;

        // Set enabled once again since `muted` might have changed.
        new_track.set_enabled(!self.muted.get());
        drop(self.track.replace(Some(new_track)));

        Ok(())
    }

    /// Returns [`platform::Transceiver`] of this [`Sender`].
    #[must_use]
    pub fn transceiver(&self) -> platform::Transceiver {
        self.transceiver.clone()
    }

    /// Returns the [`local::Track`] being sent to remote, if any.
    #[must_use]
    pub fn get_send_track(&self) -> Option<Rc<local::Track>> {
        self.track.borrow().as_ref().cloned()
    }

    /// Returns [`mid`] of this [`Sender`].
    ///
    /// [`mid`]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    #[must_use]
    pub fn mid(&self) -> Option<String> {
        self.transceiver.mid()
    }

    /// Indicates whether this [`Sender`] is enabled in
    /// [`LocalTracksConstraints`].
    fn enabled_in_cons(&self) -> bool {
        self.send_constraints.is_track_enabled_and_constrained(
            self.caps.media_kind(),
            Some(self.caps.media_source_kind()),
        )
    }

    /// Sends [`TrackEvent::MediaExchangeIntention`] with the provided
    /// [`media_exchange_state`].
    pub fn send_media_exchange_state_intention(
        &self,
        state: media_exchange_state::Transition,
    ) {
        _ = self.track_events_sender.unbounded_send(
            TrackEvent::MediaExchangeIntention {
                id: self.track_id,
                enabled: matches!(
                    state,
                    media_exchange_state::Transition::Enabling(_)
                ),
            },
        );
    }

    /// Sends [`TrackEvent::MuteUpdateIntention`] with the provided
    /// [`mute_state`].
    pub fn send_mute_state_intention(&self, state: mute_state::Transition) {
        _ = self.track_events_sender.unbounded_send(
            TrackEvent::MuteUpdateIntention {
                id: self.track_id,
                muted: matches!(state, mute_state::Transition::Muting(_)),
            },
        );
    }
}

#[cfg(feature = "mockable")]
// TODO: Try remove on next Rust version upgrade.
#[expect(clippy::allow_attributes, reason = "`#[expect]` is not considered")]
#[allow(clippy::multiple_inherent_impl, reason = "feature gated")]
impl Sender {
    /// Indicates whether general media exchange state of this [`Sender`] is in
    /// [`StableMediaExchangeState::Disabled`].
    #[must_use]
    pub fn general_disabled(&self) -> bool {
        !self.enabled_general.get()
    }

    /// Indicates whether this [`Sender`] is disabled.
    #[must_use]
    pub fn disabled(&self) -> bool {
        !self.enabled_individual.get()
    }

    /// Indicates whether this [`Sender`] is muted.
    #[must_use]
    pub fn muted(&self) -> bool {
        self.muted.get()
    }
}

impl Drop for Sender {
    fn drop(&mut self) {
        let transceiver = self.transceiver.clone();
        platform::spawn(async move {
            if !transceiver.is_stopped() {
                transceiver.set_send(false).await;
                drop(transceiver.set_send_track(None).await);
            }
        });
    }
}
