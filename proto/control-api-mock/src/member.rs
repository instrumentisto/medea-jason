use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

/// Entity that represents a [Control API] [`Member`].
///
/// [Control API]: https://tinyurl.com/yxsqplq7
#[derive(Deserialize, Serialize, Debug)]
pub struct Member {
    /// ID of this [`Member`].
    #[serde(skip_deserializing)]
    pub id: String,

    /// [Control API] pipeline of this [`Member`].
    ///
    /// [Control API]: https://tinyurl.com/yxsqplq7
    pub pipeline: HashMap<String, Endpoint>,

    /// Optional credentials of this [`Member`].
    ///
    /// If [`None`] then random credentials will be generated on Medea side.
    pub credentials: Option<Credentials>,

    /// URL to which `OnJoin` Control API callback will be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_join: Option<String>,

    /// URL to which `OnLeave` Control API callback will be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_leave: Option<String>,

    /// Timeout of receiving heartbeat messages from this [`Member`] via Client
    /// API. Once reached, the [`Member`] is considered being idle.
    #[serde(default, with = "humantime_serde")]
    pub idle_timeout: Option<Duration>,

    /// Timeout of this [`Member`] reconnecting via Client API.
    /// Once reached, the [`Member`] is considered disconnected.
    #[serde(default, with = "humantime_serde")]
    pub reconnect_timeout: Option<Duration>,

    /// Interval of sending pings from Medea to this [`Member`] via Client API.
    #[serde(default, with = "humantime_serde")]
    pub ping_interval: Option<Duration>,
}

/// Credentials of the [`Member`].
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Credentials {
    /// [Argon2] hash of the [`Member`] credentials.
    ///
    /// [Argon2]: https://en.wikipedia.org/wiki/Argon2
    Hash(String),

    /// Plain text [`Member`] credentials.
    Plain(String),
}
