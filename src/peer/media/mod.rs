//! [`PeerConnection`] media management.
//!
//! [`PeerConnection`]: crate::peer::PeerConnection

pub mod receiver;
pub mod sender;
mod transitable_state;

use std::{cell::RefCell, collections::HashMap, future::Future, rc::Rc};

use derive_more::{Display, From};
use futures::{
    channel::mpsc, future, future::LocalBoxFuture, FutureExt as _,
    TryFutureExt as _,
};
use medea_client_api_proto as proto;
#[cfg(feature = "mockable")]
use medea_client_api_proto::{ConnectionMode, MediaType, MemberId};
use medea_client_api_proto::{EncodingParameters, SvcSettings};
use proto::{MediaSourceKind, TrackId};
use tracerr::Traced;

#[cfg(feature = "mockable")]
use crate::media::{LocalTracksConstraints, RecvConstraints};
use crate::{
    media::{track::local, MediaKind},
    peer::{LocalStreamUpdateCriteria, PeerEvent},
    platform,
    platform::{
        send_encoding_parameters::SendEncodingParameters, CodecCapability,
        TransceiverInit,
    },
    utils::{Caused, Component},
};

use super::tracks_request::TracksRequest;

#[doc(inline)]
pub use self::{
    receiver::Receiver,
    sender::Sender,
    transitable_state::{
        media_exchange_state, mute_state, InStable, InTransition,
        MediaExchangeState, MediaExchangeStateController, MediaState,
        MuteState, MuteStateController, TransitableState,
        TransitableStateController,
    },
};

/// Transceiver's sending ([`Sender`]) or receiving ([`Receiver`]) side.
pub trait TransceiverSide: MediaStateControllable {
    /// Returns [`TrackId`] of this [`TransceiverSide`].
    fn track_id(&self) -> TrackId;

    /// Returns [`MediaKind`] of this [`TransceiverSide`].
    fn kind(&self) -> MediaKind;

    /// Returns [`MediaSourceKind`] of this [`TransceiverSide`].
    fn source_kind(&self) -> MediaSourceKind;

    /// Returns `true` if this [`TransceiverSide`] currently can be
    /// disabled/enabled without [`LocalTracksConstraints`] updating.
    ///
    /// [`LocalTracksConstraints`]: super::LocalTracksConstraints
    fn is_transitable(&self) -> bool;
}

/// Default functions for dealing with [`MediaExchangeStateController`] and
/// [`MuteStateController`] for objects that use it.
pub trait MediaStateControllable {
    /// Returns reference to the [`MediaExchangeStateController`].
    #[must_use]
    fn media_exchange_state_controller(
        &self,
    ) -> Rc<MediaExchangeStateController>;

    /// Returns a reference to the [`MuteStateController`].
    #[must_use]
    fn mute_state_controller(&self) -> Rc<MuteStateController>;

    /// Returns [`MediaExchangeState`] of this [`MediaStateControllable`].
    fn media_exchange_state(&self) -> MediaExchangeState {
        self.media_exchange_state_controller().state()
    }

    /// Returns [`MuteState`] of this [`MediaStateControllable`].
    #[must_use]
    fn mute_state(&self) -> MuteState {
        self.mute_state_controller().state()
    }

