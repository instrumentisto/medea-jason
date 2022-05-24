//! [`Endpoint`] definitions.

pub mod web_rtc_play;
pub mod web_rtc_publish;

use derive_more::{Display, From, Into};

pub use self::{web_rtc_play::WebRtcPlay, web_rtc_publish::WebRtcPublish};

/// `ID` of an [`Endpoint`].
#[derive(
    Clone, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
#[from(types(web_rtc_publish::Id, web_rtc_play::Id))]
pub struct Id(pub String);

/// Media element that one or more media data streams flow through.
#[allow(variant_size_differences)]
#[derive(Clone, Debug, From)]
pub enum Endpoint {
    /// [`WebRtcPublish`] element.
    WebRtcPublish(WebRtcPublish),

    /// [`WebRtcPlay`] element.
    WebRtcPlay(WebRtcPlay),
}
