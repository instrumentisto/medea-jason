//! Implementation of the `MediaTrack` with a `Recv` direction.

mod component;

use std::cell::{Cell, RefCell};

use futures::channel::mpsc;
use medea_client_api_proto as proto;
use proto::{ConnectionMode, TrackId};

#[doc(inline)]
pub use self::component::{Component, State};
use super::TransceiverSide as _;
use crate::{
    media::{MediaDirection, RecvConstraints, TrackConstraints, track::remote},
    peer::{
        MediaConnections, MediaStateControllable as _, PeerEvent, TrackEvent,
        media::media_exchange_state,
    },
    platform, utils,
};

/// Representation of a [`remote::Track`] that is being received from some
/// remote peer. It may have two states: `waiting` and `receiving`.
///
/// We can save related [`platform::Transceiver`] and the actual
/// [`remote::Track`] only when [`remote::Track`] data arrives.
#[derive(Debug)]
pub struct Receiver {
    /// ID of this [`remote::Track`].
    track_id: TrackId,

    /// Indicator whether this [`Receiver`] is working in a [P2P mesh] or [SFU]
    /// mode.
    ///
    /// [P2P mesh]: https://webrtcglossary.com/mesh
    /// [SFU]: https://webrtcglossary.com/sfu
    connection_mode: ConnectionMode,

    /// Constraints of this [`remote::Track`].
    caps: TrackConstraints,

    /// ID of the member sending this [`remote::Track`].
    sender_id: proto::MemberId,

    /// [`Transceiver`] associated with this [`remote::Track`].
    ///
    /// [`Transceiver`]: platform::Transceiver
    transceiver: RefCell<Option<platform::Transceiver>>,

    /// [MID] of the associated [`Transceiver`].
    ///
    /// [`Transceiver`]: platform::Transceiver
    /// [MID]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    mid: RefCell<Option<String>>,

    /// Actual [`remote::Track`] represented by this [`Receiver`].
    track: RefCell<Option<remote::Track>>,

    /// Indicator whether the actual [`remote::Track`] is updated with the
    /// current [`Receiver`]'s state.
    is_track_notified: Cell<bool>,

    /// Indicator whether this [`remote::Track`] is enabled generally.
    enabled_general: Cell<bool>,

    /// Indicator whether this [`remote::Track`] is enabled individually.
    enabled_individual: Cell<bool>,

    /// Current general [`MediaDirection`] of this [`Receiver`].
    media_direction: Cell<MediaDirection>,

    /// Indicator whether this [`remote::Track`] is muted.
    muted: Cell<bool>,

    /// Channel for sending [`PeerEvent`]s to the remote peer.
    peer_events_sender: mpsc::UnboundedSender<PeerEvent>,

    /// Channel for sending [`TrackEvent`]s to the actual [`remote::Track`].
    track_events_sender: mpsc::UnboundedSender<TrackEvent>,
}

impl Receiver {
    /// Creates a new [`platform::Transceiver`] if provided `mid` is [`None`],
    /// otherwise creates a [`Receiver`] without a [`platform::Transceiver`]. It
    /// will be injected when a [`remote::Track`] will arrive.
    ///
    /// Created [`platform::Transceiver`] direction is set to
    /// [`TransceiverDirection::INACTIVE`][1] if `enabled_individual` is
    /// `false`.
    ///
    /// `track` field in the created [`Receiver`] will be `None`, since
    /// [`Receiver`] must be created before the actual [`remote::Track`] data
    /// arrives.
    ///
    /// [1]: platform::TransceiverDirection::INACTIVE
    pub async fn new(
        state: &State,
        media_connections: &MediaConnections,
        track_events_sender: mpsc::UnboundedSender<TrackEvent>,
        recv_constraints: &RecvConstraints,
        connection_mode: ConnectionMode,
    ) -> Self {
        let caps = TrackConstraints::from(state.media_type().clone());

        #[expect(clippy::if_then_some_else_none, reason = "more readable")]
        let transceiver = if state.mid().is_none() {
            // Try to find send transceiver that can be used as `sendrecv`.
            let sender = media_connections
                .0
                .borrow()
                .senders
                .values()
                .find(|sndr| {
                    sndr.caps().media_kind() == caps.media_kind()
                        && sndr.caps().media_source_kind()
                            == caps.media_source_kind()
                })
                .map(utils::component::Component::obj);

            let trnsvr = if let Some(s) = sender {
                s.transceiver()
            } else {
                let new_transceiver =
                    media_connections.0.borrow().add_transceiver(
                        state.media_type().clone(),
                        platform::TransceiverDirection::INACTIVE,
                    );
                new_transceiver.await
            };
            trnsvr
                .set_recv(match connection_mode {
                    ConnectionMode::Mesh => state.enabled_individual(),
                    ConnectionMode::Sfu => true,
                })
                .await;
            Some(trnsvr)
        } else {
            None
        };

        let peer_events_sender =
            media_connections.0.borrow().peer_events_sender.clone();
        let this = Self {
            track_id: state.track_id(),
            connection_mode,
            caps,
            sender_id: state.sender_id().clone(),
            transceiver: RefCell::new(transceiver),
            mid: RefCell::new(state.mid().map(ToString::to_string)),
            track: RefCell::new(None),
            is_track_notified: Cell::new(false),
            peer_events_sender,
            enabled_general: Cell::new(state.enabled_individual()),
            enabled_individual: Cell::new(state.enabled_general()),
            muted: Cell::new(state.muted()),
            media_direction: Cell::new(state.media_direction()),
            track_events_sender,
        };

        let enabled_in_cons = match &state.media_type() {
            proto::MediaType::Audio(_) => {
                recv_constraints.is_audio_device_enabled()
                    || recv_constraints.is_audio_display_enabled()
            }
            proto::MediaType::Video(_) => {
                recv_constraints.is_video_device_enabled()
                    || recv_constraints.is_video_display_enabled()
            }
        };
        if !enabled_in_cons {
            state
                .media_exchange_state_controller()
                .transition_to(enabled_in_cons.into());
        }

        this
    }

