//! Reason of a [`Room`] closing.

#[cfg(doc)]
use crate::room::Room;
use crate::{
    api::{RoomCloseKind, dart::api::ForeignClass},
    room as core,
};

/// Reason of why a [`Room`] is closed.
#[derive(Clone, Copy, Debug)]
pub struct RoomCloseReason {
    /// [`Room`]'s close reason.
    pub reason: RoomCloseKind,

    /// Indicator whether the [`Room`] was closed by server.
    pub is_closed_by_server: bool,
}

impl From<core::RoomCloseReasonImpl> for RoomCloseReason {
    fn from(r: core::RoomCloseReasonImpl) -> Self {
        Self { reason: r.reason, is_closed_by_server: r.is_closed_by_server }
    }
}

impl ForeignClass for RoomCloseReason {}
