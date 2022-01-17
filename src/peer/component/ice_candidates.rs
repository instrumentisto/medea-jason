//! Implementation of a [`IceCandidate`]s store.

use std::{cell::RefCell, collections::HashSet};

use futures::stream::{self, LocalBoxStream};
use medea_client_api_proto::IceCandidate;
use medea_reactive::ObservableHashSet;

use crate::{
    media::LocalTracksConstraints,
    utils::{AsProtoState, SynchronizableState},
};

/// Store of all the [`IceCandidate`]s of a [`peer::Component`].
///
/// [`peer::Component`]: super::Component
#[derive(Debug)]
pub struct IceCandidates(RefCell<ObservableHashSet<IceCandidate>>);

impl IceCandidates {
    /// Returns a new empty [`IceCandidates`] store.
    #[must_use]
    pub fn new() -> Self {
        Self(RefCell::new(ObservableHashSet::new()))
    }

    /// Adds a new [`IceCandidate`] to this [`IceCandidates`] store.
    pub fn add(&self, candidate: IceCandidate) {
        let _ = self.0.borrow_mut().insert(candidate);
    }

    /// Returns [`LocalBoxStream`] with all the already added [`IceCandidate`]s
    /// and the [`IceCandidate`]s which will be added in future.
    pub fn on_add(&self) -> LocalBoxStream<'static, IceCandidate> {
        let this = self.0.borrow();
        Box::pin(stream::select(this.replay_on_insert(), this.on_insert()))
    }
}

impl SynchronizableState for IceCandidates {
    type Input = HashSet<IceCandidate>;

    fn from_proto(input: Self::Input, _: &LocalTracksConstraints) -> Self {
        Self(RefCell::new(input.into()))
    }

    fn apply(&self, input: Self::Input, _: &LocalTracksConstraints) {
        self.0.borrow_mut().update(input);
    }
}

impl AsProtoState for IceCandidates {
    type Output = HashSet<IceCandidate>;

    fn as_proto(&self) -> Self::Output {
        self.0.borrow().iter().cloned().collect()
    }
}
