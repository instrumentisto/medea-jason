//! Handy listener implementation for browser events dispatched by their names.

use std::{ops::Deref, rc::Rc};

use derive_more::{Display, From};
use tracerr::Traced;
use wasm_bindgen::{closure::Closure, convert::FromWasmAbi, JsCast};

use crate::{platform, utils::Caused};

/// Failed to bind to [`EventTarget`][1] event.
///
/// [1]: https://developer.mozilla.org/en-US/docs/Web/API/EventTarget
#[derive(Caused, Clone, Debug, Display, From, Eq, PartialEq)]
#[cause(error = "platform::Error")]
#[display(fmt = "EventTarget.addEventListener() failed: {}", _0)]
pub struct EventListenerBindError(platform::Error);

/// Wrapper for closure that handles some [`EventTarget`] event.
///
/// [`EventTarget`]: web_sys::EventTarget
#[derive(Debug)]
pub struct EventListener<T, A>
where
    T: Deref<Target = web_sys::EventTarget>,
{
    /// Name of the browser event this [`EventListener`] listens to.
    event_name: &'static str,

    /// [`EventTarget`] of to listen browser events on.
    ///
    /// [`EventTarget`]: web_sys::EventTarget
    target: Rc<T>, // TODO: Get rid of `Rc`?

    /// Function to be executed when the listened browser event fires.
    closure: Closure<dyn FnMut(A)>,
}

impl<T, A> EventListener<T, A>
where
    T: Deref<Target = web_sys::EventTarget>,
    A: FromWasmAbi + 'static,
{
    /// Creates a new [`EventListener`] from the given [`FnMut`] `closure`.
    ///
    /// # Errors
    ///
    /// Errors if [`EventListener`] bound fails.
    pub fn new_mut<F>(
        target: Rc<T>,
        event_name: &'static str,
        closure: F,
    ) -> Result<Self, Traced<EventListenerBindError>>
    where
        F: FnMut(A) + 'static,
    {
        let closure = Closure::wrap(Box::new(closure) as Box<dyn FnMut(A)>);

        target
            .add_event_listener_with_callback(
                event_name,
                closure.as_ref().unchecked_ref(),
            )
            .map_err(platform::Error::from)
            .map_err(EventListenerBindError::from)
            .map_err(tracerr::wrap!())?;

        Ok(Self {
            event_name,
            target,
            closure,
        })
    }

    /// Creates new [`EventListener`] from a given [`FnOnce`] `closure`.
    ///
    /// # Errors
    ///
    /// Errors if [`EventListener`] bound fails.
    pub fn new_once<F>(
        target: Rc<T>,
        event_name: &'static str,
        closure: F,
    ) -> Result<Self, Traced<EventListenerBindError>>
    where
        F: FnOnce(A) + 'static,
    {
        let closure: Closure<dyn FnMut(A)> = Closure::once(closure);

        target
            .add_event_listener_with_callback(
                event_name,
                closure.as_ref().unchecked_ref(),
            )
            .map_err(platform::Error::from)
            .map_err(EventListenerBindError::from)
            .map_err(tracerr::wrap!())?;

        Ok(Self {
            event_name,
            target,
            closure,
        })
    }
}

impl<T, A> Drop for EventListener<T, A>
where
    T: Deref<Target = web_sys::EventTarget>,
{
    /// Drops [`EventListener`]'s closure and unregisters appropriate event
    /// handler.
    fn drop(&mut self) {
        if let Err(err) = (self.target.as_ref() as &web_sys::EventTarget)
            .remove_event_listener_with_callback(
                self.event_name,
                self.closure.as_ref().unchecked_ref(),
            )
        {
            log::error!("Failed to remove EventListener: {err:?}");
        }
    }
}
