//! [`Endpoint`] definitions.

pub mod web_rtc_play;
pub mod web_rtc_publish;

use derive_more::{Display, From, Into};

#[doc(inline)]
pub use self::{web_rtc_play::WebRtcPlay, web_rtc_publish::WebRtcPublish};

/// Media [`Element`] flowing one or more media data streams through itself.
///
/// [`Element`]: crate::Element
#[derive(Clone, Debug, From)]
pub enum Endpoint {
    /// [`WebRtcPublish`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    WebRtcPublish(WebRtcPublish),

    /// [`WebRtcPlay`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    WebRtcPlay(WebRtcPlay),
}

impl Endpoint {
    /// Return [`Id`] of this [`Endpoint`].
    #[must_use]
    pub fn id(&self) -> Id {
        match self {
            Endpoint::WebRtcPublish(publish) => publish.id.clone().into(),
            Endpoint::WebRtcPlay(play) => play.id.clone().into(),
        }
    }
}

/// ID of an [`Endpoint`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(
    Clone, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
#[from(types(String, web_rtc_publish::Id, web_rtc_play::Id))]
#[into(owned(types(String)))]
pub struct Id(Box<str>);

// TODO: Derive via `derive::From` once it's capable to.
impl<'a> From<&'a str> for Id {
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}
