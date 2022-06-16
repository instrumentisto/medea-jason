#[cfg(feature = "client")]
mod client;
#[cfg(feature = "server")]
mod server;

use futures::channel::{mpsc, oneshot};

use crate::{control, member, ControlApi, Elements, Fid, Ping, Pong};

#[cfg(feature = "client")]
#[doc(inline)]
pub use self::client::{ControlApiClient, ControlApiClientError};
#[cfg(feature = "server")]
#[doc(inline)]
pub use self::server::ControlApiServer;

#[cfg(all(feature = "client", feature = "server"))]
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

enum ControlApiRequest<Error> {
    Create {
        request: control::Request,
        response: oneshot::Sender<Result<member::Sids, Error>>,
    },
    Apply {
        request: control::Request,
        response: oneshot::Sender<Result<member::Sids, Error>>,
    },
    Delete {
        request: Vec<Fid>,
        response: oneshot::Sender<Result<(), Error>>,
    },
    Get {
        request: Vec<Fid>,
        response: oneshot::Sender<Result<Elements, Error>>,
    },
    Healthz {
        request: Ping,
        response: oneshot::Sender<Result<Pong, Error>>,
    },
}