    /// Sets current [`MediaState`] to [`TransitableState::Transition`].
    ///
    /// # Errors
    ///
    /// Implementors might return [`ProhibitedStateError`] if a transition
    /// cannot be made for some reason.
    fn media_state_transition_to(
        &self,
        desired_state: MediaState,
    ) -> Result<(), Traced<ProhibitedStateError>> {
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

    /// Indicates whether [`Room`] should subscribe to the [`MediaState`] update
    /// when updating [`MediaStateControllable`] to the provided [`MediaState`].
    ///
    /// [`Room`]: crate::room::Room
    fn is_subscription_needed(&self, desired_state: MediaState) -> bool {
        match desired_state {
            MediaState::MediaExchange(media_exchange) => {
                let current = self.media_exchange_state();
                match current {
                    MediaExchangeState::Transition(_) => true,
                    MediaExchangeState::Stable(stable) => {
                        stable != media_exchange
                    }
                }
            }
            MediaState::Mute(mute_state) => {
                let current = self.mute_state();
                match current {
                    MuteState::Transition(_) => true,
                    MuteState::Stable(stable) => stable != mute_state,
                }
            }
        }
    }

    /// Indicates whether [`Room`] should send [`TrackPatchCommand`] to the
    /// server when updating [`MediaStateControllable`] to the provided
    /// [`MediaState`].
    ///
    /// [`TrackPatchCommand`]: medea_client_api_proto::TrackPatchCommand
    /// [`Room`]: crate::room::Room
    #[must_use]
    fn is_track_patch_needed(&self, desired_state: MediaState) -> bool {
        match desired_state {
            MediaState::MediaExchange(media_exchange) => {
                let current = self.media_exchange_state();
                match current {
                    MediaExchangeState::Stable(stable) => {
                        stable != media_exchange
                    }
                    MediaExchangeState::Transition(transition) => {
                        transition.intended() != media_exchange
                    }
                }
            }
            MediaState::Mute(mute_state) => {
                let current = self.mute_state();
                match current {
                    MuteState::Stable(stable) => stable != mute_state,
                    MuteState::Transition(transition) => {
                        transition.intended() != mute_state
                    }
                }
            }
        }
    }

    /// Returns [`Future`] which will be resolved when [`MediaState`] of this
    /// [`MediaStateControllable`] will be [`TransitableState::Stable`] or it's
    /// dropped.
    ///
    /// # Errors
    ///
    /// With an approved stable [`MediaState`] if transition to the
    /// `desired_state` cannot be made.
    ///
    /// [`Future`]: std::future::Future
    /// [`MediaState`]: super::MediaState
    fn when_media_state_stable(
        &self,
        desired_state: MediaState,
    ) -> LocalBoxFuture<'static, Result<(), MediaState>> {
        match desired_state {
            MediaState::Mute(desired_state) => self
                .mute_state_controller()
                .when_media_state_stable(desired_state)
                .map_err(MediaState::Mute)
                .boxed_local(),
            MediaState::MediaExchange(desired_state) => self
                .media_exchange_state_controller()
                .when_media_state_stable(desired_state)
                .map_err(MediaState::MediaExchange)
                .boxed_local(),
        }
    }
}

/// Direction of the `MediaTrack`.
#[derive(Clone, Copy, Debug)]
pub enum TrackDirection {
    /// Sends media data.
    Send,

    /// Receives media data.
    Recv,
}

/// Error occurring when media state transition is not allowed.
#[derive(Clone, Copy, Debug, Display)]
pub enum ProhibitedStateError {
    /// [`Sender`] cannot be disabled because it's required.
    #[display(fmt = "MediaExchangeState of Sender can't transit to \
                     disabled state, because this Sender is required.")]
    CannotDisableRequiredSender,
}

/// Errors occurring in [`MediaConnections::insert_local_tracks()`] method.
#[derive(Caused, Clone, Debug, Display, From)]
#[cause(error = platform::Error)]
pub enum InsertLocalTracksError {
    /// [`local::Track`] doesn't satisfy [`Sender`]'s constraints.
    #[display(fmt = "Provided Track doesn't satisfy senders constraints")]
    InvalidMediaTrack,

    /// There are not enough [`local::Track`]s being inserted into [`Sender`]s.
    #[display(fmt = "Provided stream does not have all necessary Tracks")]
    NotEnoughTracks,

    /// Insertion of a [`local::Track`] into a [`Sender`] fails.
    CouldNotInsertLocalTrack(#[cause] sender::InsertTrackError),
}

/// Errors occurring in [`MediaConnections::get_mids()`] method.
#[derive(Clone, Copy, Debug, Display)]
pub enum GetMidsError {
    /// Cannot get the `mid` from a [`Sender`].
    #[display(fmt = "Peer has senders without mid")]
    SendersWithoutMid,

