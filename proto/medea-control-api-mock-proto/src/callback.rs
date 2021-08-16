//! Control API Callback service implementation.

use serde::{Deserialize, Serialize};

/// All callbacks which can happen.
#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum CallbackEvent {
    OnJoin(join::OnJoin),
    OnLeave(leave::OnLeave),
}

/// Control API callback.
#[derive(Clone, Deserialize, Serialize)]
pub struct CallbackItem {
    /// FID (Full ID) of element with which this event was occurred.
    pub fid: String,

    /// Event which occurred.
    pub event: CallbackEvent,

    /// Time on which callback was occurred.
    pub at: String,
}

/// `on_join` callback's related entities and implementations.
mod join {
    use serde::{Deserialize, Serialize};

    /// `OnJoin` callback for Control API.
    #[derive(Clone, Deserialize, Serialize)]
    pub struct OnJoin;
}

/// `on_leave` callback's related entities and implementations.
mod leave {
    use derive_more::Display;
    use serde::{Deserialize, Serialize};

    /// `OnLeave` callback of Control API.
    #[derive(Clone, Deserialize, Serialize)]
    pub struct OnLeave {
        /// Reason of why `Member` leaves.
        pub reason: OnLeaveReason,
    }

    /// Reason of why `Member` leaves.
    #[derive(Clone, Deserialize, Display, Serialize)]
    pub enum OnLeaveReason {
        /// `Member` was normally disconnected.
        Disconnected,

        /// Connection with `Member` was lost.
        LostConnection,

        /// Server is shutting down.
        ServerShutdown,

        /// `Member` was forcibly disconnected by server.
        Kicked,
    }
}
