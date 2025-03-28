//! Progressable analogue of a [`Cell`].
//!
//! [`Cell`]: std::cell::Cell

use std::{
    cell::{Ref, RefCell},
    mem,
};

use futures::stream::LocalBoxStream;

use crate::{
    Guarded, MutObservableFieldGuard, Progressable,
    subscribers_store::progressable::{self, Processed},
};

/// Reactive [`Cell`] with a progress tracking.
///
/// Subscription to changes works the same way as in [`Progressable`], but
/// working with an underlying data of [`ProgressableCell`] is different in a
/// way allowing mutating and replacing it.
///
/// [`Cell`]: std::cell::Cell
#[derive(Debug)]
pub struct ProgressableCell<D>(RefCell<Progressable<D>>);

impl<D> ProgressableCell<D>
where
    D: 'static,
{
    /// Returns new [`ProgressableCell`].
    #[must_use]
    pub fn new(data: D) -> Self {
        Self(RefCell::new(Progressable::new(data)))
    }

    /// Returns immutable reference to underlying data.
    #[must_use]
    pub fn borrow(&self) -> Ref<'_, D> {
        let reference = self.0.borrow();
        Ref::map(reference, |observable| &**observable)
    }
}

impl<D> ProgressableCell<D>
where
    D: Clone + 'static,
{
    /// Returns copy of an underlying data.
    #[must_use]
    pub fn get(&self) -> D {
        self.0.borrow().data.clone()
    }

    /// Returns [`Stream`] into which the underlying data updates will be
    /// emitted.
    ///
    /// [`Stream`]: futures::Stream
    pub fn subscribe(&self) -> LocalBoxStream<'static, Guarded<D>> {
        self.0.borrow().subscribe()
    }

    /// Returns [`Future`] that will be resolved when all the underlying data
    /// updates will be processed by all subscribers.
    pub fn when_all_processed(&self) -> Processed<'static> {
        self.0.borrow().when_all_processed()
    }
}

impl<D> ProgressableCell<D>
where
    D: Clone + PartialEq + 'static,
{
    /// Replaces the wrapped value with a `new_data` one.
    pub fn set(&self, new_data: D) {
        drop(self.replace(new_data));
    }

    /// Replaces the wrapped value with a `new_data` one, returning the old
    /// value.
    #[must_use]
    pub fn replace(&self, mut new_data: D) -> D {
        mem::swap(&mut *self.0.borrow_mut().borrow_mut(), &mut new_data);
        new_data
    }

    /// Updates the underlying data using the provided function accepting a
    /// mutable reference to the underlying data.
    pub fn mutate<F>(&self, f: F)
    where
        F: FnOnce(MutObservableFieldGuard<'_, D, progressable::SubStore<D>>),
    {
        (f)(self.0.borrow_mut().borrow_mut());
    }
}