    /// Cannot get the `mid` from a [`Receiver`].
    #[display(fmt = "Peer has receivers without mid")]
    ReceiversWithoutMid,
}

/// Actual data of [`MediaConnections`] storage.
#[derive(Debug)]
struct InnerMediaConnections {
    /// Reference to the parent [`platform::RtcPeerConnection`].
    ///
    /// Used to generate transceivers for [`Sender`]s and [`Receiver`]s.
    peer: Rc<platform::RtcPeerConnection>,

    /// [`PeerEvent`]s tx.
    peer_events_sender: mpsc::UnboundedSender<PeerEvent>,

    /// [`TrackId`] to its [`sender::Component`].
    senders: HashMap<TrackId, sender::Component>,

    /// [`TrackId`] to its [`receiver::Component`].
    receivers: HashMap<TrackId, receiver::Component>,
}

impl InnerMediaConnections {
    /// Returns [`Iterator`] over [`sender::Component`]s with provided
    /// [`MediaKind`] and [`MediaSourceKind`].
    fn iter_senders_with_kind_and_source_kind(
        &self,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
    ) -> impl Iterator<Item = &sender::Component> {
        self.senders
            .values()
            .filter(move |sender| sender.state().kind() == kind)
            .filter(move |sender| {
                source_kind
                    .map_or(true, |sk| sender.caps().media_source_kind() == sk)
            })
    }

    /// Returns [`Iterator`] over [`receiver::Component`]s with provided
    /// [`MediaKind`] and [`MediaSourceKind`].
    fn iter_receivers_with_kind_and_source_kind(
        &self,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
    ) -> impl Iterator<Item = &receiver::Component> {
        self.receivers
            .values()
            .filter(move |s| s.state().kind() == kind)
            .filter(move |s| {
                source_kind
                    .map_or(true, |skind| s.state().source_kind() == skind)
            })
    }

    /// Returns all [`TransceiverSide`]s by provided [`TrackDirection`],
    /// [`MediaKind`] and [`MediaSourceKind`].
    #[allow(clippy::as_conversions)]
    fn get_transceivers_by_direction_and_kind(
        &self,
        direction: TrackDirection,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
    ) -> Vec<Rc<dyn TransceiverSide>> {
        match direction {
            TrackDirection::Send => self
                .iter_senders_with_kind_and_source_kind(kind, source_kind)
                .map(|tx| tx.state() as Rc<dyn TransceiverSide>)
                .collect(),
            TrackDirection::Recv => self
                .iter_receivers_with_kind_and_source_kind(kind, source_kind)
                .map(|rx| rx.state() as Rc<dyn TransceiverSide>)
                .collect(),
        }
    }

    /// Creates a [`platform::Transceiver`] and adds it to the
    /// [`platform::RtcPeerConnection`].
    fn add_transceiver(
        &self,
        kind: MediaKind,
        direction: platform::TransceiverDirection,
        encodings: Vec<EncodingParameters>,
        svc: Vec<SvcSettings>,
    ) -> impl Future<Output = platform::Transceiver> + 'static {
        let peer = self.peer.clone();
        async move {
            let mut init = TransceiverInit::new(direction);
            let codecs = CodecCapability::get_sender_codec_capabilities(kind)
                .await
                .unwrap();
            let mut target_scalability_mode = None;
            let mut target_codecs = Vec::new();

            for svc_setting in svc {
                let res = codecs.iter().find(|codec| {
                    let mime = codec.mime_type().unwrap();
                    let svc_mime =
                        format!("video/{}", svc_setting.codec.to_string());
                    mime == svc_mime
                        || mime == "video/rtx"
                        || mime == "video/red"
                        || mime == "video/ulpfec"
                });

                if let Some(res) = res {
                    target_codecs.push(res.clone());
                    target_scalability_mode =
                        Some(svc_setting.scalability_mode);
                }
            }

            if encodings.is_empty() && target_scalability_mode.is_some() {
                let mut enc =
                    SendEncodingParameters::from(EncodingParameters {
                        rid: "0".to_owned(),
                        active: true,
                        max_bitrate: None,
                        scale_resolution_down_by: None,
                    });
                if let Some(t) = target_scalability_mode {
                    enc.set_scalability_mode(t);
                }
                init.sending_encodings(vec![enc]);
            } else {
                encodings
                    .into_iter()
                    .map(SendEncodingParameters::from)
                    .for_each(|mut enc| {
                        if let Some(target_sm) = target_scalability_mode {
                            enc.set_scalability_mode(target_sm);
                        }
                        init.sending_encodings(vec![enc]);
                    });
            };

            // TODO(evdokimovs): Remove unwrap
            let transceiver = peer.add_transceiver(kind, init).await.unwrap();
            transceiver.set_preferred_codecs(target_codecs);
            transceiver
        }
    }

