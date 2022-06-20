//! [`Endpoint`] definitions.

pub mod web_rtc_play;
pub mod web_rtc_publish;

use derive_more::{AsRef, Display, From, Into};
use ref_cast::RefCast;

#[doc(inline)]
pub use self::{web_rtc_play::WebRtcPlay, web_rtc_publish::WebRtcPublish};

/// Media [`Element`] flowing one or more media data streams through itself.
///
/// [`Element`]: crate::Element
#[allow(variant_size_differences)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(tag = "kind"))]
#[derive(Clone, Debug, Eq, From, PartialEq)]
pub enum Endpoint {
    /// [`WebRtcPublish`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    WebRtcPublishEndpoint {
        /// [`WebRtcPublish`] media [`Element`].
        ///
        /// [`Element`]: crate::Element
        spec: WebRtcPublish,
    },

    /// [`WebRtcPlay`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    WebRtcPlayEndpoint {
        /// [`WebRtcPlay`] media [`Element`].
        ///
        /// [`Element`]: crate::Element
        spec: WebRtcPlay,
    },
}

/// ID of an [`Endpoint`] media [`Element`].
///
/// [`Element`]: crate::Element
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
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
