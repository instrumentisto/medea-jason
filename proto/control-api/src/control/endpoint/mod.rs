//! [`Endpoint`] definitions.

pub mod web_rtc_play;
pub mod web_rtc_publish;

use derive_more::{AsRef, Display, From, Into};
use ref_cast::RefCast;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[doc(inline)]
pub use self::{web_rtc_play::WebRtcPlay, web_rtc_publish::WebRtcPublish};

/// Media [`Element`] flowing one or more media data streams through itself.
///
/// [`Element`]: crate::Element
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Endpoint {
    /// ID of this [`Endpoint`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub id: Id,

    /// [`Endpoint`] spec.
    pub spec: Spec,
}

impl From<WebRtcPlay> for Endpoint {
    fn from(play: WebRtcPlay) -> Self {
        Self {
            id: play.id.into(),
            spec: play.spec.into(),
        }
    }
}

impl From<WebRtcPublish> for Endpoint {
    fn from(publish: WebRtcPublish) -> Self {
        Self {
            id: publish.id.into(),
            spec: publish.spec.into(),
        }
    }
}

/// [`Endpoint`] spec.
#[allow(variant_size_differences)]
#[derive(Clone, Debug, Eq, From, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind"))]
pub enum Spec {
    /// [`WebRtcPublish`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    WebRtcPublishEndpoint {
        /// Spec of the [`WebRtcPublish`] media [`Element`].
        ///
        /// [`Element`]: crate::Element
        spec: web_rtc_publish::Spec,
    },

    /// [`WebRtcPlay`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    WebRtcPlayEndpoint {
        /// Spec of the [`WebRtcPlay`] media [`Element`].
        ///
        /// [`Element`]: crate::Element
        spec: web_rtc_play::Spec,
    },
}

/// ID of an [`Endpoint`] media [`Element`].
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
#[from(types(String, web_rtc_publish::Id, web_rtc_play::Id))]
#[into(owned(types(String)))]
#[repr(transparent)]
pub struct Id(Box<str>);

// TODO: Derive via `derive::From` once it's capable to.
impl<'a> From<&'a str> for Id {
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}
