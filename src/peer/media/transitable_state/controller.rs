//! Component managing [`TransitableState`].

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};

use futures::{
    FutureExt as _, StreamExt as _, future, future::Either,
    stream::LocalBoxStream,
};
use medea_reactive::{Processed, ProgressableCell};

use super::TransitableState;
use crate::{
    peer::media::transitable_state::{
        InStable, InTransition, media_exchange_state, mute_state,
    },
    platform,
    utils::{ResettableDelayHandle, resettable_delay_for},
};

/// [`TransitableStateController`] for the [`mute_state`].
pub type MuteStateController =
    TransitableStateController<mute_state::Stable, mute_state::Transition>;

/// [`TransitableStateController`] for the [`media_exchange_state`].
pub type MediaExchangeStateController = TransitableStateController<
    media_exchange_state::Stable,
    media_exchange_state::Transition,
>;

/// Component managing all kinds of [`TransitableState`].
#[derive(Debug)]
pub struct TransitableStateController<S, T> {
    /// Actual [`TransitableState`].
    state: ProgressableCell<TransitableState<S, T>>,

    /// Timeout of the [`TransitableStateController::state`] transition.
    timeout_handle: RefCell<Option<ResettableDelayHandle>>,

    /// Indicator whether [`TransitableStateController::timeout_handle`]'s
    /// timeout is stopped.
    is_transition_timeout_stopped: Cell<bool>,
}