    /// Lookups a [`platform::Transceiver`] by the provided [`mid`].
    ///
    /// [`mid`]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    fn get_transceiver_by_mid(
        &self,
        mid: String,
    ) -> impl Future<Output = Option<platform::Transceiver>> + 'static {
        self.peer.get_transceiver_by_mid(mid)
    }
}

/// Storage of [`platform::RtcPeerConnection`]'s [`sender::Component`] and
/// [`receiver::Component`].
#[derive(Debug)]
pub struct MediaConnections(RefCell<InnerMediaConnections>);

impl MediaConnections {
    /// Instantiates a new [`MediaConnections`] storage for the given
    /// [`platform::RtcPeerConnection`].
    #[must_use]
    pub fn new(
        peer: Rc<platform::RtcPeerConnection>,
        peer_events_sender: mpsc::UnboundedSender<PeerEvent>,
    ) -> Self {
        Self(RefCell::new(InnerMediaConnections {
            peer,
            peer_events_sender,
            senders: HashMap::new(),
            receivers: HashMap::new(),
        }))
    }

    /// Returns all [`Sender`]s and [`Receiver`]s from this [`MediaConnections`]
    /// with provided [`MediaKind`], [`TrackDirection`] and
    /// [`MediaSourceKind`].
    pub fn get_transceivers_sides(
        &self,
        kind: MediaKind,
        direction: TrackDirection,
        source_kind: Option<MediaSourceKind>,
    ) -> Vec<Rc<dyn TransceiverSide>> {
        self.0.borrow().get_transceivers_by_direction_and_kind(
            direction,
            kind,
            source_kind,
        )
    }

    /// Indicates whether all [`TransceiverSide`]s with provided [`MediaKind`],
    /// [`TrackDirection`] and [`MediaSourceKind`] is in the provided
    /// [`MediaExchangeState`].
    #[must_use]
    pub fn is_all_tracks_in_media_state(
        &self,
        kind: MediaKind,
        direction: TrackDirection,
        source_kind: Option<MediaSourceKind>,
        state: MediaState,
    ) -> bool {
        let transceivers =
            self.0.borrow().get_transceivers_by_direction_and_kind(
                direction,
                kind,
                source_kind,
            );
        for transceiver in transceivers {
            if !transceiver.is_transitable() {
                continue;
            }

            let not_in_state = match state {
                MediaState::Mute(mute_state) => {
                    transceiver.mute_state() != mute_state.into()
                }
                MediaState::MediaExchange(media_exchange) => {
                    transceiver.media_exchange_state() != media_exchange.into()
                }
            };
            if not_in_state {
                return false;
            }
        }

        true
    }

    /// Returns mapping from a [`proto::Track`] ID to a `mid` of this track's
    /// [`platform::Transceiver`].
    ///
    /// # Errors
    ///
    /// See [`GetMidsError`] for details.
    pub fn get_mids(
        &self,
    ) -> Result<HashMap<TrackId, String>, Traced<GetMidsError>> {
        let inner = self.0.borrow();
        let mut mids =
            HashMap::with_capacity(inner.senders.len() + inner.receivers.len());
        #[allow(clippy::iter_over_hash_type)] // order doesn't matter here
        for (track_id, sender) in &inner.senders {
            drop(
                mids.insert(
                    *track_id,
                    sender
                        .mid()
                        .ok_or(GetMidsError::SendersWithoutMid)
                        .map_err(tracerr::wrap!())?,
                ),
            );
        }
        #[allow(clippy::iter_over_hash_type)] // order doesn't matter here
        for (track_id, receiver) in &inner.receivers {
            drop(
                mids.insert(
                    *track_id,
                    receiver
                        .mid()
                        .ok_or(GetMidsError::ReceiversWithoutMid)
                        .map_err(tracerr::wrap!())?,
                ),
            );
        }
        Ok(mids)
    }

