//! [`ControlApi`] client and [`ControlApi`] server direct in-process
//! implementations.

use async_trait::async_trait;
use derive_more::{Display, Error, From};
use futures::{
    channel::{mpsc, oneshot},
    StreamExt as _,
};

use crate::{callback, CallbackApi, ControlApi};

use super::{CallbackApiRequest, ControlApiRequest};

/// Direct in-process [`ControlApi`] server.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct ControlApiServer<T: ControlApi> {
    /// Inner [`ControlApi`] implementation.
    pub(crate) api: T,

    /// [`mpsc::UnboundedReceiver`] to receive [`ControlApiRequest`].
    pub(crate) receiver: mpsc::UnboundedReceiver<ControlApiRequest<T::Error>>,
}

impl<T: ControlApi> ControlApiServer<T> {
    /// Runs this [`ControlApiServer`].
    ///
    /// Completes after all [`ControlApiClient`]s linked to this
    /// [`ControlApiServer`] are dropped.
    ///
    /// `limit` argument specifies number of concurrently handled requests.
    /// Note, that `limit` of zero is interpreted as no limit at all, and will
    /// have the same result as passing in a `None`.
    ///
    /// [`ControlApiClient`]: super::ControlApiClient
    pub async fn run(self, limit: impl Into<Option<usize>>) {
        self.receiver
            .for_each_concurrent(limit, |req| async {
                let _ = match req {
                    ControlApiRequest::Create { request, sender } => {
                        sender.send(self.api.create(request).await).ok()
                    }
                    ControlApiRequest::Apply { request, sender } => {
                        sender.send(self.api.apply(request).await).ok()
                    }
                    ControlApiRequest::Delete { request, sender } => {
                        sender.send(self.api.delete(&request).await).ok()
                    }
                    ControlApiRequest::Get { request, sender } => {
                        sender.send(self.api.get(&request).await).ok()
                    }
                    ControlApiRequest::Healthz { request, sender } => {
                        sender.send(self.api.healthz(request).await).ok()
                    }
                };
            })
            .await;
    }
}

/// Direct in-process [`CallbackApi`] client.
#[derive(Debug)]
pub struct CallbackApiClient<Error> {
    /// [`mpsc::UnboundedSender`] to send [`CallbackApiRequest`]s to linked
    /// [`CallbackApiServer`].
    ///
    /// [`CallbackApiServer`]: super::CallbackApiServer
    pub(crate) sender: mpsc::UnboundedSender<CallbackApiRequest<Error>>,
}

// Implemented manually to omit redundant `Error: Clone` trait bound, imposed by
// `#[derive(Clone)]`.
impl<Error> Clone for CallbackApiClient<Error> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
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
            .unbounded_send(CallbackApiRequest { request, sender })
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
