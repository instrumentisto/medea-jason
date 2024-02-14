//! Functionality for calling platform callbacks.

use std::cell::RefCell;

use super::Function;

/// Wrapper for a single argument callback function.
#[derive(Debug)]
pub struct Callback<A>(pub RefCell<Option<Function<A>>>);

impl<A> Callback<A> {
    /// Sets the inner [`Function`].
    pub fn set_func(&self, f: Function<A>) {
        drop(self.0.borrow_mut().replace(f));
    }

    /// Indicates whether this [`Callback`] is set.
    #[must_use]
    pub fn is_set(&self) -> bool {
        self.0.borrow().as_ref().is_some()
    }
}

impl Callback<()> {
    /// Invokes the underlying [`Function`] (if any) passing no arguments to it.
    pub fn call0(&self) {
        if let Some(f) = self.0.borrow().as_ref() {
            f.call0();
        };
    }
}

// Implemented manually to omit redundant `A: Default` trait bound, imposed by
// `#[derive(Default)]`.
impl<A> Default for Callback<A> {
    fn default() -> Self {
        Self(RefCell::new(None))
    }
}