    /// Returns activity statuses of the all the [`Sender`]s and [`Receiver`]s
    /// from these [`MediaConnections`].
    pub fn get_transceivers_statuses(
        &self,
    ) -> impl Future<Output = HashMap<TrackId, bool>> + 'static {
        let inner = self.0.borrow();
        let transceivers = inner
            .senders
            .iter()
            .map(|(&track_id, sender)| {
                let sender = sender.obj();
                async move { (track_id, sender.is_publishing().await) }
                    .boxed_local()
            })
            .chain(inner.receivers.iter().map(|(&track_id, receiver)| {
                let receiver = receiver.obj();
                async move { (track_id, receiver.is_receiving().await) }
                    .boxed_local()
            }))
            .collect::<Vec<_>>();

        future::join_all(transceivers).map(|r| r.into_iter().collect())
    }

    /// Returns [`Rc`] to [`TransceiverSide`] with a provided [`TrackId`].
    ///
    /// Returns `None` if [`TransceiverSide`] with a provided [`TrackId`]
    /// doesn't exists in this [`MediaConnections`].
    #[allow(clippy::as_conversions)]
    pub fn get_transceiver_side_by_id(
        &self,
        track_id: TrackId,
    ) -> Option<Rc<dyn TransceiverSide>> {
        let inner = self.0.borrow();
        inner
            .senders
            .get(&track_id)
            .map(|sndr| sndr.state() as Rc<dyn TransceiverSide>)
            .or_else(|| {
                inner
                    .receivers
                    .get(&track_id)
                    .map(|rcvr| rcvr.state() as Rc<dyn TransceiverSide>)
            })
    }

    /// Inserts new [`sender::Component`] into [`MediaConnections`].
    pub fn insert_sender(&self, sender: sender::Component) {
        drop(
            self.0
                .borrow_mut()
                .senders
                .insert(sender.state().id(), sender),
        );
    }

    /// Inserts new [`receiver::Component`] into [`MediaConnections`].
    pub fn insert_receiver(&self, receiver: receiver::Component) {
        drop(
            self.0
                .borrow_mut()
                .receivers
                .insert(receiver.state().id(), receiver),
        );
    }

    /// Returns [`TracksRequest`] based on [`Sender`]s in this
    /// [`MediaConnections`]. [`Sender`]s are chosen based on provided
    /// [`LocalStreamUpdateCriteria`].
    pub fn get_tracks_request(
        &self,
        kinds: LocalStreamUpdateCriteria,
    ) -> Option<TracksRequest> {
        let mut stream_request = None;
        #[allow(clippy::iter_over_hash_type)] // order doesn't matter here
        for sender in self.0.borrow().senders.values() {
            if kinds
                .has(sender.state().media_kind(), sender.state().media_source())
            {
                stream_request
                    .get_or_insert_with(TracksRequest::default)
                    .add_track_request(
                        sender.state().track_id(),
                        sender.caps().clone(),
                    );
            }
        }
        stream_request
    }

    /// Inserts provided tracks into [`Sender`]s based on track IDs.
    ///
    /// [`local::Track`]s are inserted into [`Sender`]'s
    /// [`platform::Transceiver`]s via a [`replaceTrack` method][1], changing
    /// its direction to `sendonly`.
    ///
    /// Returns [`HashMap`] with [`media_exchange_state::Stable`]s updates for
    /// the [`Sender`]s.
    ///
    /// # Errors
    ///
    /// See [`InsertLocalTracksError`] for details.
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpsender-replacetrack
    pub async fn insert_local_tracks(
        &self,
        tracks: &HashMap<TrackId, Rc<local::Track>>,
    ) -> Result<
        HashMap<TrackId, media_exchange_state::Stable>,
        Traced<InsertLocalTracksError>,
    > {
        // Build sender to track pairs to catch errors before inserting.
        let mut sender_and_track =
            Vec::with_capacity(self.0.borrow().senders.len());
        let mut media_exchange_state_updates = HashMap::new();
        let senders = self
            .0
            .borrow()
            .senders
            .values()
            .map(|c| (c.obj(), c.state()))
            .collect::<Vec<_>>();
        for (sender, state) in senders {
            if let Some(track) = tracks.get(&state.id()).cloned() {
                if sender.caps().satisfies(track.as_ref()).await {
                    sender_and_track.push((sender, track));
                } else {
                    return Err(tracerr::new!(
                        InsertLocalTracksError::InvalidMediaTrack
                    ));
                }
            } else if sender.caps().required() {
                return Err(tracerr::new!(
                    InsertLocalTracksError::NotEnoughTracks
                ));
            } else {
                _ = media_exchange_state_updates
                    .insert(state.id(), media_exchange_state::Stable::Disabled);
            }
        }

        future::try_join_all(sender_and_track.into_iter().map(
            |(sender, track)| async move {
                Rc::clone(&sender).insert_track(track).await
            },
        ))
        .await
        .map(drop)
        .map_err(tracerr::map_from_and_wrap!())?;

        Ok(media_exchange_state_updates)
    }

    /// Adds a new track to the corresponding [`Receiver`].
    ///
    /// # Errors
    ///
    /// Errors with a transceivers `mid` if could not find [`Receiver`] by this
    /// `mid`.
    ///
    /// # Panics
    ///
    /// If the provided [`platform::Transceiver`] doesn't have a [`mid`]. Not
    /// supposed to happen, since [`platform::MediaStreamTrack`] is only fired
    /// when a [`platform::Transceiver`] is negotiated, thus have a [`mid`].
    ///
    /// [`mid`]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    pub async fn add_remote_track(
        &self,
        track: platform::MediaStreamTrack,
        transceiver: platform::Transceiver,
    ) -> Result<(), String> {
        // Cannot fail, since transceiver is guaranteed to be negotiated at this
        // point.
        let mid = transceiver.mid().ok_or("No Transceiver::mid found")?;
        let receiver = self
            .0
            .borrow()
            .receivers
            .values()
            .find(|rcvr| rcvr.mid().as_ref() == Some(&mid))
            .map(Component::obj);

        if let Some(rcvr) = receiver {
            rcvr.set_remote_track(transceiver, track).await;
            Ok(())
        } else {
            Err(mid)
        }
    }

    /// Iterates over all [`Receiver`]s with [`mid`] and without
    /// [`platform::Transceiver`], trying to find the corresponding
    /// [`platform::Transceiver`] in the [`platform::RtcPeerConnection`] and to
    /// insert it into the [`Receiver`].
    ///
    /// [`mid`]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    pub fn sync_receivers(&self) -> impl Future<Output = ()> + 'static {
        future::join_all({
            self.0
                .borrow()
                .receivers
                .values()
                .filter_map(|receiver| {
                    // Suppress Clippy warn because this impl is easier to
                    // follow.
                    #[allow(clippy::question_mark)]
                    if receiver.transceiver().is_none() {
                        return None;
                    }
                    receiver.mid().map(|mid| {
                        let fut = {
                            self.0.borrow().peer.get_transceiver_by_mid(mid)
                        };
                        let receiver = Component::obj(receiver);
                        async move {
                            if let Some(t) = fut.await {
                                receiver.set_transceiver(t);
                            }
                        }
                    })
                })
                .collect::<Vec<_>>()
        })
        .map(drop)
    }

    /// Returns all [`Sender`]s which are matches provided
    /// [`LocalStreamUpdateCriteria`] and doesn't have [`local::Track`].
    pub fn get_senders_without_tracks_ids(
        &self,
        kinds: LocalStreamUpdateCriteria,
    ) -> Vec<TrackId> {
        self.0
            .borrow()
            .senders
            .values()
            .filter_map(|s| {
                (kinds.has(s.state().kind(), s.state().source_kind())
                    && s.state().enabled()
                    && !s.has_track())
                .then_some(s.state().id())
            })
            .collect()
    }

    /// Drops [`local::Track`]s of all [`Sender`]s which are matches provided
    /// [`LocalStreamUpdateCriteria`].
    pub async fn drop_send_tracks(&self, kinds: LocalStreamUpdateCriteria) {
        let remove_tracks_fut = future::join_all(
            self.0
                .borrow()
                .senders
                .values()
                .filter(|&s| {
                    kinds.has(s.state().kind(), s.state().source_kind())
                })
                .map(|s| {
                    let sender = s.obj();
                    async move {
                        sender.remove_track().await;
                    }
                }),
        );
        drop(remove_tracks_fut.await);
    }

    /// Removes a [`sender::Component`] or a [`receiver::Component`] with the
    /// provided [`TrackId`] from these [`MediaConnections`].
    pub fn remove_track(&self, track_id: TrackId) {
        let mut inner = self.0.borrow_mut();
        if inner.receivers.remove(&track_id).is_none() {
            drop(inner.senders.remove(&track_id));
        }
    }
}

