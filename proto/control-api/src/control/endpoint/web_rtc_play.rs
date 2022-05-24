//! [`WebRtcPlay`] definitions.

use std::fmt;

use derive_more::{Display, Error, From, Into};
use url::Url;

use crate::control::{
    endpoint::web_rtc_publish, member, room, ErrorCode, ErrorResponse,
};

/// `ID` of a [`WebRtcPlay`].
#[derive(
    Clone, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
pub struct Id(pub String);

/// Media element which is able to play media data for client via
/// `WebRTC`.
#[derive(Clone, Debug)]
pub struct WebRtcPlay {
    /// `ID` of this [`WebRtcPlay`].
    pub id: Id,

    /// Source URI in format
    /// `local://{room_id}/{member_id}/{endpoint_id}`.
    pub src: SrcUri,

    /// Option to relay all media through a `TURN` server forcibly.
    pub force_relay: bool,
}

/// Special `URI` with pattern `local://{room_id}/{member_id}/{endpoint_id}`.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
/// [`EndpointId`]: crate::api::control::EndpointId
#[derive(Clone, Debug)]
pub struct SrcUri {
    /// `ID` of the [`Room`].
    ///
    /// [`Room`]: room::Room
    pub room_id: room::Id,

    /// `ID` of the [`Member`].
    ///
    /// [`Member`]: member::Member
    pub member_id: member::Id,

    /// `ID` of the [`WebRtcPublish`].
    ///
    /// [`WebRtcPublish`]: endpoint::WebRtcPublish
    pub endpoint_id: web_rtc_publish::Id,
}

impl fmt::Display for SrcUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "local://{}/{}/{}",
            self.room_id, self.member_id, self.endpoint_id,
        )
    }
}

impl TryFrom<String> for SrcUri {
    type Error = SrcUriParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(SrcUriParseError::Empty);
        }

        let url = match Url::parse(&value) {
            Ok(url) => url,
            Err(err) => return Err(SrcUriParseError::UrlParseErr(value, err)),
        };

        if url.scheme() != "local" {
            return Err(SrcUriParseError::NotLocal(value));
        }

        let room_id = match url.host() {
            Some(host) => {
                let host = host.to_string();
                if host.is_empty() {
                    return Err(SrcUriParseError::MissingPaths(value));
                }
                room::Id(host)
            }
            None => return Err(SrcUriParseError::MissingPaths(value)),
        };

        let mut path = url
            .path_segments()
            .ok_or_else(|| SrcUriParseError::MissingPaths(value.clone()))?;

        let member_id = path
            .next()
            .filter(|id| !id.is_empty())
            .map(|id| member::Id(id.into()))
            .ok_or_else(|| SrcUriParseError::MissingPaths(value.clone()))?;

        let endpoint_id = path
            .next()
            .filter(|id| !id.is_empty())
            .map(|id| web_rtc_publish::Id(id.into()))
            .ok_or_else(|| SrcUriParseError::MissingPaths(value.clone()))?;

        if path.next().is_some() {
            return Err(SrcUriParseError::TooManyPaths(value));
        }

        Ok(Self {
            room_id,
            member_id,
            endpoint_id,
        })
    }
}

/// Error which can happen while [`SrcUri`] parsing.
#[derive(Debug, Display, Error)]
pub enum SrcUriParseError {
    /// Protocol of provided URI is not "local://".
    #[display(fmt = "Provided URIs protocol is not `local://`")]
    NotLocal(#[error(not(source))] String),

    /// Too many paths in provided URI.
    ///
    /// `local://room_id/member_id/endpoint_id/redundant_path` for example.
    #[display(fmt = "Too many paths in provided URI ({})", _0)]
    TooManyPaths(#[error(not(source))] String),

    /// Some paths is missing in URI.
    ///
    /// `local://room_id//qwerty` for example.
    #[display(fmt = "Missing fields: {}", _0)]
    MissingPaths(#[error(not(source))] String),

    /// Error while parsing URI by [`url::Url`].
    #[display(fmt = "Error while parsing URL: {}", _0)]
    UrlParseErr(String, #[error(source)] url::ParseError),

    /// Provided empty URI.
    #[display(fmt = "Provided empty local URI")]
    Empty,
}

impl From<SrcUriParseError> for ErrorResponse {
    fn from(err: SrcUriParseError) -> Self {
        use SrcUriParseError as E;

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
