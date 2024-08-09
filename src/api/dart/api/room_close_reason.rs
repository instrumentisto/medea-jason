//! Reason of a [`Room`] closing.
//!
//! [`Room`]: room::Room

use derive_more::From;
use flutter_rust_bridge::frb;

use crate::{api::dart::api::ForeignClass, room as core};

/// Reason of why a [`Room`] is closed.
#[derive(Debug, From)]
#[frb(opaque)]
pub struct RoomCloseReason(core::RoomCloseReason);

impl RoomCloseReason {
    /// Returns the [`Room`]'s close reason.
    ///
    /// [`Room`]: room::Room
    #[frb(sync)]
    #[must_use]
    pub fn reason(&self) -> String {
        self.0.reason()
    }

    /// Indicates whether the [`Room`] was closed by server.
    ///
    /// [`Room`]: room::Room
    #[frb(sync)]
    #[must_use]
    pub fn is_closed_by_server(&self) -> bool {
        self.0.is_closed_by_server()
    }

    /// Indicates whether the [`Room`] close reason is considered as an error.
    ///
    /// [`Room`]: room::Room
    #[frb(sync)]
    #[must_use]
    pub fn is_err(&self) -> bool {
        self.0.is_err()
    }
}

impl ForeignClass for RoomCloseReason {}