#[cfg(feature = "mockable")]
#[allow(clippy::multiple_inherent_impl)]
impl MediaConnections {
    /// Indicates whether all [`Receiver`]s with [`MediaKind::Video`] are
    /// enabled.
    #[must_use]
    pub fn is_recv_video_enabled(&self) -> bool {
        !self
            .0
            .borrow()
            .iter_receivers_with_kind_and_source_kind(MediaKind::Video, None)
            .any(|s| !s.state().enabled_individual())
    }

    /// Indicates whether if all [`Receiver`]s with [`MediaKind::Audio`] are
    /// enabled.
    #[must_use]
    pub fn is_recv_audio_enabled(&self) -> bool {
        !self
            .0
            .borrow()
            .iter_receivers_with_kind_and_source_kind(MediaKind::Audio, None)
            .any(|s| !s.state().enabled_individual())
    }

    /// Returns [`Receiver`] with the provided [`TrackId`].
    #[must_use]
    pub fn get_receiver_by_id(&self, id: TrackId) -> Option<Rc<Receiver>> {
        self.0.borrow().receivers.get(&id).map(Component::obj)
    }

    /// Returns [`Sender`] with a provided [`TrackId`].
    #[must_use]
    pub fn get_sender_by_id(&self, id: TrackId) -> Option<Rc<Sender>> {
        self.0.borrow().senders.get(&id).map(Component::obj)
    }

