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
    /// State in [P2P mesh] mode.
    ///
    /// [P2P mesh]: https://webrtcglossary.com/mesh
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

/// [RTCPeerConnectionState][1] describing a state of a network connection
/// between two peers.
///
/// [1]: https://w3.org/TR/webrtc#dom-rtcpeerconnectionstate
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum PeerConnectionState {
    /// The connection was just created and has not yet started negotiating.
    New,

    /// The ICE agent is trying to establish a connection with the remote peer.
    Connecting,

    /// A connection has been successfully established and media/data can flow.
    Connected,

    /// The connection has been temporarily lost (e.g., network issue). ICE
    /// will try to reconnect.
    Disconnected,

    /// The connection failed completely (e.g., ICE failed, DTLS error).
    Failed,

    /// The connection has been closed.
    Closed,
}

// TODO: Use `derive_more` once its capable of it.
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
