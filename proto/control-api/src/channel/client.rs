use async_trait::async_trait;
use derive_more::{Display, Error, From};
use futures::channel::{mpsc, oneshot};

use crate::{control, member, ControlApi, Elements, Fid, Ping, Pong};

use super::ControlApiRequest;

pub struct ControlApiClient<Error> {
    pub(crate) sender: mpsc::UnboundedSender<ControlApiRequest<Error>>,
}

#[async_trait]
impl<Error> ControlApi for ControlApiClient<Error> {
    type Error = Error;

    async fn create(
        &self,
        request: control::Request,
    ) -> Result<member::Sids, Self::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .clone()
            .try_send(ControlApiRequest::Create {
                request,
                response: sender,
            })
            .map_err(mpsc::TrySendError::into_send_error)?;

        match receiver.await {
            Ok(Ok(ok)) => Ok(ok),
            Ok(Err(e)) => ControlApiClientError::ControlApiServer(e),
            Err(e) => Err(e.into()),
        }
    }

    async fn apply(
        &self,
        req: control::Request,
    ) -> Result<member::Sids, Self::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .clone()
            .try_send(ControlApiRequest::Apply {
                request,
                response: sender,
            })
            .map_err(mpsc::TrySendError::into_send_error)?;

        match receiver.await {
            Ok(Ok(ok)) => Ok(ok),
            Ok(Err(e)) => ControlApiClientError::ControlApiServer(e),
            Err(e) => Err(e.into()),
        }
    }

    async fn delete(&self, fids: &[Fid]) -> Result<(), Self::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .clone()
            .try_send(ControlApiRequest::Delete {
                request: fids.to_vec(),
                response: sender,
            })
            .map_err(mpsc::TrySendError::into_send_error)?;

        match receiver.await {
            Ok(Ok(ok)) => Ok(ok),
            Ok(Err(e)) => ControlApiClientError::ControlApiServer(e),
            Err(e) => Err(e.into()),
        }
    }

    async fn get(&self, fids: &[Fid]) -> Result<Elements, Self::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .clone()
            .try_send(ControlApiRequest::Get {
                request: fids.to_vec(),
                response: sender,
            })
            .map_err(mpsc::TrySendError::into_send_error)?;

        match receiver.await {
            Ok(Ok(ok)) => Ok(ok),
            Ok(Err(e)) => ControlApiClientError::ControlApiServer(e),
            Err(e) => Err(e.into()),
        }
    }

    async fn healthz(&self, request: Ping) -> Result<Pong, Self::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .clone()
            .try_send(ControlApiRequest::Healthz {
                request,
                response: sender,
            })
            .map_err(mpsc::TrySendError::into_send_error)?;

        match receiver.await {
            Ok(Ok(ok)) => Ok(ok),
            Ok(Err(e)) => ControlApiClientError::ControlApiServer(e),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Clone, Debug, Display, Error)]
pub enum ControlApiClientError<E> {
    ControlApiServer(E),
    Send(mpsc::SendError),
    Cancelled(oneshot::Canceled),
}
