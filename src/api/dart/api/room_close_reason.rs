//! Reason of a [`Room`] closing.

use derive_more::From;
use flutter_rust_bridge::frb;

use crate::{api::dart::api::ForeignClass, room as core};

#[cfg(doc)]
use crate::room::Room;

/// Reason of why a [`Room`] is closed.
#[derive(Debug, From)]
pub struct RoomCloseReason {
    /// Returns the [`Room`]'s close reason.
    pub reason: String,

    /// Indicates whether the [`Room`] was closed by server.
    pub is_closed_by_server: bool,

    /// Indicates whether the [`Room`] close reason is considered as an error.
    pub is_err: bool,
}

impl From<core::RoomCloseReason> for RoomCloseReason {
    fn from(r: core::RoomCloseReason) -> Self {
        Self {
            reason: r.reason,
            is_closed_by_server: r.is_closed_by_server,
            is_err: r.is_err,
        }
    }
}

impl ForeignClass for RoomCloseReason {}
