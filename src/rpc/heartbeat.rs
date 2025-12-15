//! Connection loss detection via ping/pong mechanism.

use std::{cell::RefCell, rc::Rc, time::Duration};

use derive_more::with_trait::{Debug, Mul};
use futures::{StreamExt as _, channel::mpsc, future, stream::LocalBoxStream};
use medea_client_api_proto::{ClientMsg, ServerMsg};

use crate::{platform, utils::TaskHandle};

/// Idle timeout of [`WebSocketRpcClient`].
///
/// [`WebSocketRpcClient`]: super::WebSocketRpcClient
#[derive(Clone, Copy, Debug)]
pub struct IdleTimeout(pub Duration);

/// Ping interval of [`WebSocketRpcClient`].
///
/// [`WebSocketRpcClient`]: super::WebSocketRpcClient
#[derive(Clone, Copy, Debug, Mul)]
pub struct PingInterval(pub Duration);

/// Inner data of [`Heartbeat`].
#[derive(Debug)]
struct Inner {
    /// [`platform::RpcTransport`] which heartbeats.
    #[debug("{transport:p}")]
    transport: Rc<dyn platform::RpcTransport>,

    /// Idle timeout of the [`platform::RpcTransport`].
    idle_timeout: IdleTimeout,

    /// Ping interval of the [`platform::RpcTransport`].
    ping_interval: PingInterval,

    /// [`TaskHandle`] for [`Future`] which sends [`ClientMsg::Pong`] on
    /// [`ServerMsg::Ping`].
    handle_ping_task: Option<TaskHandle>,

    /// [`TaskHandle`] for idle watchdog.
    idle_watchdog_task: Option<TaskHandle>,

    /// Number of last received [`ServerMsg::Ping`].
    last_ping_num: u32,

    /// [`mpsc::UnboundedSender`]s for a [`Heartbeat::on_idle`].
    on_idle_subs: Vec<mpsc::UnboundedSender<()>>,
}

impl Inner {
    /// Sends [`ClientMsg::Pong`] to a server.
    ///
    /// If some error happen then it will be printed with [`log::error`].
    fn send_pong(&self, n: u32) {
        _ = self
            .transport
            .send(&ClientMsg::Pong(n))
            .map_err(tracerr::wrap!(=> platform::TransportError))
            .map_err(|e| log::error!("Failed to send pong: {e}"));
    }
}

/// Detector of connection loss via ping/pong mechanism.
#[derive(Debug)]
pub struct Heartbeat(Rc<RefCell<Inner>>);

impl Heartbeat {
    /// Starts this [`Heartbeat`] for the provided [`platform::RpcTransport`]
    /// with the provided `idle_timeout` and `ping_interval`.
    #[must_use]
    pub fn start(
        transport: Rc<dyn platform::RpcTransport>,
        ping_interval: PingInterval,
        idle_timeout: IdleTimeout,
    ) -> Self {
        let inner = Rc::new(RefCell::new(Inner {
            idle_timeout,
            ping_interval,
            transport,
            handle_ping_task: None,
            idle_watchdog_task: None,
            on_idle_subs: Vec::new(),
            last_ping_num: 0,
        }));

        let handle_ping_task = spawn_ping_handle_task(Rc::clone(&inner));
        let idle_watchdog_task = spawn_idle_watchdog_task(Rc::clone(&inner));

        inner.borrow_mut().idle_watchdog_task = Some(idle_watchdog_task);
        inner.borrow_mut().handle_ping_task = Some(handle_ping_task);

        Self(inner)
    }

    /// Updates this [`Heartbeat`] settings.
    pub fn update_settings(
        &self,
        idle_timeout: IdleTimeout,
        ping_interval: PingInterval,
    ) {
        self.0.borrow_mut().idle_timeout = idle_timeout;
        self.0.borrow_mut().ping_interval = ping_interval;
    }

    /// Returns [`LocalBoxStream`] to which will sent `()` when [`Heartbeat`]
    /// considers that [`platform::RpcTransport`] is idle.
    #[must_use]
    pub fn on_idle(&self) -> LocalBoxStream<'static, ()> {
        let (on_idle_tx, on_idle_rx) = mpsc::unbounded();
        self.0.borrow_mut().on_idle_subs.push(on_idle_tx);

        Box::pin(on_idle_rx)
    }
}

/// Spawns idle watchdog task returning its handle.
///
/// This task is responsible for throwing [`Heartbeat::on_idle`] when
/// [`ServerMsg`] hasn't been received within `idle_timeout`.
///
/// Also this watchdog will repeat [`ClientMsg::Pong`] if
/// [`ServerMsg::Ping`] wasn't received within `ping_interval * 2`.
fn spawn_idle_watchdog_task(this: Rc<RefCell<Inner>>) -> TaskHandle {
    let (idle_watchdog_fut, idle_watchdog_handle) =
        future::abortable(async move {
            let wait_for_ping = this.borrow().ping_interval * 2;
            platform::delay_for(wait_for_ping.0).await;

            let last_ping_num = this.borrow().last_ping_num;
            this.borrow().send_pong(last_ping_num + 1);

            let idle_timeout = this.borrow().idle_timeout;
            #[expect(clippy::expect_used, reason = "not happens")]
            let delay = idle_timeout
                .0
                .checked_sub(wait_for_ping.0)
                .expect("`wait_for_ping` > `idle_timeout`");
            platform::delay_for(delay).await;
            this.borrow_mut()
                .on_idle_subs
                .retain(|sub| sub.unbounded_send(()).is_ok());
        });

    platform::spawn(async move {
        _ = idle_watchdog_fut.await.ok();
    });

    idle_watchdog_handle.into()
}

/// Spawns ping handle task returning its handle.
///
/// This task is responsible for answering [`ServerMsg::Ping`] with
/// [`ClientMsg::Pong`] and renewing idle watchdog task.
fn spawn_ping_handle_task(this: Rc<RefCell<Inner>>) -> TaskHandle {
    let mut on_message_stream = this.borrow().transport.on_message();

    let (handle_ping_fut, handle_ping_task) = future::abortable(async move {
        while let Some(msg) = on_message_stream.next().await {
            let idle_task = spawn_idle_watchdog_task(Rc::clone(&this));
            this.borrow_mut().idle_watchdog_task = Some(idle_task);

            if let ServerMsg::Ping(num) = msg {
                this.borrow_mut().last_ping_num = num;
                this.borrow().send_pong(num);
            }
        }
    });
    platform::spawn(async move {
        _ = handle_ping_fut.await.ok();
    });
    handle_ping_task.into()
}

impl Drop for Heartbeat {
    fn drop(&mut self) {
        let mut inner = self.0.borrow_mut();
        drop(inner.handle_ping_task.take());
        drop(inner.idle_watchdog_task.take());
    }
}
