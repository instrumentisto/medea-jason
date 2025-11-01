//! Implementation of Control API gRPC [Callback service].
//!
//! [Callback service]: https://tinyurl.com/y5fajesq

use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};

use actix::{Actor, Addr, Arbiter, Context, Handler, Message};
use medea_control_api_proto::grpc::callback::{
    self as proto,
    callback_server::{
        Callback as CallbackService, CallbackServer as TonicCallbackServer,
    },
};
use tonic::transport::Server;
use tracing as log;

use crate::{Cli, callback::CallbackItem};

/// Type which used in [`GrpcCallbackServer`] for [`CallbackItem`] storing.
type CallbackItems = Arc<Mutex<Vec<CallbackItem>>>;

/// [`Actor`] wrapper for [`tonic`] gRPC server.
///
/// Also this [`Actor`] can return all received callbacks
/// with [`GetCallbackItems`] [`Message`].
#[derive(Debug)]
pub struct GrpcCallbackServer {
    /// All [`CallbackItem`]s which this server received.
    events: CallbackItems,
}

impl Actor for GrpcCallbackServer {
    type Context = Context<Self>;
}

/// Implementation for [`CallbackService`] gRPC service.
#[derive(Clone, Debug)]
pub struct GrpcCallbackService {
    /// All [`CallbackItem`]s which this server received.
    events: CallbackItems,
}

impl GrpcCallbackService {
    /// Returns [`GrpcCallbackService`] with provided pointer to [`Vec`] of
    /// [`CallbackItem`]s.
    #[must_use]
    pub const fn new(events: CallbackItems) -> Self {
        Self { events }
    }
}

#[tonic::async_trait]
impl CallbackService for GrpcCallbackService {
    async fn on_event(
        &self,
        request: tonic::Request<proto::Request>,
    ) -> Result<tonic::Response<proto::Response>, tonic::Status> {
        log::info!("Callback request received: [{request:?}]");
        self.events.lock().unwrap().push(request.into_inner().into());
        Ok(tonic::Response::new(proto::Response {}))
    }
}

/// [`Message`] which returns all [`CallbackItem`]s received by this
/// [`GrpcCallbackServer`].
#[derive(Clone, Copy, Debug, Message)]
#[rtype(result = "Result<Vec<CallbackItem>, Infallible>")]
pub struct GetCallbackItems;

impl Handler<GetCallbackItems> for GrpcCallbackServer {
    type Result = Result<Vec<CallbackItem>, Infallible>;

    fn handle(
        &mut self,
        _: GetCallbackItems,
        _: &mut Self::Context,
    ) -> Self::Result {
        Ok(self.events.lock().unwrap().clone())
    }
}

/// Run [`GrpcCallbackServer`].
///
/// # Panics
///
/// If cannot bind and run gRPC server.
#[must_use]
pub fn run(opts: &Cli) -> Addr<GrpcCallbackServer> {
    let events = Arc::new(Mutex::new(Vec::new()));

    let service =
        TonicCallbackServer::new(GrpcCallbackService::new(Arc::clone(&events)));
    let addr = format!("{}:{}", opts.callback_host, opts.callback_port)
        .parse()
        .unwrap();

    _ = Arbiter::current().spawn(async move {
        Server::builder().add_service(service).serve(addr).await.unwrap();
    });

    log::debug!("gRPC callback server started.");

    GrpcCallbackServer::start_in_arbiter(&Arbiter::new().handle(), move |_| {
        GrpcCallbackServer { events }
    })
}
