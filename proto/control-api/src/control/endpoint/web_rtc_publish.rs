//! [`WebRtcPublish`] [`Endpoint`] definitions.
//!
//! [`Endpoint`]: crate::Endpoint

use derive_more::{AsRef, Display, From, Into};
use ref_cast::RefCast;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::control::endpoint;

/// Media [`Element`] receiving media data from a client via [WebRTC] (allows to
/// publish media data).
///
/// [`Element`]: crate::Element
/// [WebRTC]: https://w3.org/TR/webrtc
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebRtcPublish {
    /// ID of this [`WebRtcPublish`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub id: Id,

    /// [`Spec`] of this [`WebRtcPublish`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub spec: Spec,
}

/// Spec of a [`WebRtcPublish`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Spec {
    /// Peer-to-peer mode of this [`WebRtcPublish`] [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub p2p: P2pMode,

    /// Indicator whether to relay all media data through a [TURN] server
    /// forcibly.
    ///
    /// [TURN]: https://webrtc.org/getting-started/turn-server
    #[cfg_attr(feature = "serde", serde(default))]
    pub force_relay: bool,

    /// Settings for the audio media type of this [`WebRtcPublish`]
    /// [`Element`].
    ///
    /// [`Element`]: crate::Element
    #[cfg_attr(feature = "serde", serde(default))]
    pub audio_settings: AudioSettings,

    /// Settings for the video media type of this [`WebRtcPublish`]
    /// [`Element`].
    ///
    /// [`Element`]: crate::Element
    #[cfg_attr(feature = "serde", serde(default))]
    pub video_settings: VideoSettings,
}

/// ID of a [`WebRtcPublish`] media [`Element`]
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
#[from(types(String))]
#[into(owned(types(String)))]
#[repr(transparent)]
pub struct Id(Box<str>);

// TODO: Derive via `derive::From` once it's capable to.
impl<'a> From<&'a str> for Id {
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}

impl AsRef<endpoint::Id> for Id {
    fn as_ref(&self) -> &endpoint::Id {
        endpoint::Id::ref_cast(&self.0)
    }
}

/// Possible peer-to-peer modes of [WebRTC] interaction in a [`WebRtcPublish`]
/// media [`Element`].
///
/// [`Element`]: crate::Element
/// [WebRTC]: https://w3.org/TR/webrtc
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AudioSettings {
    /// [`Policy`] to publish the audio media type with.
    #[cfg_attr(feature = "serde", serde(default))]
    pub publish_policy: Policy,
}

/// Video media type settings of a [`WebRtcPublish`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct VideoSettings {
    /// [`Policy`] to publish the video media type with.
    #[cfg_attr(feature = "serde", serde(default))]
    pub publish_policy: Policy,
}

/// Policy of how a video or an audio media type can be published in a
/// [`WebRtcPublish`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
