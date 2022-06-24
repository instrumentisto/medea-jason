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

    /// [`Spec`] of this [`Endpoint`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
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

/// Spec of an [`Endpoint`] media [`Element`].
///
/// [`Element`]: crate::Element
#[allow(variant_size_differences)]
#[derive(Clone, Debug, Eq, From, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", content = "spec"))]
pub enum Spec {
    /// [`WebRtcPublish`] media [`Element`] spec.
    ///
    /// [`Element`]: crate::Element
    WebRtcPublishEndpoint(web_rtc_publish::Spec),

    /// [`WebRtcPlay`] media [`Element`] spec.
    ///
    /// [`Element`]: crate::Element
    WebRtcPlayEndpoint(web_rtc_play::Spec),
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
#[from(types(String, web_rtc_play::Id, web_rtc_publish::Id))]
#[into(owned(types(String, web_rtc_play::Id, web_rtc_publish::Id)))]
#[repr(transparent)]
pub struct Id(Box<str>);

// TODO: Derive via `derive::From` once it's capable to.
impl<'a> From<&'a str> for Id {
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}

impl AsRef<web_rtc_play::Id> for Id {
    fn as_ref(&self) -> &web_rtc_play::Id {
        web_rtc_play::Id::ref_cast(&self.0)
    }
}

impl AsRef<web_rtc_publish::Id> for Id {
    fn as_ref(&self) -> &web_rtc_publish::Id {
        web_rtc_publish::Id::ref_cast(&self.0)
    }
}
