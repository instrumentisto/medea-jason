//! Reason of a [`Room`] closing.

#[cfg(doc)]
use crate::room::Room;
use crate::{api::dart::api::ForeignClass, room as core};

/// Reason of why a [`Room`] is closed.
#[derive(Debug)]
pub struct RoomCloseReason {
    /// [`Room`]'s close reason.
    pub reason: String,

    /// Indicator whether the [`Room`] was closed by server.
    pub is_closed_by_server: bool,

    /// Indicator whether the [`Room`] close reason is considered as an error.
    pub is_err: bool,
}

impl From<core::RoomCloseReasonImpl> for RoomCloseReason {
    fn from(r: core::RoomCloseReasonImpl) -> Self {
        Self {
            reason: r.reason,
            is_closed_by_server: r.is_closed_by_server,
            is_err: r.is_err,
        }
    }
}

impl ForeignClass for RoomCloseReason {}
