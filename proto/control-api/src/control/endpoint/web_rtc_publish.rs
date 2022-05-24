//! [`WebRtcPublish`] definitions.

use derive_more::{Display, From, Into};
use smart_default::SmartDefault;

/// `ID` of [`WebRtcPublish`].
#[derive(
    Clone, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
pub struct Id(pub String);

/// Media element which is able to publish media data for another client via
/// `WebRTC`.
#[derive(Clone, Debug)]
pub struct WebRtcPublish {
    /// `ID` og this [`WebRtcPublish`].
    pub id: Id,

    /// Peer-to-peer mode of this [`WebRtcPublish`].
    pub p2p: P2pMode,

    /// Option to relay all media through a `TURN` server forcibly.
    pub force_relay: bool,

    /// Settings for the audio media type of the [`WebRtcPublish`].
    pub audio_settings: AudioSettings,

    /// Settings for the video media type of the [`WebRtcPublish`].
    pub video_settings: VideoSettings,
}

/// Peer-to-peer mode of [`WebRtcPublish`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum P2pMode {
    /// Never connect peer-to-peer.
    Never = 0,

    /// Connect peer-to-peer if it possible.
    IfPossible = 1,

    /// Always connect peer-to-peer.
    Always = 2,
}

/// Settings for the audio media type of the [`WebRtcPublish`].
#[derive(Clone, Copy, Debug, Default)]
pub struct AudioSettings {
    /// Publishing policy of the audio media type in the [`WebRtcPublish`].
    pub publish_policy: Policy,
}

/// Settings for the video media type of the [`WebRtcPublish`].
#[derive(Clone, Copy, Debug, Default)]
pub struct VideoSettings {
    /// Publishing policy of the video media type in the [`WebRtcPublish`].
    pub publish_policy: Policy,
}

/// Publishing policy of the video or audio media type in the [`WebRtcPublish`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, SmartDefault)]
pub enum Policy {
    /// Specified media type __may__ be published.
    ///
    /// Media server will try to initialize publishing, but won't
    /// produce any errors if user application will fail to
    /// or choose not to acquire required track. Media
    /// server will approve user request to stop and
    /// restart publishing specified media type.
    #[default]
    Optional = 0,

    /// Specified media type __must__ be published.
    ///
    /// Media server will try to initialize publishing. If required
    /// media track could not be acquired, then an error
    /// will be thrown. Media server will deny all requests
    /// to stop publishing.
    Required = 1,

    /// Media type __must__ not be published.
    ///
    /// Media server will not try to initialize publishing.
    Disabled = 2,
}
