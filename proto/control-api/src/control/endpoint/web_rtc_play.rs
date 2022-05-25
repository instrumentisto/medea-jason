//! [`WebRtcPlay`] [`Endpoint`] definitions.
//!
//! [`Endpoint`]: crate::Endpoint

use derive_more::{Display, Error, From, Into};
use url::Url;

use crate::control::{
    endpoint::web_rtc_publish, member, room, ErrorCode, ErrorResponse,
};

/// Media [`Element`] playing media data for a client via [WebRTC].
///
/// [`Element`]: crate::Element
/// [WebRTC]: https://w3.org/TR/webrtc
#[derive(Clone, Debug)]
pub struct WebRtcPlay {
    /// ID of this [`WebRtcPlay`] [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub id: Id,

    /// Source to play media data from.
    pub src: LocalSrcUri,

    /// Indicator whether to relay all media data through a [TURN] server
    /// forcibly.
    ///
    /// [TURN]: https://webrtc.org/getting-started/turn-server
    pub force_relay: bool,
}

/// ID of a [`WebRtcPlay`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(
    Clone, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
#[from(types(String))]
#[into(owned(types(String)))]
pub struct Id(Box<str>);

/// [URI] describing a source of media data for a [`WebRtcPlay`] media
/// [`Element`] located locally on the same media server.
///
/// [`Element`]: crate::Element
/// [URI]: https://en.wikipedia.org/wiki/Uniform_Resource_Identifier
#[derive(Clone, Debug, Display)]
#[display(fmt = "local://{}/{}/{}", room_id, member_id, endpoint_id)]
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

impl TryFrom<String> for LocalSrcUri {
    type Error = LocalSrcUriParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(LocalSrcUriParseError::Empty);
        }

        let url = match Url::parse(&value) {
            Ok(url) => url,
            Err(err) => {
                return Err(LocalSrcUriParseError::UrlParseErr(value, err))
            }
        };
        if url.scheme() != "local" {
            return Err(LocalSrcUriParseError::NotLocal(value));
        }

        let room_id = match url.host_str() {
            Some(host) => {
                if host.is_empty() {
                    return Err(LocalSrcUriParseError::MissingPaths(value));
                }
                room::Id::from(host.to_owned())
            }
            None => return Err(LocalSrcUriParseError::MissingPaths(value)),
        };

        let mut path = url.path_segments().ok_or_else(|| {
            LocalSrcUriParseError::MissingPaths(value.clone())
        })?;

        let member_id = path
            .next()
            .filter(|id| !id.is_empty())
            .map(|id| member::Id::from(id.to_owned()))
            .ok_or_else(|| {
                LocalSrcUriParseError::MissingPaths(value.clone())
            })?;

        let endpoint_id = path
            .next()
            .filter(|id| !id.is_empty())
            .map(|id| web_rtc_publish::Id::from(id.to_owned()))
            .ok_or_else(|| {
                LocalSrcUriParseError::MissingPaths(value.clone())
            })?;

        if path.next().is_some() {
            return Err(LocalSrcUriParseError::TooManyPaths(value));
        }

        Ok(Self {
            room_id,
            member_id,
            endpoint_id,
        })
    }
}

/// Possible errors of parsing a [`LocalSrcUri`].
#[derive(Debug, Display, Error)]
pub enum LocalSrcUriParseError {
    /// Protocol of the provided URI is not `local://`.
    #[display(fmt = "Provided URI protocol is not `local://`: {}", _0)]
    NotLocal(#[error(not(source))] String),

    /// Too many paths in the provided URI.
    ///
    /// `local://room_id/member_id/endpoint_id/redundant_path` for example.
    #[display(fmt = "Too many paths in provided URI: {}", _0)]
    TooManyPaths(#[error(not(source))] String),

    /// Some paths are missing in the provided URI.
    ///
    /// `local://room_id//qwerty` for example.
    #[display(fmt = "Missing paths in provided URI: {}", _0)]
    MissingPaths(#[error(not(source))] String),

    /// Error of parsing the provided URI.
    #[display(fmt = "Cannot parse provided URI `{}`: {}", _0, _1)]
    UrlParseErr(String, #[error(source)] url::ParseError),

    /// Provided URI is empty.
    #[display(fmt = "Provided URI cannot be empty")]
    Empty,
}

impl From<LocalSrcUriParseError> for ErrorResponse {
    fn from(err: LocalSrcUriParseError) -> Self {
        use LocalSrcUriParseError as E;

        match err {
            E::NotLocal(text) => {
                Self::new(ErrorCode::ElementIdIsNotLocal, &text)
            }
            E::TooManyPaths(text) => {
                Self::new(ErrorCode::ElementIdIsTooLong, &text)
            }
            E::Empty => Self::without_id(ErrorCode::EmptyElementId),
            E::MissingPaths(text) => {
                Self::new(ErrorCode::MissingFieldsInSrcUri, &text)
            }
            E::UrlParseErr(id, _) => Self::new(ErrorCode::InvalidSrcUri, &id),
        }
    }
}
