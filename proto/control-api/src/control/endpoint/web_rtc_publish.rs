//! [`WebRtcPublish`] [`Endpoint`] definitions.
//!
//! [`Endpoint`]: crate::Endpoint

use derive_more::{Display, From, Into};
use smart_default::SmartDefault;

/// Media [`Element`] receiving media data from a client via [WebRTC] (allows to
/// publish media data).
///
/// [`Element`]: crate::Element
/// [WebRTC]: https://w3.org/TR/webrtc
#[derive(Clone, Debug)]
pub struct WebRtcPublish {
    /// ID of this [`WebRtcPublish`] [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub id: Id,

    /// Peer-to-peer mode of this [`WebRtcPublish`] [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub p2p: P2pMode,

    /// Indicator whether to relay all media data through a [TURN] server
    /// forcibly.
    ///
    /// [TURN]: https://webrtc.org/getting-started/turn-server
    pub force_relay: bool,

    /// Settings for the audio media type of this [`WebRtcPublish`]
    /// [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub audio_settings: AudioSettings,

    /// Settings for the video media type of this [`WebRtcPublish`]
    /// [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub video_settings: VideoSettings,
}

/// ID of a [`WebRtcPublish`] media [`Element`]
///
/// [`Element`]: crate::Element
#[derive(
    Clone, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
#[from(types(String))]
#[into(owned(types(String)))]
pub struct Id(Box<str>);

// TODO: Derive via `derive::From` once it's capable to.
impl<'a> From<&'a str> for Id {
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}

/// Possible peer-to-peer modes of [WebRTC] interaction in a [`WebRtcPublish`]
/// media [`Element`].
///
/// [`Element`]: crate::Element
/// [WebRTC]: https://w3.org/TR/webrtc
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum P2pMode {
    /// Never use peer-to-peer connections and always send media data through a
    /// media server.
    Never = 0,

    /// Use peer-to-peer connections directly if it's possible, otherwise send
    /// media data through a media server.
    IfPossible = 1,

    /// Send media data via peer-to-peer connections only, and never through a
    /// media server.
    Always = 2,
}

/// Audio media type settings of a [`WebRtcPublish`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(Clone, Copy, Debug, Default)]
pub struct AudioSettings {
    /// [`Policy`] to publish the audio media type with.
    pub publish_policy: Policy,
}

/// Video media type settings of a [`WebRtcPublish`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(Clone, Copy, Debug, Default)]
pub struct VideoSettings {
    /// [`Policy`] to publish the video media type with.
    pub publish_policy: Policy,
}

/// Policy of how a video or an audio media type can be published in a
/// [`WebRtcPublish`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(Clone, Copy, Debug, Eq, PartialEq, SmartDefault)]
pub enum Policy {
    /// Media type __may__ be published.
    ///
    /// Media server will try to initialize publishing, but won't produce any
    /// errors if user application fails to (or chooses not to) acquire the
    /// required media track. Media server will approve user requests to stop
    /// and to restart publishing the specified media type.
    #[default]
    Optional = 0,

    /// Media type __must__ be published.
    ///
    /// Media server will try to initialize publishing, and if the required
    /// media track cannot be acquired, then an error will be thrown. Media
    /// server will deny all requests to stop publishing.
    Required = 1,

    /// Media type __must not__ be published.
    ///
    /// Media server will not try to initialize publishing.
    Disabled = 2,
}