    /// Returns [`TrackConstraints`] of this [`Receiver`].
    #[must_use]
    pub const fn caps(&self) -> &TrackConstraints {
        &self.caps
    }

    /// Returns [`mid`] of this [`Receiver`].
    ///
    /// [`mid`]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    #[must_use]
    pub fn mid(&self) -> Option<String> {
        if self.mid.borrow().is_none() && self.transceiver.borrow().is_some() {
            if let Some(transceiver) =
                self.transceiver.borrow().as_ref().cloned()
            {
                drop(self.mid.replace(Some(transceiver.mid()?)));
            }
        }
        self.mid.borrow().clone()
    }

    /// Indicates whether this [`Receiver`] receives media data.
    pub async fn is_receiving(&self) -> bool {
        let transceiver = self.transceiver.borrow().clone();
        let is_recv_direction = if let Some(trcv) = transceiver {
            trcv.has_direction(platform::TransceiverDirection::RECV).await
        } else {
            false
        };

        self.enabled_individual.get() && is_recv_direction
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

    /// Adds the provided [`platform::MediaStreamTrack`] and
    /// [`platform::Transceiver`] to this [`Receiver`].
    ///
    /// Sets [`platform::MediaStreamTrack::enabled`] same as
    /// `enabled_individual` of this [`Receiver`].
    pub async fn set_remote_track(
        &self,
        transceiver: platform::Transceiver,
        new_track: platform::MediaStreamTrack,
    ) {
        if let Some(old_track) = self.track.borrow().as_ref() {
            if old_track.id() == new_track.id() {
                return;
            }
        }

        self.set_transceiver(transceiver);

        let new_track = remote::Track::new(
            new_track,
            self.caps.media_source_kind(),
            self.muted.get(),
            self.media_direction.get(),
        );
        if let Some(prev_track) = self.track.replace(Some(new_track)) {
            platform::spawn(async move {
                prev_track.stop().await;
            });
        }

        // It's OK to `.clone()` here, as the `Transceiver` represents a pointer
        // to a garbage-collectable memory on each platform.
        let trnscvr = self.transceiver.borrow().as_ref().cloned();
        if let Some(t) = trnscvr {
            t.set_recv(match self.connection_mode {
                ConnectionMode::Mesh => self.enabled_individual.get(),
                ConnectionMode::Sfu => true,
            })
            .await;
        }

        self.maybe_notify_track().await;
    }

    /// Updates [`MediaDirection`] of this [`Receiver`].
    pub fn set_media_direction(&self, direction: MediaDirection) {
        self.media_direction.set(direction);
        if let Some(track) = self.track.borrow().as_ref().cloned() {
            track.set_media_direction(direction);
        }
    }

    /// Replaces [`Receiver`]'s [`platform::Transceiver`] with the provided
    /// [`platform::Transceiver`].
    ///
    /// Doesn't update [`platform::TransceiverDirection`] of the
    /// [`platform::Transceiver`].
    ///
    /// No-op if provided with the same [`platform::Transceiver`] as already
    /// exists in this [`Receiver`].
    pub fn set_transceiver(&self, transceiver: platform::Transceiver) {
        if self.transceiver.borrow().is_none()
            && self.mid.borrow().as_ref() == transceiver.mid().as_ref()
        {
            drop(self.transceiver.replace(Some(transceiver)));
        }
    }

    /// Returns a [`platform::Transceiver`] of this [`Receiver`].
    ///
    /// Returns [`None`] if this [`Receiver`] doesn't have a
    /// [`platform::Transceiver`].
    pub fn transceiver(&self) -> Option<platform::Transceiver> {
        self.transceiver.borrow().clone()
    }

    /// Emits [`PeerEvent::NewRemoteTrack`] if [`Receiver`] is receiving media
    /// and has not notified yet.
    async fn maybe_notify_track(&self) {
        if self.is_track_notified.get() {
            return;
        }
        if !self.is_receiving().await {
            return;
        }
        // Re-check since state might have changed during `.await`.
        if self.is_track_notified.get() {
            return;
        }
        if let Some(track) = self.track.borrow().as_ref() {
            drop(self.peer_events_sender.unbounded_send(
                PeerEvent::NewRemoteTrack {
                    sender_id: self.sender_id.clone(),
                    track: track.clone(),
                },
            ));
            self.is_track_notified.set(true);
        }
    }
}

#[cfg(feature = "mockable")]
// TODO: Try remove on next Rust version upgrade.
#[expect(clippy::allow_attributes, reason = "`#[expect]` is not considered")]
#[allow(clippy::multiple_inherent_impl, reason = "feature gated")]
impl Receiver {
    /// Returns the current `enabled_general` status of this [`Receiver`].
    #[must_use]
    pub fn enabled_general(&self) -> bool {
        self.enabled_general.get()
    }

    /// Returns the current `media_direction` status of this [`Receiver`].
    #[must_use]
    pub fn direction(&self) -> MediaDirection {
        self.media_direction.get()
    }
}

impl Drop for Receiver {
    fn drop(&mut self) {
        let transceiver = self.transceiver.borrow_mut().take();
        if let Some(transceiver) = transceiver {
            platform::spawn(async move {
                if !transceiver.is_stopped() {
                    transceiver.set_recv(false).await;
                }
            });
        }
        if let Some(recv_track) = self.track.borrow_mut().take() {
            platform::spawn(recv_track.stop());
        }
    }
}
