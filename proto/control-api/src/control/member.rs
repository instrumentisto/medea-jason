//! [`Member`] definitions.

use std::{
    collections::HashMap,
    fmt::Write as _,
    hash::{Hash, Hasher},
    time::Duration,
};

use derive_more::with_trait::{AsRef, Display, Error, From, FromStr, Into};
use ref_cast::RefCast;
use secrecy::{ExposeSecret as _, SecretString};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};
use url::Url;

use super::{Pipeline, endpoint, room};

/// Media [`Element`] representing a client authorized to participate in some
/// bigger media pipeline ([`Room`], for example).
///
/// [`Element`]: crate::Element
/// [`Room`]: crate::Room
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    /// ID of this [`Member`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub id: Id,

    /// [`Spec`] of this [`Member`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub spec: Spec,
}

/// Spec of a [`Member`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Spec {
    /// Media [`Pipeline`] representing this [`Member`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub pipeline: Pipeline<endpoint::Id, endpoint::Spec>,

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
    #[cfg_attr(feature = "serde", serde(default, with = "humantime_serde"))]
    pub idle_timeout: Option<Duration>,

    /// Timeout of reconnecting for this [`Member`] via [Client API].
    ///
    /// Once reached, this [`Member`] is considered disconnected.
    ///
    /// [Client API]: https://tinyurl.com/266y74tf
    #[cfg_attr(feature = "serde", serde(default, with = "humantime_serde"))]
    pub reconnect_timeout: Option<Duration>,

    /// Interval of pinging with heartbeat messages this [`Member`] via
    /// [Client API] by a media server.
    ///
    /// If [`None`] then the default interval of a media server is used, if
    /// configured.
    ///
    /// [Client API]: https://tinyurl.com/266y74tf
    #[cfg_attr(feature = "serde", serde(default, with = "humantime_serde"))]
    pub ping_interval: Option<Duration>,
}

/// ID of a [`Member`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(
    AsRef,
    Clone,
    Debug,
    Display,
    Eq,
    From,
    Hash,
    Into,
    Ord,
    PartialEq,
    PartialOrd,
    RefCast,
)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[from(&str, String)]
#[into(String)]
#[repr(transparent)]
pub struct Id(Box<str>);

#[cfg(feature = "client-api")]
impl From<medea_client_api_proto::MemberId> for Id {
    fn from(id: medea_client_api_proto::MemberId) -> Self {
        id.0.into()
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

    /// [`PlainCredentials`] of the [`Member`] to authenticate him with.
    pub creds: Option<PlainCredentials>,
}

impl Sid {
    /// Renders the [URI] string of this [`Sid`]
    ///
    /// [URI]: https://en.wikipedia.org/wiki/Uniform_Resource_Identifier
    // TODO: Return `SecretString` once `secrecy` crate allows to unwrap it:
    //       https://github.com/iqlusioninc/crates/issues/1182
    #[must_use]
    pub fn to_uri_string(&self) -> String {
        let mut sid =
            format!("{}/{}/{}", self.public_url, self.room_id, self.member_id);
        #[expect(clippy::expect_used, reason = "never fails")]
        if let Some(plain) = &self.creds {
            #[expect(clippy::expect_used, reason = "never fails")]
            write!(sid, "?token={}", plain.expose_str())
                .expect("writing to `String` never fails");
        }
        sid
    }
}

impl FromStr for Sid {
    type Err = ParseSidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut url = Url::parse(s)
            .map_err(|e| ParseSidError::InvalidUrl(s.into(), e))?;

        let creds = url.query_pairs().find_map(|(k, v)| {
            (k.as_ref() == "token").then(|| v.as_ref().into())
        });

        url.set_fragment(None);
        url.set_query(None);

        let err_missing = || ParseSidError::MissingPaths(s.into());
        let mut segments = url.path_segments().ok_or_else(err_missing)?.rev();
        let member_id = segments.next().ok_or_else(err_missing)?.into();
        let room_id = segments.next().ok_or_else(err_missing)?.into();

        // Removes last two segments.
        if let Ok(mut path) = url.path_segments_mut() {
            _ = path.pop().pop();
        }

        Ok(Self { public_url: url.into(), room_id, member_id, creds })
    }
}

/// Possible errors of parsing a [`Sid`].
#[derive(Debug, Display, Error)]
pub enum ParseSidError {
    /// Some paths are missing in the provided [URI].
    ///
    /// `ws://localhost:8080/ws//qwerty`, for example.
    ///
    /// [URI]: https://en.wikipedia.org/wiki/Uniform_Resource_Identifier
    #[display("Missing paths in URI: {_0}")]
    MissingPaths(#[error(not(source))] Box<str>),

    /// Error of parsing the provided [URI].
    ///
    /// [URI]: https://en.wikipedia.org/wiki/Uniform_Resource_Identifier
    #[display("Cannot parse provided URI `{_0}`: {_1}")]
    InvalidUrl(Box<str>, #[error(source)] url::ParseError),
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
#[derive(
    AsRef,
    Clone,
    Debug,
    Display,
    Eq,
    From,
    FromStr,
    Hash,
    Into,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct PublicUrl(Url);

/// Credentials of a [`Member`] media [`Element`] for its client side to
/// authorize via [Client API] with.
///
/// [`Element`]: crate::Element
/// [Client API]: https://tinyurl.com/266y74tf
#[derive(Clone, Debug, Eq, From, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Credentials {
    /// [Argon2] hash of credentials.
    ///
    /// [`Sid`] won't contain a `token` query parameter if
    /// [`Credentials::Hash`] is used, so it should be appended manually on
    /// a client side.
    ///
    /// [Argon2]: https://en.wikipedia.org/wiki/Argon2
    #[from(ignore)]
    Hash(Box<str>),

    /// Plain text credentials.
    Plain(PlainCredentials),
}

impl Credentials {
    /// Generates new random [`Credentials::Plain`].
    #[must_use]
    pub fn random() -> Self {
        use rand::{Rng as _, distr::Alphanumeric};

        Self::Plain(
            rand::rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect::<String>()
                .into(),
        )
    }
}

/// Plain [`Credentials`] returned in a [`Sid`].
#[derive(AsRef, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct PlainCredentials(SecretString);

impl PlainCredentials {
    /// Provides access to the underlying secret [`str`].
    #[must_use]
    pub fn expose_str(&self) -> &str {
        self.0.expose_secret()
    }
}

impl<T> From<T> for PlainCredentials
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into().into())
    }
}

impl Hash for PlainCredentials {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.expose_str().hash(state);
    }
}

impl Eq for PlainCredentials {}

impl PartialEq for PlainCredentials {
    fn eq(&self, other: &Self) -> bool {
        use subtle::ConstantTimeEq as _;

        self.expose_str().as_bytes().ct_eq(other.expose_str().as_bytes()).into()
    }
}

#[cfg(feature = "serde")]
impl Serialize for PlainCredentials {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.expose_secret().serialize(serializer)
    }
}
