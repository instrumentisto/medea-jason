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
    /// [`Room`]: room::Room
    pub room_id: room::Id,

    /// [`Id`] of the [`Member`] participating in the [`Room`].
    ///
    /// [`Room`]: room::Room
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

/// [`Sid`]s used by [`Member`]s to connect to a [Medea] server via
/// [Client API].
///
/// [`Sid`]: member::Sid
/// [Client Api]: https://tinyurl.com/266y74tf
/// [Medea]: https://git.instrumentisto.com/streaming/medea
pub type Sids = HashMap<Id, Sid>;

/// Public `URL` of HTTP server. Address for exposed [Client API].
/// It's assumed that HTTP server can be reached via this URL externally.
///
/// This address is returned from [Control API] in `sids` field and [Jason]
/// uses this address to start its session.
///
/// [Client Api]: https://tinyurl.com/266y74tf
/// [Control API]: https://tinyurl.com/yxsqplq7
/// [Jason]: https://github.com/instrumentisto/medea-jason
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
    /// [`Sid`] won't contain a `token` query parameter if [`Credentials::Hash`]
    /// are used, so it should be appended manually on a client side.
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