impl<S, T> TransitableStateController<S, T>
where
    S: InStable<Transition = T> + Into<TransitableState<S, T>> + 'static,
    T: InTransition<Stable = S> + Into<TransitableState<S, T>> + 'static,
{
    /// Timeout for the current state to transit into the desired one.
    const TRANSITION_TIMEOUT: Duration = {
        #[cfg(not(feature = "mockable"))]
        {
            Duration::from_secs(10)
        }
        #[cfg(feature = "mockable")]
        {
            Duration::from_millis(500)
        }
    };

    /// Returns new [`TransitableStateController`] with the provided
    /// stable state.
    #[must_use]
    pub fn new(state: S) -> Rc<Self> {
        let this = Rc::new(Self {
            state: ProgressableCell::new(state.into()),
            timeout_handle: RefCell::new(None),
            is_transition_timeout_stopped: Cell::new(false),
        });
        Rc::clone(&this).spawn();
        this
    }

    /// Spawns all the required [`Stream`] listeners for this
    /// [`TransitableStateController`].
    ///
    /// [`Stream`]: futures::Stream
    fn spawn(self: Rc<Self>) {
        // We don't care about initial state, be cause transceiver is inactive
        // at that moment.
        let mut state_changes = self.state.subscribe().skip(1);
        let weak_self = Rc::downgrade(&self);
        platform::spawn(async move {
            while let Some(state) = state_changes.next().await {
                let (state, _guard) = state.into_parts();
                if let Some(this) = weak_self.upgrade() {
                    if let TransitableState::Transition(_) = state {
                        let weak_this = Rc::downgrade(&this);
                        platform::spawn(async move {
                            let mut states = this.state.subscribe().skip(1);
                            let (timeout, timeout_handle) =
                                resettable_delay_for(
                                    Self::TRANSITION_TIMEOUT,
                                    this.is_transition_timeout_stopped.get(),
                                );
                            drop(
                                this.timeout_handle
                                    .borrow_mut()
                                    .replace(timeout_handle),
                            );
                            match future::select(
                                states.next(),
                                Box::pin(timeout),
                            )
                            .await
                            {
                                Either::Left(_) => (),
                                Either::Right(_) => {
                                    #[expect( // false positive
                                        clippy::shadow_unrelated,
                                        reason = "actually related"
                                    )]
                                    if let Some(this) = weak_this.upgrade() {
                                        let stable = this
                                            .state
                                            .get()
                                            .cancel_transition();
                                        this.state.set(stable);
                                    }
                                }
                            }
                        });
                    }
                } else {
                    break;
                }
            }
        });
    }

    /// Returns [`Stream`] into which the [`TransitableState::Stable`] updates
    /// will be emitted.
    ///
    /// [`Stream`]: futures::Stream
    pub fn subscribe_stable(&self) -> LocalBoxStream<'static, S> {
        self.state
            .subscribe()
            .filter_map(async |s| {
                let (s, _guard) = s.into_parts();
                if let TransitableState::Stable(stable) = s {
                    Some(stable)
                } else {
                    None
                }
            })
            .boxed_local()
    }

    /// Returns [`Stream`] into which the [`TransitableState::Transition`]
    /// updates will be emitted.
    ///
    /// [`Stream`]: futures::Stream
    pub fn subscribe_transition(&self) -> LocalBoxStream<'static, T> {
        self.state
            .subscribe()
            .filter_map(async |s| {
                let (s, _guard) = s.into_parts();
                if let TransitableState::Transition(transition) = s {
                    Some(transition)
                } else {
                    None
                }
            })
            .boxed_local()
    }

    /// Stops disable/enable timeout of this [`TransitableStateController`].
    pub fn stop_transition_timeout(&self) {
        self.is_transition_timeout_stopped.set(true);
        if let Some(timer) = &*self.timeout_handle.borrow() {
            timer.stop();
        }
    }

    /// Resets disable/enable timeout of this [`TransitableStateController`].
    pub fn reset_transition_timeout(&self) {
        self.is_transition_timeout_stopped.set(false);
        if let Some(timer) = &*self.timeout_handle.borrow() {
            timer.reset();
        }
    }

    /// Returns current [`TransitableStateController::state`].
    #[must_use]
    pub fn state(&self) -> TransitableState<S, T> {
        self.state.get()
    }

    /// Starts transition of the [`TransitableStateController::state`] to the
    /// provided one.
    pub fn transition_to(&self, desired_state: S) {
        let current_state = self.state.get();
        self.state.set(current_state.transition_to(desired_state));
    }

    /// Returns [`Future`] which will be resolved when state of this
    /// [`TransitableStateController`] will be [`TransitableState::Stable`] or
    /// the [`TransitableStateController`] is dropped.
    ///
    /// Succeeds if [`TransitableStateController`]'s state transits into the
    /// `desired_state` or the [`TransitableStateController`] is dropped.
    ///
    /// # Errors
    ///
    /// With an approved stable [`MediaState`] if transition to the
    /// `desired_state` cannot be made.
    ///
    /// [`MediaState`]: super::MediaState
    pub fn when_media_state_stable(
        &self,
        desired_state: S,
    ) -> future::LocalBoxFuture<'static, Result<(), S>> {
        let mut states = self.state.subscribe();
        async move {
            while let Some(state) = states.next().await {
                let (state, _guard) = state.into_parts();
                match state {
                    TransitableState::Transition(_) => {}
                    TransitableState::Stable(s) => {
                        return if s == desired_state {
                            Ok(())
                        } else {
                            Err(s)
                        };
                    }
                }
            }
            Ok(())
        }
        .boxed_local()
    }

    /// Returns [`Processed`] that will be resolved once all the underlying data
    /// updates are processed by all subscribers.
    pub fn when_processed(&self) -> Processed<'static> {
        self.state.when_all_processed()
    }

    /// Returns [`Future`] which will be resolved once [`TransitableState`] is
    /// transited to the [`TransitableState::Stable`].
    pub fn when_stabilized(self: Rc<Self>) -> Processed<'static, ()> {
        Processed::new(Box::new(move || {
            let stable = self.subscribe_stable();
            Box::pin(async move {
                stable.fuse().select_next_some().map(drop).await;
            })
        }))
    }

    /// Updates [`TransitableStateController::state`].
    pub(in super::super) fn update(&self, new_state: S) {
        let current_state = self.state.get();

        let state_update = match current_state {
            TransitableState::Stable(_) => new_state.into(),
            TransitableState::Transition(t) => {
                if t.intended() == new_state {
                    new_state.into()
                } else {
                    t.set_inner(new_state).into()
                }
            }
        };

        self.state.set(state_update);
    }
}

impl MuteStateController {
    /// Indicates whether [`TransitableStateController`]'s mute state is in
    /// [`mute_state::Stable::Muted`].
    #[must_use]
    pub fn muted(&self) -> bool {
        self.state.get() == mute_state::Stable::Muted.into()
    }

    /// Indicates whether [`TransitableStateController`]'s mute state is in
    /// [`mute_state::Stable::Unmuted`].
    #[must_use]
    pub fn unmuted(&self) -> bool {
        self.state.get() == mute_state::Stable::Unmuted.into()
    }
}

impl MediaExchangeStateController {
    /// Indicates whether [`TransitableStateController`]'s media exchange state
    /// is in [`media_exchange_state::Stable::Disabled`].
    #[must_use]
    pub fn disabled(&self) -> bool {
        self.state.get() == media_exchange_state::Stable::Disabled.into()
    }

    /// Indicates whether [`TransitableStateController`]'s media exchange state
    /// is in [`media_exchange_state::Stable::Enabled`].
    #[must_use]
    pub fn enabled(&self) -> bool {
        self.state.get() == media_exchange_state::Stable::Enabled.into()
    }
}
