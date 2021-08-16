//! `Endpoint` related methods and entities.

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

/// P2P mode of [`WebRtcPublishEndpoint`].
#[derive(Debug, Deserialize, Serialize)]
pub enum P2pMode {
    Always,
    Never,
    IfPossible,
}

/// Publishing policy of the video or audio media type in the
/// [`WebRtcPublishEndpoint`].
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub enum PublishPolicy {
    /// Publish this media type if it possible.
    #[default]
    Optional,

    /// Don't start call if this media type can't be published.
    Required,

    /// Media type __must__ not be published.
    ///
    /// Media server will not try to initialize publishing.
    Disabled,
}

/// Settings for the audio media type of the [`WebRtcPublishEndpoint`].
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AudioSettings {
    /// Publishing policy of the audio media type in the
    /// [`WebRtcPublishEndpoint`].
    #[serde(default)]
    pub publish_policy: PublishPolicy,
}

/// Settings for the video media type of the [`WebRtcPublishEndpoint`].
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VideoSettings {
    /// Publishing policy of the video media type in the
    /// [`WebRtcPublishEndpoint`].
    #[serde(default)]
    pub publish_policy: PublishPolicy,
}

/// [Control API]'s `WebRtcPublishEndpoint` representation.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
#[derive(Debug, Deserialize, Serialize)]
pub struct WebRtcPublishEndpoint {
    /// ID of [`WebRtcPublishEndpoint`].
    #[serde(skip_deserializing)]
    pub id: String,

    /// Mode of connection for this [`WebRtcPublishEndpoint`].
    pub p2p: P2pMode,

    /// Option to relay all media through a TURN server forcibly.
    #[serde(default)]
    pub force_relay: bool,

    /// Settings for the audio media type of the [`WebRtcPublishEndpoint`].
    #[serde(default)]
    pub audio_settings: AudioSettings,

    /// Settings for the video media type of the [`WebRtcPublishEndpoint`].
    #[serde(default)]
    pub video_settings: VideoSettings,
}

/// [Control API]'s `WebRtcPlayEndpoint` element representation.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
#[derive(Debug, Deserialize, Serialize)]
pub struct WebRtcPlayEndpoint {
    /// ID of this [`WebRtcPlayEndpoint`].
    #[serde(skip_deserializing)]
    pub id: String,

    /// URI in format `local://{room_id}/{member_id}/{endpoint_id}` pointing to
    /// [`WebRtcPublishEndpoint`] which this [`WebRtcPlayEndpoint`] plays.
    pub src: String,

    /// Option to relay all media through a TURN server forcibly.
    #[serde(default)]
    pub force_relay: bool,
}

/// `Endpoint` element representation.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind")]
pub enum Endpoint {
    WebRtcPublishEndpoint(WebRtcPublishEndpoint),
    WebRtcPlayEndpoint(WebRtcPlayEndpoint),
}
