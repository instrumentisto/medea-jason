//! [`ControlApi`] client and [`CallbackApi`] server direct in-process
//! implementations.
//!
//! [`channel`]: futures::channel

use async_trait::async_trait;
use derive_more::{Display, Error, From};
use futures::{
    channel::{mpsc, oneshot},
    StreamExt as _,
};

use crate::{
    control, member, CallbackApi, ControlApi, Elements, Fid, Ping, Pong,
};

use super::{CallbackApiRequest, ControlApiRequest, SendErr};

/// Direct in-process [`CallbackApi`] server.
#[derive(Debug)]
pub struct CallbackApiServer<T: CallbackApi> {
    /// Inner [`CallbackApi`] implementation.
    pub(crate) api: T,

    /// [`mpsc::UnboundedReceiver`] to receive [`CallbackApiRequest`] via.
    pub(crate) receiver: mpsc::UnboundedReceiver<CallbackApiRequest<T::Error>>,
}

impl<T: CallbackApi> CallbackApiServer<T> {
    /// Runs this [`CallbackApiServer`].
    ///
    /// Completes after all the [`CallbackApiClient`]s linked to this
    /// [`CallbackApiServer`] are dropped.
    ///
    /// # Errors
    ///
    /// If failed to send response via [`oneshot::Sender`].
    ///
    /// [`CallbackApiClient`]: super::CallbackApiClient
    #[allow(clippy::map_err_ignore)]
    pub async fn run(mut self) -> Result<(), SendErr> {
        while let Some(ev) = self.receiver.next().await {
            ev.sender
                .send(self.api.on_event(ev.request).await)
                .map_err(|_| SendErr)?;
        }
        Ok(())
    }
}

/// Direct in-process [`ControlApi`] client.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct ControlApiClient<Error> {
    /// [`mpsc::UnboundedSender`] to send [`ControlApiRequest`]s to linked
    /// [`ControlApiServer`].
    ///
    /// [`ControlApiServer`]: super::ControlApiServer
    pub(crate) sender: mpsc::UnboundedSender<ControlApiRequest<Error>>,
}

// Implemented manually to omit redundant `Error: Clone` trait bound, imposed by
// `#[derive(Clone)]`.
impl<Error> Clone for ControlApiClient<Error> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

#[async_trait]
impl<Error> ControlApi for ControlApiClient<Error>
where
    Error: Send,
{
    type Error = ControlApiClientError<Error>;

    async fn create(
        &self,
        request: control::Request,
    ) -> Result<member::Sids, Self::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .clone()
            .unbounded_send(ControlApiRequest::Create { request, sender })
            .map_err(mpsc::TrySendError::into_send_error)?;
        match receiver.await {
            Ok(Ok(ok)) => Ok(ok),
            Ok(Err(e)) => Err(ControlApiClientError::ControlApiServer(e)),
            Err(e) => Err(e.into()),
        }
    }

    async fn apply(
        &self,
        request: control::Request,
    ) -> Result<member::Sids, Self::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .clone()
            .unbounded_send(ControlApiRequest::Apply { request, sender })
            .map_err(mpsc::TrySendError::into_send_error)?;
        match receiver.await {
            Ok(Ok(ok)) => Ok(ok),
            Ok(Err(e)) => Err(ControlApiClientError::ControlApiServer(e)),
            Err(e) => Err(e.into()),
        }
    }

    async fn delete(&self, fids: &[Fid]) -> Result<(), Self::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .clone()
            .unbounded_send(ControlApiRequest::Delete {
                request: fids.to_vec(),
                sender,
            })
            .map_err(mpsc::TrySendError::into_send_error)?;
        match receiver.await {
            Ok(Ok(ok)) => Ok(ok),
            Ok(Err(e)) => Err(ControlApiClientError::ControlApiServer(e)),
            Err(e) => Err(e.into()),
        }
    }

    async fn get(&self, fids: &[Fid]) -> Result<Elements, Self::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .clone()
            .unbounded_send(ControlApiRequest::Get {
                request: fids.to_vec(),
                sender,
            })
            .map_err(mpsc::TrySendError::into_send_error)?;
        match receiver.await {
            Ok(Ok(ok)) => Ok(ok),
            Ok(Err(e)) => Err(ControlApiClientError::ControlApiServer(e)),
            Err(e) => Err(e.into()),
        }
    }

    async fn healthz(&self, request: Ping) -> Result<Pong, Self::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .clone()
            .unbounded_send(ControlApiRequest::Healthz { request, sender })
            .map_err(mpsc::TrySendError::into_send_error)?;
        match receiver.await {
            Ok(Ok(ok)) => Ok(ok),
            Ok(Err(e)) => Err(ControlApiClientError::ControlApiServer(e)),
            Err(e) => Err(e.into()),
        }
    }
}

/// [`ControlApiClient`] error.
#[derive(Clone, Debug, Display, Error, From)]
pub enum ControlApiClientError<E> {
    /// [`ControlApiServer`] error.
    ///
    /// [`ControlApiServer`]: super::ControlApiServer
    #[from(ignore)]
    ControlApiServer(E),

    /// Failed to send request to [`ControlApiServer`].
    ///
    /// [`ControlApiServer`]: super::ControlApiServer
    Send(mpsc::SendError),

    /// Failed to receive response from [`ControlApiServer`].
    ///
    /// [`ControlApiServer`]: super::ControlApiServer
    Cancelled(oneshot::Canceled),
}
