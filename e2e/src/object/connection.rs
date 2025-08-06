//! `Connection` JS object's representation.

use std::str::FromStr;

use serde::Serialize;

use super::Error;
use crate::{
    browser::Statement,
    object::{MediaKind, Object, room::ParsingFailedError, tracks_store},
};

/// Representation of a `Connection` JS object.
#[derive(Clone, Copy, Debug)]
pub struct Connection;

impl Object<Connection> {
    /// Returns a [`tracks_store::Remote`] of this [`Connection`].
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn tracks_store(
        &self,
    ) -> Result<Object<tracks_store::Remote>, Error> {
        self.execute_and_fetch(Statement::new(
            // language=JavaScript
            "async (conn) => conn.tracksStore",
            [],
        ))
        .await
    }

    /// Enables remote media receiving for the provided [`MediaKind`].
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn enable_remote_media(
        &self,
        kind: MediaKind,
    ) -> Result<(), Error> {
        let enable = match kind {
            MediaKind::Audio => "c.conn.enable_remote_audio()",
            MediaKind::Video => "c.conn.enable_remote_video()",
        };
        self.execute(Statement::new(
            // language=JavaScript
            &format!(
                "
                async (c) => {{
                    await {enable};
                }}
                ",
            ),
            [],
        ))
        .await
        .map(drop)
    }

    /// Disables remote media receiving for the provided [`MediaKind`].
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn disable_remote_media(
        &self,
        kind: MediaKind,
    ) -> Result<(), Error> {
        let disable = match kind {
            MediaKind::Audio => "c.conn.disable_remote_audio()",
            MediaKind::Video => "c.conn.disable_remote_video()",
        };
        self.execute(Statement::new(
            // language=JavaScript
            &format!(
                "
                async (c) => {{
                    await {disable};
                }}
                ",
            ),
            [],
        ))
        .await
        .map(drop)
    }

    /// Returns a [`Future`] resolving when `Connection.on_close()` callback is
    /// fired.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_close(&self) -> Result<(), Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (conn) => {
                await new Promise((resolve) => {
                    if (!conn.closeListener.isClosed) {
                        conn.closeListener.subs.push(resolve);
                    } else {
                        resolve();
                    }
                });
            }
            ",
            [],
        ))
        .await
        .map(drop)
    }
}

/// [`Connection`]'s state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", content = "value")]
pub enum MemberConnectionState {
    /// State in `P2P` mode.
    P2P(PeerConnectionState),
}

impl FromStr for MemberConnectionState {
    type Err = ParsingFailedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("::").collect::<Vec<_>>();

        if parts.len() != 2 {
            return Err(ParsingFailedError);
        }

        match parts[0] {
            "P2P" => Ok(Self::P2P(PeerConnectionState::from_str(parts[1])?)),
            _ => Err(ParsingFailedError),
        }
    }
}

/// `PeerConnection`'s connection state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum PeerConnectionState {
    /// At least one of the connection's ICE transports are in the
    /// `IceConnectionState::New` state, and none of them are in one
    /// of the following states: `IceConnectionState::Checking`,
    /// `IceConnectionState::Failed`, or
    /// `IceConnectionState::Disconnected`, or all of the connection's
    /// transports are in the `IceConnectionState::Closed` state.
    New,

    /// One or more of the ICE transports are currently in the process of
    /// establishing a connection; that is, their [`IceConnectionState`] is
    /// either `IceConnectionState::Checking` or
    /// `IceConnectionState::Connected`, and no transports are in the
    /// `IceConnectionState::Failed` state.
    Connecting,

    /// Every ICE transport used by the connection is either in use (state
    /// `IceConnectionState::Connected` or `IceConnectionState::Completed`)
    /// or is closed (`IceConnectionState::Closed`); in addition,
    /// at least one transport is either `IceConnectionState::Connected` or
    /// `IceConnectionState::Completed`.
    Connected,

    /// At least one of the ICE transports for the connection is in the
    /// `IceConnectionState::Disconnected` state and none of the other
    /// transports are in the state `IceConnectionState::Failed` or
    /// `IceConnectionState::Checking`.
    ///
    /// It's not a terminal state, and it can go back to `Connecting`
    /// and then `Connected` on its own.
    Disconnected,

    /// One or more of the ICE transports on the connection is in the
    /// `IceConnectionState::Failed` state.
    ///
    /// It's not a terminal state, and it can be fixed with ICE restart if
    /// signalling connection is alive.
    Failed,

    /// The `PeerConnection` is closed.
    ///
    /// It's a terminal state.
    Closed,
}

impl FromStr for PeerConnectionState {
    type Err = ParsingFailedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "New" => Self::New,
            "Connecting" => Self::Connecting,
            "Connected" => Self::Connected,
            "Disconnected" => Self::Disconnected,
            "Failed" => Self::Failed,
            "Closed" => Self::Closed,
            _ => return Err(ParsingFailedError),
        })
    }
}
