//! [`ControlApi`] client and [`ControlApi`] server [`channel`]
//! implementations.
//!
//! [`channel`]: futures::channel

use async_trait::async_trait;
use derive_more::{Display, Error, From};
use futures::{
    channel::{mpsc, oneshot},
    StreamExt as _,
};

use crate::{callback, CallbackApi, ControlApi};

use super::{CallbackApiRequest, ControlApiRequest, SendErr};

/// [`channel`]-based [`ControlApi`] server.
///
/// [`channel`]: futures::channel
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct ControlApiServer<T: ControlApi> {
    /// Inner [`ControlApi`] implementation.
    pub(crate) api: T,

    /// [`mpsc::UnboundedReceiver`] to receive [`ControlApiRequest`].
    pub(crate) receiver: mpsc::UnboundedReceiver<ControlApiRequest<T::Error>>,
}

impl<T: ControlApi> ControlApiServer<T> {
    /// Runs this [`ControlApiServer`]. Completes after all
    /// [`ControlApiClient`]s linked to this [`ControlApiServer`] are dropped.
    ///
    /// # Errors
    ///
    /// In case failed to send response via [`oneshot::Sender`].
    ///
    /// [`ControlApiClient`]: super::ControlApiClient
    #[allow(clippy::map_err_ignore)]
    pub async fn run(mut self) -> Result<(), SendErr> {
        while let Some(request) = self.receiver.next().await {
            match request {
                ControlApiRequest::Create { request, response } => {
                    response
                        .send(self.api.create(request).await)
                        .map_err(|_| SendErr)?;
                }
                ControlApiRequest::Apply { request, response } => {
                    response
                        .send(self.api.apply(request).await)
                        .map_err(|_| SendErr)?;
                }
                ControlApiRequest::Delete { request, response } => {
                    response
                        .send(self.api.delete(&request).await)
                        .map_err(|_| SendErr)?;
                }
                ControlApiRequest::Get { request, response } => {
                    response
                        .send(self.api.get(&request).await)
                        .map_err(|_| SendErr)?;
                }
                ControlApiRequest::Healthz { request, response } => {
                    response
                        .send(self.api.healthz(request).await)
                        .map_err(|_| SendErr)?;
                }
            }
        }
        Ok(())
    }
}

/// [`channel`]-based [`CallbackApi`] client.
///
/// [`channel`]: futures::channel
#[derive(Clone, Debug)]
pub struct CallbackApiClient<Error> {
    /// [`mpsc::UnboundedSender`] to send [`CallbackApiRequest`]s to linked
    /// [`CallbackApiServer`].
    ///
    /// [`CallbackApiServer`]: super::CallbackApiServer
    pub(crate) sender: mpsc::UnboundedSender<CallbackApiRequest<Error>>,
}

#[async_trait]
impl<Error> CallbackApi for CallbackApiClient<Error>
where
    Error: Send,
{
    type Error = CallbackApiClientError<Error>;

    async fn on_event(
        &self,
        request: callback::Request,
    ) -> Result<(), Self::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .clone()
            .unbounded_send(CallbackApiRequest {
                request,
                response: sender,
            })
            .map_err(mpsc::TrySendError::into_send_error)?;
        match receiver.await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(CallbackApiClientError::ControlApiServer(e)),
            Err(e) => Err(e.into()),
        }
    }
}

/// [`CallbackApiClient`] error.
#[derive(Clone, Debug, Display, Error, From)]
pub enum CallbackApiClientError<E> {
    /// [`CallbackApiServer`] error.
    ///
    /// [`CallbackApiServer`]: super::CallbackApiServer
    #[from(ignore)]
    ControlApiServer(E),

    /// Failed to send request to [`CallbackApiServer`].
    ///
    /// [`CallbackApiServer`]: super::CallbackApiServer
    Send(mpsc::SendError),

    /// Failed to receive response from [`CallbackApiServer`].
    ///
    /// [`CallbackApiServer`]: super::CallbackApiServer
    Cancelled(oneshot::Canceled),
}
