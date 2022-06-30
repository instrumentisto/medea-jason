//! Direct in-process [Control API] implementation.
//!
//! [Control API]: https://tinyurl.com/yxsqplq7

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "server")]
mod server;

use derive_more::{Display, Error};
use futures::channel::{mpsc, oneshot};

use crate::{
    callback, control, member, CallbackApi, ControlApi, Elements, Fid, Ping,
    Pong,
};

#[cfg(feature = "client")]
#[doc(inline)]
pub use self::client::{
    CallbackApiServer, ControlApiClient, ControlApiClientError,
};
#[cfg(feature = "server")]
#[doc(inline)]
pub use self::server::{
    CallbackApiClient, CallbackApiClientError, ControlApiServer,
};

#[cfg(all(feature = "client", feature = "server"))]
/// Creates pair of a [`ControlApiClient`] and a [`ControlApiServer`].
pub fn control_api<T: ControlApi>() -> (
    ControlApiClient<T::Error>,
    impl FnOnce(T) -> ControlApiServer<T>,
) {
    let (sender, receiver) = mpsc::unbounded();
    (ControlApiClient { sender }, move |api| ControlApiServer {
        api,
        receiver,
    })
}

#[cfg(all(feature = "client", feature = "server"))]
/// Creates a pair of a [`CallbackApiClient`] and a [`CallbackApiServer`].
pub fn callback_api<T: CallbackApi>() -> (
    CallbackApiClient<T::Error>,
    impl FnOnce(T) -> CallbackApiServer<T>,
) {
    let (sender, receiver) = mpsc::unbounded();
    (CallbackApiClient { sender }, move |api| CallbackApiServer {
        api,
        receiver,
    })
}

/// Error of sending response via [`oneshot::Sender`].
#[derive(Clone, Copy, Debug, Display, Error)]
#[display(fmt = "`oneshot::Sender` errored")]
pub struct SendErr;

/// [`ControlApi`] request paired with an [`oneshot::Sender`] to send response
/// via.
#[derive(Debug)]
pub(crate) enum ControlApiRequest<Error> {
    /// [`ControlApi::create()`].
    Create {
        /// [`ControlApi::create()`] request.
        request: control::Request,

        /// [`oneshot::Sender`] to send [`ControlApi::create()`] response via.
        sender: oneshot::Sender<Result<member::Sids, Error>>,
    },

    /// [`ControlApi::apply()`].
    Apply {
        /// [`ControlApi::apply()`] request.
        request: control::Request,

        /// [`oneshot::Sender`] to send [`ControlApi::apply()`] response via.
        sender: oneshot::Sender<Result<member::Sids, Error>>,
    },

    /// [`ControlApi::delete()`].
    Delete {
        /// [`ControlApi::delete()`] request.
        request: Vec<Fid>,

        /// [`oneshot::Sender`] to send [`ControlApi::delete()`] response via.
        sender: oneshot::Sender<Result<(), Error>>,
    },

    /// [`ControlApi::get()`].
    Get {
        /// [`ControlApi::get()`] request.
        request: Vec<Fid>,

        /// [`oneshot::Sender`] to send [`ControlApi::get()`] response via.
        sender: oneshot::Sender<Result<Elements, Error>>,
    },

    /// [`ControlApi::healthz()`].
    Healthz {
        /// [`ControlApi::healthz()`] request.
        request: Ping,

        /// [`oneshot::Sender`] to send [`ControlApi::healthz()`] response via.
        sender: oneshot::Sender<Result<Pong, Error>>,
    },
}

/// [`CallbackApi`] request paired with an [`oneshot::Sender`] to send response
/// via.
#[derive(Debug)]
pub(crate) struct CallbackApiRequest<Error> {
    /// [`CallbackApi::on_event()`] request.
    request: callback::Request,

    /// [`oneshot::Sender`] to send [`CallbackApi::on_event()`] response via.
    sender: oneshot::Sender<Result<(), Error>>,
}
