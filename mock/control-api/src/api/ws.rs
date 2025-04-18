//! [WebSocket] [Control API] mock server implementation.
//!
//! [Control API]: https://tinyurl.com/yxsqplq7
//! [WebSocket]: https://en.wikipedia.org/wiki/WebSocket

use std::{sync::Arc, time::Duration};

use actix::{
    Actor, ActorContext as _, AsyncContext as _, Handler, Message,
    StreamHandler,
};
use actix_web::{
    Error, HttpRequest, HttpResponse,
    web::{Data, Path, Payload},
};
use actix_web_actors::ws;
use serde::Serialize;
use serde_json::Value;

use crate::{
    api::{AppContext, Element, Subscribers},
    client::Fid,
    prelude::*,
};

/// Handles HTTP upgrade request trying to perform handshake and establish
/// [WebSocket] connection.
///
/// # Errors
///
/// Errors if handshake fails for any underlying reason.
///
/// [WebSocket]: https://en.wikipedia.org/wiki/WebSocket
pub async fn create_ws(
    request: HttpRequest,
    path: Path<String>,
    state: Data<AppContext>,
    payload: Payload,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsSession {
            room_id: path.into_inner(),
            subscribers: Arc::clone(&state.subscribers),
            last_ping_num: 0,
        },
        &request,
        payload,
    )
}

/// Notification about a some mutating operation being performed to some `Room`.
#[derive(Clone, Debug, Message, Serialize)]
#[rtype(result = "()")]
pub struct Notification(Value);

/// [`Notification`] serialization helper.
#[derive(Serialize)]
#[serde(tag = "method")]
enum NotificationVariants<'a> {
    /// [`Notification`] broadcast to all participants.
    Broadcast {
        /// Payload to be broadcast to all participants.
        payload: Value,
    },

    /// [`Notification`] about an [`Element`] being created.
    Created {
        /// FID of the created [`Element`].
        fid: &'a str,

        /// Created [`Element`].
        element: &'a Element,
    },

    /// [`Notification`] about an [`Element`] being deleted.
    Deleted {
        /// FID of the deleted [`Element`].
        fid: &'a str,
    },
}

impl Notification {
    /// Builds `method: Created` [`Notification`].
    #[expect(clippy::missing_panics_doc, reason = "serialization")]
    #[must_use]
    pub fn created(fid: &Fid, element: &Element) -> Self {
        Self(
            serde_json::to_value(NotificationVariants::Created {
                fid: fid.as_ref(),
                element,
            })
            .unwrap(),
        )
    }

    /// Builds `method: Deleted` [`Notification`].
    #[expect(clippy::missing_panics_doc, reason = "serialization")]
    #[must_use]
    pub fn deleted(fid: &Fid) -> Self {
        Self(
            serde_json::to_value(NotificationVariants::Deleted {
                fid: fid.as_ref(),
            })
            .unwrap(),
        )
    }

    /// Builds `method: Broadcast` [`Notification`].
    #[expect(clippy::missing_panics_doc, reason = "serialization")]
    #[must_use]
    pub fn broadcast(payload: Value) -> Self {
        Self(
            serde_json::to_value(NotificationVariants::Broadcast { payload })
                .unwrap(),
        )
    }
}

/// [WebSocket] connection with a [`Notification`] subscriber.
///
/// [WebSocket]: https://en.wikipedia.org/wiki/WebSocket
#[derive(Default)]
struct WsSession {
    /// `Room` id that this [`WsSession`] is subscribed to.
    room_id: String,

    /// Map of subscribers to [`Notification`]s.
    subscribers: Subscribers,

    /// `Ping` messages counter.
    last_ping_num: u32,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    /// Adds [`WsSession`] to [`WsSession`]s map and schedules `Ping` task.
    fn started(&mut self, ctx: &mut Self::Context) {
        let recp = ctx.address().recipient();

        self.subscribers
            .lock()
            .unwrap()
            .entry(self.room_id.clone())
            .or_default()
            .push(recp);

        _ = ctx.run_interval(Duration::from_secs(10), |this: &mut Self, cx| {
            this.last_ping_num += 1;
            cx.ping(&this.last_ping_num.to_be_bytes());
        });
    }

    /// Removes [`WsSession`] from [`WsSession`]s map.
    fn stopped(&mut self, ctx: &mut Self::Context) {
        let this = ctx.address().recipient();
        if let Some(subs) =
            self.subscribers.lock().unwrap().get_mut(&self.room_id)
        {
            subs.retain(|sub| *sub != this);
        }
    }
}

impl Handler<Notification> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: Notification, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(
        &mut self,
        item: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match item {
            Ok(msg) => match msg {
                ws::Message::Ping(ping) => {
                    ctx.pong(&ping);
                }
                ws::Message::Close(reason) => {
                    ctx.close(reason);
                    ctx.stop();
                }
                ws::Message::Pong(_) => {}
                ws::Message::Text(text) => {
                    match serde_json::from_str::<Value>(&text) {
                        Ok(json) => {
                            let this = ctx.address().recipient();
                            if let Some(subs) = self
                                .subscribers
                                .lock()
                                .unwrap()
                                .get(&self.room_id)
                            {
                                subs.iter()
                                    .filter(|sub| **sub != this)
                                    .for_each(|sub| {
                                        sub.do_send(Notification::broadcast(
                                            json.clone(),
                                        ));
                                    });
                            }
                        }
                        Err(err) => error!(
                            "Received broadcast message but it is not a valid \
                             JSON: {err:?}",
                        ),
                    }
                }
                ws::Message::Binary(_)
                | ws::Message::Continuation(_)
                | ws::Message::Nop => {
                    error!("Unsupported client message: {msg:?}");
                }
            },
            Err(e) => {
                error!("WS StreamHandler error: {e}");
            }
        }
    }
}
