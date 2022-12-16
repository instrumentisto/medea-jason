//! [`WebRtcPlay`] [`Endpoint`] definitions.
//!
//! [`Endpoint`]: crate::Endpoint

use std::str::FromStr;

use derive_more::{AsRef, Display, Error, From, Into};
use ref_cast::RefCast;
#[cfg(feature = "serde")]
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use url::Url;

use crate::control::{
    endpoint::{self, web_rtc_publish},
    member, room,
};

/// Media [`Element`] playing media data for a client via [WebRTC].
///
/// [`Element`]: crate::Element
/// [WebRTC]: https://w3.org/TR/webrtc
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebRtcPlay {
    /// ID of this [`WebRtcPlay`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub id: Id,

    /// [`Spec`] of this [`WebRtcPlay`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub spec: Spec,
}

/// Spec of a [`WebRtcPlay`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Spec {
    /// Source to play media data from.
    pub src: LocalSrcUri,

    /// Indicator whether to relay all media data through a [TURN] server
    /// forcibly.
    ///
    /// [TURN]: https://webrtc.org/getting-started/turn-server
    #[cfg_attr(feature = "serde", serde(default))]
    pub force_relay: bool,
}

/// ID of a [`WebRtcPlay`] media [`Element`].
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
#[from(types(String))]
#[into(owned(types(String)))]
#[repr(transparent)]
pub struct Id(Box<str>);

// TODO: Derive via `derive::From` once it's capable to.
impl<'a> From<&'a str> for Id {
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}

impl AsRef<endpoint::Id> for Id {
    fn as_ref(&self) -> &endpoint::Id {
        endpoint::Id::ref_cast(&self.0)
    }
}

/// [URI] describing a source of media data for a [`WebRtcPlay`] media
/// [`Element`] located locally on the same media server.
///
/// [`Element`]: crate::Element
/// [URI]: https://en.wikipedia.org/wiki/Uniform_Resource_Identifier
#[derive(Clone, Debug, Display, Eq, PartialEq)]
#[display(fmt = "local://{room_id}/{member_id}/{endpoint_id}")]
pub struct LocalSrcUri {
    /// ID of the [`Room`].
    ///
    /// [`Room`]: crate::Room
    pub room_id: room::Id,

    /// ID of the [`Member`].
    ///
    /// [`Member`]: crate::Member
    pub member_id: member::Id,

    /// ID of the [`WebRtcPublish`] [`Element`] the media data is produced by.
    ///
    /// [`Element`]: crate::Element
    /// [`WebRtcPublish`]: web_rtc_publish::WebRtcPublish
    pub endpoint_id: web_rtc_publish::Id,
}

impl FromStr for LocalSrcUri {
    type Err = LocalSrcUriParseError;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        if val.is_empty() {
            return Err(LocalSrcUriParseError::Empty);
        }

        let url = Url::parse(val)
            .map_err(|e| LocalSrcUriParseError::UrlParseErr(val.into(), e))?;
        if url.scheme() != "local" {
            return Err(LocalSrcUriParseError::NotLocal(val.into()));
        }

        let room_id = url
            .host_str()
            .filter(|h| !h.is_empty())
            .ok_or_else(|| LocalSrcUriParseError::MissingPaths(val.into()))?
            .into();

        let mut path = url
            .path_segments()
            .ok_or_else(|| LocalSrcUriParseError::MissingPaths(val.into()))?;

        let member_id = path
            .next()
            .filter(|id| !id.is_empty())
            .ok_or_else(|| LocalSrcUriParseError::MissingPaths(val.into()))?
            .into();

        let endpoint_id = path
            .next()
            .filter(|id| !id.is_empty())
            .ok_or_else(|| LocalSrcUriParseError::MissingPaths(val.into()))?
            .into();

        if path.next().is_some() {
            return Err(LocalSrcUriParseError::TooManyPaths(val.into()));
        }

        Ok(Self {
            room_id,
            member_id,
            endpoint_id,
        })
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for LocalSrcUri {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(de)?
            .parse::<Self>()
            .map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for LocalSrcUri {
    fn serialize<S>(&self, se: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        se.serialize_str(&self.to_string())
    }
}

/// Possible errors of parsing a [`LocalSrcUri`].
#[derive(Debug, Display, Error)]
pub enum LocalSrcUriParseError {
    /// Protocol of the provided URI is not `local://`.
    #[display(fmt = "Provided URI protocol is not `local://`: {_0}")]
    NotLocal(#[error(not(source))] Box<str>),

    /// Too many paths in the provided URI.
    ///
    /// `local://room_id/member_id/endpoint_id/redundant_path` for example.
    #[display(fmt = "Too many paths in URI: {_0}")]
    TooManyPaths(#[error(not(source))] Box<str>),

    /// Some paths are missing in the provided URI.
    ///
    /// `local://room_id//qwerty` for example.
    #[display(fmt = "Missing paths in URI: {_0}")]
    MissingPaths(#[error(not(source))] Box<str>),

    /// Error of parsing the provided URI.
    #[display(fmt = "Cannot parse provided URI `{_0}`: {_1}")]
    UrlParseErr(Box<str>, #[error(source)] url::ParseError),

    /// Provided URI is empty.
    #[display(fmt = "Provided URI cannot be empty")]
    Empty,
}
