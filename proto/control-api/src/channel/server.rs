use futures::{
    channel::{mpsc, oneshot},
    StreamExt as _,
};

use crate::ControlApi;

use super::ControlApiRequest;

#[derive(Debug)]
pub struct ControlApiServer<T: ControlApi> {
    pub(crate) api: T,
    pub(crate) receiver: mpsc::UnboundedReceiver<(ControlApiRequest<T::Error>)>,
}

impl<T: ControlApi> ControlApiServer<T> {
    async fn run(mut self) {
        while let Some(request) = self.receiver.next().await {
            match request {
                ControlApiRequest::Create { request, response } => {
                    response.send(self.api.create(request).await)
                }
                ControlApiRequest::Apply { request, response } => {
                    response.send(self.api.apply(request).await)
                }
                ControlApiRequest::Delete { request, response } => {
                    response.send(self.api.delete(&request).await)
                }
                ControlApiRequest::Get { request, response } => {
                    response.send(self.api.get(&request).await)
                }
                ControlApiRequest::Healthz { request, response } => {
                    response.send(self.api.healthz(request).await)
                }
            }
        }
    }
}
