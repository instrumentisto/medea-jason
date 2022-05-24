//! [`Member`] definitions.

use std::{collections::HashMap, fmt, time::Duration};

use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};

use super::{endpoint, room, Endpoint};

/// `ID` of a [`Member`].
#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    Eq,
    From,
    Hash,
    Into,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct Id(pub String);

/// [`ControlApi`]'s `Member` element specification.
///
/// [`ControlApi`]: crate::ControlApi
#[derive(Clone, Debug)]
pub struct Member {
    /// `ID` of this [`Member`].
    pub id: Id,

    /// Spec of this [`Member`].
    pub pipeline: HashMap<endpoint::Id, Endpoint>,

    /// [`Credentials`] to authorize [`Member`] with.
    pub credentials: Credentials,

    /// `OnJoin` `URL` which will be sent to the [`CallbackApi`]. Nothing will
    /// be sent, in case it's [`None`].
    ///
    /// [`CallbackApi`]: crate::CallbackApi
    pub on_join: Option<String>,

    /// `OnLeave` `URL` which will be sent to the [`CallbackApi`]. Nothing will
    /// be sent, in case it's [`None`].
    ///
    /// [`CallbackApi`]: crate::CallbackApi
    pub on_leave: Option<String>,

    /// Timeout of receiving heartbeat messages from the [`Member`] via
    /// [Client API].
    ///
    /// Once reached, the [`Member`] is considered being idle.
    ///
    /// [Client Api]: https://tinyurl.com/266y74tf
    pub idle_timeout: Option<Duration>,

    /// Timeout of the [`Member`] reconnecting via [Client API].
    ///
    /// Once reached, the [`Member`] is considered disconnected.
    ///
    /// [Client Api]: https://tinyurl.com/266y74tf
    pub reconnect_timeout: Option<Duration>,

    /// Interval of sending `Ping`s to the [`Member`] via [Client API].
    ///
    /// [Client Api]: https://tinyurl.com/266y74tf
    pub ping_interval: Option<Duration>,
}

/// `URI` used by [`Member`]s to connect to a media server via [Client API].
///
/// [Client Api]: https://tinyurl.com/266y74tf
#[derive(Clone, Debug)]
pub struct Sid {
    /// Public `URL` to establish `WebSocket` connection with.
    pub public_url: PublicUrl,

    /// [`Id`] of the [`Room`] the [`Member`] participates in.
    ///
    /// [`Id`]: room::Id
    pub room_id: room::Id,

    /// [`Id`] of the [`Member`] participating in the [`Room`].
    pub member_id: Id,

    /// [`Credentials`] of the [`Member`] to authorize his connection with.
    pub credentials: Credentials,
}

impl fmt::Display for Sid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}/{}", self.public_url, self.room_id, self.member_id)?;
        if let Credentials::Plain(plain) = &self.credentials {
            write!(f, "?token={plain}")?;
        }
        Ok(())
    }
}

/// Public `URL` of HTTP server. Address for exposed [Client API].
/// It's assumed that HTTP server can be reached via this URL externally.
///
/// This address is returned from [Control API] in `sids` field and [Jason]
/// uses this address to start its session.
///
/// [Client Api]: https://tinyurl.com/266y74tf
/// [Control API]: https://tinyurl.com/yxsqplq7
/// [Jason]: https://github.com/instrumentisto/medea-jason
#[derive(Clone, Debug, Display, Deserialize, Serialize, From)]
pub struct PublicUrl(pub String);

/// Credentials of the [`Member`] element.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Credentials {
    /// [Argon2] hash of the `Member` credential.
    ///
    /// [Argon2]: https://en.wikipedia.org/wiki/Argon2
    Hash(String),

    /// Plain text `Member` credentials.
    Plain(String),
}

impl Credentials {
    /// Length of [`Credentials`].
    const LEN: usize = 32;
}

impl Default for Credentials {
    fn default() -> Self {
        use rand::{distributions::Alphanumeric, Rng as _};

        Self::Plain(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(Self::LEN)
                .map(char::from)
                .collect(),
        )
    }
}
