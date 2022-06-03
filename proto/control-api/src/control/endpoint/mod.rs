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
    /// Returns [`Id`] of this [`Endpoint`].
    #[must_use]
    pub fn id(&self) -> &Id {
        match self {
            Self::WebRtcPublish(publish) => Id::ref_cast(publish.id.as_ref()),
            Self::WebRtcPlay(play) => Id::ref_cast(play.id.as_ref()),
        }
    }
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