    /// Indicates whether all [`Sender`]s with [`MediaKind::Audio`] are enabled.
    #[must_use]
    pub fn is_send_audio_enabled(&self) -> bool {
        self.0
            .borrow()
            .iter_senders_with_kind_and_source_kind(MediaKind::Audio, None)
            .all(|s| s.state().enabled())
    }

    /// Indicates whether all [`Sender`]s with [`MediaKind::Video`] are enabled.
    #[must_use]
    pub fn is_send_video_enabled(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> bool {
        self.0
            .borrow()
            .iter_senders_with_kind_and_source_kind(
                MediaKind::Video,
                source_kind,
            )
            .all(|s| s.state().enabled())
    }

    /// Indicates whether all [`Sender`]'s video tracks are unmuted.
    #[must_use]
    pub fn is_send_video_unmuted(
        &self,
        source_kind: Option<MediaSourceKind>,
    ) -> bool {
        !self
            .0
            .borrow()
            .iter_senders_with_kind_and_source_kind(
                MediaKind::Video,
                source_kind,
            )
            .any(|s| s.muted())
    }

    /// Indicates whether all [`Sender`]'s audio tracks are unmuted.
    #[must_use]
    pub fn is_send_audio_unmuted(&self) -> bool {
        !self
            .0
            .borrow()
            .iter_senders_with_kind_and_source_kind(MediaKind::Audio, None)
            .any(|s| s.muted())
    }

    /// Creates a new [`sender::Component`] with the provided data.
    ///
    /// # Errors
    ///
    /// See [`sender::CreateError`] for details.
    #[allow(clippy::too_many_arguments)] // TODO: refactor
    pub async fn create_sender(
        &self,
        id: TrackId,
        media_type: MediaType,
        media_direction: proto::MediaDirection,
        muted: bool,
        mid: Option<String>,
        receivers: Vec<MemberId>,
        send_constraints: &LocalTracksConstraints,
        connection_mode: ConnectionMode,
    ) -> Result<sender::Component, Traced<sender::CreateError>> {
        let sender_state = sender::State::new(
            id,
            mid,
            media_type,
            media_direction,
            muted,
            receivers,
            send_constraints.clone(),
            connection_mode,
        );
        let sender = sender::Sender::new(
            &sender_state,
            self,
            send_constraints.clone(),
            mpsc::unbounded().0,
        )
        .await?;

        Ok(sender::Component::new(sender, Rc::new(sender_state)))
    }

    /// Creates a new [`receiver::Component`] with the provided data.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_receiver(
        &self,
        id: TrackId,
        media_type: MediaType,
        media_direction: proto::MediaDirection,
        muted: bool,
        mid: Option<String>,
        sender: MemberId,
        recv_constraints: &RecvConstraints,
        connection_mode: ConnectionMode,
    ) -> receiver::Component {
        let state = receiver::State::new(
            id,
            mid,
            media_type,
            media_direction,
            muted,
            sender,
            connection_mode,
        );
        let receiver = receiver::Receiver::new(
            &state,
            self,
            mpsc::unbounded().0,
            recv_constraints,
            connection_mode,
        )
        .await;

        receiver::Component::new(Rc::new(receiver), Rc::new(state))
    }

