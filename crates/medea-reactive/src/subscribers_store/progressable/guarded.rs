//! Wrapper around a data decrementing its underlying counter on [`Drop`].

use std::{ops::Deref, rc::Rc};

use crate::ObservableCell;

/// Wrapper around a data `T` decrementing its underlying counter on [`Drop`].
#[derive(Debug)]
pub struct Guarded<T> {
    /// Guarded value of data `T`.
    value: T,

    /// Guard itself guarding the value.
    guard: Guard,
}

impl<T> Guarded<T> {
    /// Wraps the `value` into a new [`Guarded`] basing on the `counter`.
    pub(super) fn wrap(value: T, counter: Rc<ObservableCell<u32>>) -> Self {
        Self { value, guard: Guard::new(counter) }
    }

    /// Unwraps this [`Guarded`] into its inner value and its [`Guard`].
    #[must_use]
    pub fn into_parts(self) -> (T, Guard) {
        (self.value, self.guard)
    }

    /// Unwraps this [`Guarded`] into its inner value dropping its [`Guard`]
    /// in-place.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> Guarded<Option<T>> {
    /// Transposes an [`Guarded`] [`Option`] into a [`Option`] with a
    /// [`Guarded`] value within.
    #[must_use]
    pub fn transpose(self) -> Option<Guarded<T>> {
        let (value, guard) = self.into_parts();
        value.map(move |v| Guarded { value: v, guard })
    }
}

impl<T> AsRef<T> for Guarded<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> Deref for Guarded<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// Guard backed by a counter incrementing on its creation and decrementing on
/// [`Drop`]ping.
#[derive(Debug)]
pub struct Guard(Rc<ObservableCell<u32>>);

impl Guard {
    /// Creates new [`Guard`] on the given `counter`.
    fn new(counter: Rc<ObservableCell<u32>>) -> Self {
        #[expect(clippy::expect_used, reason = "overflowing is unexpected")]
        counter.mutate(|mut c| {
            *c = c
                .checked_add(1)
                .expect("`progressable::Guard` counter overflow");
        });
        Self(counter)
    }
}

impl Drop for Guard {
    /// Decrements the counter backing this [`Guard`].
    fn drop(&mut self) {
        self.0.mutate(|mut c| {
            *c = c.saturating_sub(1);
        });
    }
}
