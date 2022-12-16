//! Implementation of a store of [`sender::State`]s and [`receiver::State`]s.
//!
//! [`receiver::State`]: super::receiver::State

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use derive_more::From;
use futures::{
    future, future::LocalBoxFuture, stream::LocalBoxStream, FutureExt as _,
    TryFutureExt,
};
use medea_client_api_proto::TrackId;
use medea_reactive::{AllProcessed, Guarded, ProgressableHashMap};
use tracerr::Traced;

use crate::{
    media::LocalTracksConstraints,
    peer::UpdateLocalStreamError,
    utils::{AsProtoState, SynchronizableState, Updatable},
};

use super::sender;

/// Repository of all the [`sender::State`]s/[`receiver::State`]s of a
/// [`Component`].
///
/// [`Component`]: super::Component
/// [`receiver::State`]: super::receiver::State
#[derive(Debug, From)]
pub(crate) struct TracksRepository<S: 'static>(
    RefCell<ProgressableHashMap<TrackId, Rc<S>>>,
);

impl<S> TracksRepository<S> {
    /// Creates a new [`TracksRepository`].
    #[must_use]
    pub(crate) fn new() -> Self {
        Self(RefCell::new(ProgressableHashMap::new()))
    }

    /// Returns [`Future`] resolving once all inserts/removes are processed.
    ///
    /// [`Future`]: std::future::Future
    pub(crate) fn when_all_processed(&self) -> AllProcessed<'static> {
        self.0.borrow().when_all_processed()
    }

    /// Inserts the provided track identified by the given `id`.
    pub(crate) fn insert(&self, id: TrackId, track: Rc<S>) {
        drop(self.0.borrow_mut().insert(id, track));
    }

    /// Returns a track with the provided `id`.
    #[must_use]
    pub(crate) fn get(&self, id: TrackId) -> Option<Rc<S>> {
        self.0.borrow().get(&id).cloned()
    }

    /// Returns a [`Stream`] streaming all the [`TracksRepository::insert`]ions.
    ///
    /// [`Stream`]: futures::Stream
    pub(crate) fn on_insert(
        &self,
    ) -> LocalBoxStream<'static, Guarded<(TrackId, Rc<S>)>> {
        self.0.borrow().on_insert_with_replay()
    }

    /// Returns a [`Stream`] streaming all the [`TracksRepository::remove`]s.
    ///
    /// [`Stream`]: futures::Stream
    pub(crate) fn on_remove(
        &self,
    ) -> LocalBoxStream<'static, Guarded<(TrackId, Rc<S>)>> {
        self.0.borrow().on_remove()
    }

    /// Removes a track with the provided [`TrackId`], reporting whether it has
    /// been removed or it hasn't existed at all.
    pub(crate) fn remove(&self, id: TrackId) -> bool {
        self.0.borrow_mut().remove(&id).is_some()
    }
}

impl TracksRepository<sender::State> {
    /// Returns all the [`sender::State`]s which require a local `MediaStream`
    /// update.
    #[must_use]
    pub(crate) fn get_outdated(&self) -> Vec<Rc<sender::State>> {
        self.0
            .borrow()
            .values()
            .filter(|s| s.is_local_stream_update_needed())
            .cloned()
            .collect()
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
    /// [`Future`]: std::future::Future
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    /// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
    pub(crate) fn local_stream_update_result(
        &self,
        tracks_ids: HashSet<TrackId>,
    ) -> LocalBoxFuture<'static, Result<(), Traced<UpdateLocalStreamError>>>
    {
        let senders = self.0.borrow();
        Box::pin(
            future::try_join_all(tracks_ids.into_iter().filter_map(|id| {
                Some(
                    senders
                        .get(&id)?
                        .local_stream_update_result()
                        .map_err(tracerr::map_from_and_wrap!()),
                )
            }))
            .map(|r| r.map(drop)),
        )
    }
}

impl<S> SynchronizableState for TracksRepository<S>
where
    S: SynchronizableState,
{
    type Input = HashMap<TrackId, S::Input>;

    fn from_proto(
        input: Self::Input,
        send_cons: &LocalTracksConstraints,
    ) -> Self {
        Self(RefCell::new(
            input
                .into_iter()
                .map(|(id, t)| (id, Rc::new(S::from_proto(t, send_cons))))
                .collect(),
        ))
    }

    fn apply(&self, input: Self::Input, send_cons: &LocalTracksConstraints) {
        self.0.borrow_mut().remove_not_present(&input);

        for (id, track) in input {
            if let Some(sync_track) = self.0.borrow().get(&id) {
                sync_track.apply(track, send_cons);
            } else {
                drop(
                    self.0
                        .borrow_mut()
                        .insert(id, Rc::new(S::from_proto(track, send_cons))),
                );
            }
        }
    }
}

impl<S> Updatable for TracksRepository<S>
where
    S: Updatable,
{
    /// Returns [`Future`] resolving once all tracks from this
    /// [`TracksRepository`] will be stabilized meaning that all track's
    /// components won't contain any pending state change transitions.
    ///
    /// [`Future`]: std::future::Future
    fn when_stabilized(&self) -> AllProcessed<'static> {
        let when_futs: Vec<_> = self
            .0
            .borrow()
            .values()
            .map(|s| s.when_stabilized().into())
            .collect();
        medea_reactive::when_all_processed(when_futs)
    }

    /// Returns [`Future`] resolving once all tracks updates are applied.
    ///
    /// [`Future`]: std::future::Future
    fn when_updated(&self) -> AllProcessed<'static> {
        let when_futs: Vec<_> = self
            .0
            .borrow()
            .values()
            .map(|s| s.when_updated().into())
            .collect();
        medea_reactive::when_all_processed(when_futs)
    }

    /// Notifies all the tracks about RPC connection loss.
    fn connection_lost(&self) {
        self.0.borrow().values().for_each(|s| s.connection_lost());
    }

    /// Notifies all the tracks about RPC connection recovering.
    fn connection_recovered(&self) {
        self.0
            .borrow()
            .values()
            .for_each(|s| s.connection_recovered());
    }
}

impl<S> AsProtoState for TracksRepository<S>
where
    S: AsProtoState,
{
    type Output = HashMap<TrackId, S::Output>;

    fn as_proto(&self) -> Self::Output {
        self.0
            .borrow()
            .iter()
            .map(|(id, s)| (*id, s.as_proto()))
            .collect()
    }
}
