//! [`WebRtcPlay`] definitions.

use derive_more::{Display, From, Into};
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::control::{endpoint::web_rtc_publish, member, room};

/// `ID` of a [`WebRtcPublish`].
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

/// Media element which is able to play media data for client via
/// `WebRTC`.
#[derive(Clone, Deserialize, Debug)]
pub struct WebRtcPlay {
    /// `ID` of this [`WebRtcPlay`].
    pub id: Id,

    /// Source URI in format
    /// `local://{room_id}/{member_id}/{endpoint_id}`.
    pub src: SrcUri,

    /// Option to relay all media through a `TURN` server forcibly.
    #[serde(default)]
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
    /// [`Room`]: crate::signalling::room::Room
    pub room_id: room::Id,

    /// `ID` of the [`Member`].
    pub member_id: member::Id,

    /// `ID` of the [`WebRtcPublish`].
    ///
    /// [`WebRtcPublish`]: endpoint::WebRtcPublish
    pub endpoint_id: web_rtc_publish::Id,
}

/// [Serde] deserializer for [`SrcUri`].
///
/// Deserializes URIs with pattern:
/// `local://room_id/member_id/publish_endpoint_id`.
///
/// [Serde]: serde
impl<'de> Deserialize<'de> for SrcUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// [`Visitor`] implementation for [`Deserialize`]ing
        /// [`SrcUri`].
        struct SrcUriVisitor;

        impl<'de> Visitor<'de> for SrcUriVisitor {
            type Value = SrcUri;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(
                    "URI in format: \
                             local://room_id/member_id/endpoint_id",
                )
            }

            fn visit_str<E>(self, value: &str) -> Result<SrcUri, E>
            where
                E: de::Error,
            {
                match SrcUri::try_from(value.to_owned()) {
                    Ok(src_uri) => Ok(src_uri),
                    Err(e) => Err(de::Error::custom(e)),
                }
            }
        }

        deserializer.deserialize_identifier(SrcUriVisitor)
    }
}

impl fmt::Display for SrcUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "local://{}/{}/{}",
            self.room_id, self.member_id, self.endpoint_id
        )
    }
}
