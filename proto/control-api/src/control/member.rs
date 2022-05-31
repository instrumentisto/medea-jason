//! [`Member`] definitions.

use std::{collections::HashMap, fmt, time::Duration};

use derive_more::{Display, From, Into};
use url::Url;

use super::{endpoint, room, Endpoint};

/// Media [`Element`] representing a client authorized to participate in some
/// bigger media pipeline ([`Room`], for example).
///
/// [`Element`]: crate::Element
/// [`Room`]: crate::Room
#[derive(Clone, Debug)]
pub struct Member {
    /// ID of this [`Member`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub id: Id,

    /// Media pipeline representing this [`Member`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub pipeline: HashMap<endpoint::Id, Endpoint>,

    /// [`Credentials`] to authenticate this [`Member`] in [Client API] with.
    ///
    /// [`None`] if no authentication is required.
    ///
    /// [Client API]: https://tinyurl.com/266y74tf
    pub credentials: Option<Credentials>,

    /// [`Url`] of the callback to fire when this [`Member`] establishes a
    /// persistent connection with a media server via [Client API].
    ///
    /// [Client API]: https://tinyurl.com/266y74tf
    pub on_join: Option<Url>,

    /// [`Url`] of the callback to fire when this [`Member`] finishes a
    /// persistent connection with a media server via [Client API].
    ///
    /// [Client API]: https://tinyurl.com/266y74tf
    pub on_leave: Option<Url>,

    /// Timeout of receiving heartbeat messages from this [`Member`] via
    /// [Client API].
    ///
    /// Once reached, this [`Member`] is considered being idle.
    ///
    /// [Client API]: https://tinyurl.com/266y74tf
    pub idle_timeout: Option<Duration>,

    /// Timeout of reconnecting for this [`Member`] via [Client API].
    ///
    /// Once reached, this [`Member`] is considered disconnected.
    ///
    /// [Client API]: https://tinyurl.com/266y74tf
    pub reconnect_timeout: Option<Duration>,

    /// Interval of pinging with heartbeat messages this [`Member`] via
    /// [Client API] by a media server.
    ///
    /// If [`None`] then the default interval of a media server is used, if
    /// configured.
    ///
    /// [Client API]: https://tinyurl.com/266y74tf
    pub ping_interval: Option<Duration>,
}

/// ID of a [`Member`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(
    Clone, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
#[from(types(String))]
#[into(owned(types(String)))]
pub struct Id(Box<str>);

// TODO: Derive via `derive::From` once it's capable to.
impl<'a> From<&'a str> for Id {
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}

/// [URI] used by a [`Member`] to connect to a media server via [Client API].
///
/// [Client API]: https://tinyurl.com/266y74tf
/// [URI]: https://en.wikipedia.org/wiki/Uniform_Resource_Identifier
#[derive(Clone, Debug)]
pub struct Sid {
    /// Public [URL] to establish [WebSocket] connections with.
    ///
    /// [URL]: https://en.wikipedia.org/wiki/URL
    /// [WebSocket]: https://en.wikipedia.org/wiki/WebSocket
    pub public_url: PublicUrl,

    /// ID of the [`Room`] the [`Member`] participates in.
    ///
    /// [`Room`]: room::Room
    pub room_id: room::Id,

    /// ID of the [`Member`] who establishes [WebSocket] connections.
    ///
    /// [WebSocket]: https://en.wikipedia.org/wiki/WebSocket
    pub member_id: Id,

    /// [`Credentials`] of the [`Member`] to authenticate him with.
    pub creds: Credentials,
}

impl fmt::Display for Sid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}/{}", self.public_url, self.room_id, self.member_id)?;
        if let Credentials::Plain(plain) = &self.creds {
            write!(f, "?token={plain}")?;
        }
        Ok(())
    }
}

/// Collection of [`Sid`]s to be used by [`Member`]s to connect to a media
/// server via [Client API].
///
/// [Client API]: https://tinyurl.com/266y74tf
pub type Sids = HashMap<Id, Sid>;

/// Public [URL] of HTTP server exposing [Client API]. It's assumed that HTTP
/// server can be reached via this [URL] externally.
///
/// This address is returned from [`ControlApi`] in a [`Sid`] and a client side
/// should use this address to start its session.
///
/// [`ControlApi`]: crate::ControlApi
/// [Client API]: https://tinyurl.com/266y74tf
/// [URL]: https://en.wikipedia.org/wiki/URL
#[derive(Clone, Debug, Display, From)]
#[from(types(String))]
pub struct PublicUrl(Box<str>);

/// Credentials of a [`Member`] media [`Element`] for its client side to
/// authorize via [Client API] with.
///
/// [`Element`]: crate::Element
/// [Client API]: https://tinyurl.com/266y74tf
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Credentials {
    /// [Argon2] hash of credentials.
    ///
    /// [`Sid`] won't contain a `token` query parameter if
    /// [`Credentials::Hash`] is used, so it should be appended manually on
    /// a client side.
    ///
    /// [Argon2]: https://en.wikipedia.org/wiki/Argon2
    Hash(Box<str>),

    /// Plain text credentials.
    Plain(Box<str>),
}

impl Credentials {
    /// Generates new random [`Credentials::Plain`].
    #[must_use]
    pub fn random() -> Self {
        use rand::{distributions::Alphanumeric, Rng as _};

        Self::Plain(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect::<String>()
                .into(),
        )
    }
}

impl Default for Credentials {
    fn default() -> Self {
        Self::random()
    }
}
