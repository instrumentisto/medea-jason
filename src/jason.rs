//! General library interface.

use futures::FutureExt as _;
use std::{cell::RefCell, rc::Rc};

use crate::platform;

use crate::{
    media::{MediaManager, MediaManagerHandle},
    room::{Room, RoomHandle},
    rpc::{
        ClientDisconnect, RpcSession, WebSocketRpcClient, WebSocketRpcSession,
    },
};

/// General library interface.
///
/// Responsible for managing shared transports, local media and room
/// initialization.
#[derive(Debug)]
pub struct Jason(Rc<RefCell<Inner>>);

/// Inner representation if a [`Jason`].
#[derive(Debug)]
struct Inner {
    /// [`Jason`]s [`MediaManager`].
    ///
    /// It's shared across [`Room`]s since [`MediaManager`] contains media
    /// tracks that can be used by multiple [`Room`]s.
    media_manager: Rc<MediaManager>,

    /// [`Room`]s maintained by this [`Jason`] instance.
    rooms: Vec<Room>,

    /// Connection with a media server.
    ///
    /// Only one [`WebSocketRpcClient`] is supported at the moment.
    rpc: Rc<WebSocketRpcClient>,
}

impl Jason {
    /// Instantiates a new [`Jason`] interface to interact with this library.
    #[must_use]
    pub fn new() -> Self {
        platform::set_panic_hook();
        if !log::logger().enabled(&log::Metadata::builder().build()) {
            platform::init_logger();
        }

        Self::with_rpc_client(Rc::new(WebSocketRpcClient::new(Box::new(
            || Rc::new(platform::WebSocketRpcTransport::new()),
        ))))
    }

    /// Creates a new [`Room`] and returns its [`RoomHandle`].
    #[must_use]
    pub fn init_room(&self) -> RoomHandle {
        let rpc = Rc::clone(&self.0.borrow().rpc);
        self.inner_init_room(WebSocketRpcSession::new(rpc))
    }

    /// Returns a [`MediaManagerHandle`].
    #[must_use]
    pub fn media_manager(&self) -> MediaManagerHandle {
        self.0.borrow().media_manager.new_handle()
    }

    /// Closes the provided [`RoomHandle`].
    #[allow(clippy::needless_pass_by_value)]
    pub fn close_room(&self, room_to_delete: RoomHandle) {
        let index = self
            .0
            .borrow()
            .rooms
            .iter()
            .enumerate()
            .find(|(_, room)| room.inner_ptr_eq(&room_to_delete))
            .map(|(i, _)| i);

        if let Some(i) = index {
            let this = &mut self.0.borrow_mut();
            let room = this.rooms.swap_remove(i);
            room.set_close_reason(ClientDisconnect::RoomClosed.into());
            drop(room);
            if this.rooms.is_empty() {
                this.rpc = Rc::new(WebSocketRpcClient::new(Box::new(|| {
                    Rc::new(platform::WebSocketRpcTransport::new())
                })));
            }
        }
    }

    /// Drops this [`Jason`] API object, so all the related objects (rooms,
    /// connections, streams, etc.) respectively. All objects related to this
    /// [`Jason`] API object will be detached (you will still hold them, but
    /// unable to use).
    pub fn dispose(self) {
        self.0.borrow_mut().rooms.drain(..).for_each(|room| {
            room.close(ClientDisconnect::RoomClosed.into());
        });
    }

    /// Returns a new [`Jason`] with the provided [`WebSocketRpcClient`].
    pub fn with_rpc_client(rpc: Rc<WebSocketRpcClient>) -> Self {
        Self(Rc::new(RefCell::new(Inner {
            rpc,
            rooms: Vec::new(),
            media_manager: Rc::new(MediaManager::default()),
        })))
    }

    /// Returns a [`RoomHandle`] for an initialized  [`Room`].
    fn inner_init_room(&self, rpc: Rc<dyn RpcSession>) -> RoomHandle {
        let on_normal_close = rpc.on_normal_close();
        let room = Room::new(rpc, Rc::clone(&self.0.borrow().media_manager));

        let weak_room = room.downgrade();
        let weak_inner = Rc::downgrade(&self.0);
        platform::spawn(on_normal_close.map(move |reason| {
            let _ = (|| {
                let this_room = weak_room.upgrade()?;
                let inner = weak_inner.upgrade()?;
                let mut inner = inner.borrow_mut();
                let index =
                    inner.rooms.iter().position(|r| r.ptr_eq(&this_room));
                if let Some(i) = index {
                    inner.rooms.remove(i).close(reason);
                }
                Some(())
            })();
        }));

        let handle = room.new_handle();
        self.0.borrow_mut().rooms.push(room);
        handle
    }
}

impl Default for Jason {
    fn default() -> Self {
        Self::new()
    }
}