    /// Creates a new [`sender::Component`]s/[`receiver::Component`]s from the
    /// provided [`proto::Track`]s.
    ///
    /// # Errors
    ///
    /// See [`sender::CreateError`] for details.
    pub async fn create_tracks(
        &self,
        tracks: Vec<proto::Track>,
        send_constraints: &LocalTracksConstraints,
        recv_constraints: &RecvConstraints,
        connection_mode: ConnectionMode,
    ) -> Result<(), Traced<sender::CreateError>> {
        for track in tracks {
            match track.direction {
                proto::Direction::Send { mid, receivers } => {
                    let is_muted = send_constraints.muted(&track.media_type);
                    let component = self
                        .create_sender(
                            track.id,
                            track.media_type,
                            proto::MediaDirection::SendRecv,
                            is_muted,
                            mid,
                            receivers,
                            send_constraints,
                            connection_mode,
                        )
                        .await?;
                    drop(
                        self.0.borrow_mut().senders.insert(track.id, component),
                    );
                }
                proto::Direction::Recv { mid, sender } => {
                    let component = self
                        .create_receiver(
                            track.id,
                            track.media_type,
                            proto::MediaDirection::SendRecv,
                            false,
                            mid,
                            sender,
                            recv_constraints,
                            connection_mode,
                        )
                        .await;
                    drop(
                        self.0
                            .borrow_mut()
                            .receivers
                            .insert(track.id, component),
                    );
                }
            }
        }
        Ok(())
    }

    /// Returns all underlying [`Sender`]'s.
    pub fn get_senders(&self) -> Vec<Rc<Sender>> {
        self.0
            .borrow()
            .senders
            .values()
            .map(Component::obj)
            .collect()
    }

    /// Returns [`sender::State`] with the provided [`TrackId`].
    #[must_use]
    pub fn get_sender_state_by_id(
        &self,
        id: TrackId,
    ) -> Option<Rc<sender::State>> {
        self.0.borrow().senders.get(&id).map(Component::state)
    }
}
