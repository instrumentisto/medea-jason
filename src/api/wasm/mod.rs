//! External [`Jason`] API for `wasm32-unknown-unknown` target, designed to be
//! used in a web environment with JavaScript.
//!
//! [`Jason`]: crate::api::Jason

// TODO: See https://github.com/rustwasm/wasm-bindgen/pull/2719
#![allow(clippy::use_self)]

pub mod connection_handle;
pub mod err;
pub mod jason;
pub mod local_media_track;
pub mod media_device_info;
pub mod media_manager_handle;
pub mod media_stream_settings;
pub mod reconnect_handle;
pub mod remote_media_track;
pub mod room_close_reason;
pub mod room_handle;

use derive_more::Display;
use wasm_bindgen::prelude::*;

use crate::media;

pub use self::{
    connection_handle::ConnectionHandle,
    err::Error,
    jason::Jason,
    local_media_track::LocalMediaTrack,
    media_device_info::MediaDeviceInfo,
    media_manager_handle::MediaManagerHandle,
    media_stream_settings::{
        AudioTrackConstraints, DeviceVideoTrackConstraints,
        DisplayVideoTrackConstraints, MediaStreamSettings,
    },
    reconnect_handle::ReconnectHandle,
    remote_media_track::RemoteMediaTrack,
    room_close_reason::RoomCloseReason,
    room_handle::RoomHandle,
};

/// [MediaStreamTrack.kind][1] representation.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-kind
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum MediaKind {
    /// Audio track.
    Audio,

    /// Video track.
    Video,
}

impl From<media::MediaKind> for MediaKind {
    fn from(that: media::MediaKind) -> Self {
        match that {
            media::MediaKind::Audio => Self::Audio,
            media::MediaKind::Video => Self::Video,
        }
    }
}

impl From<MediaKind> for media::MediaKind {
    fn from(that: MediaKind) -> Self {
        match that {
            MediaKind::Audio => Self::Audio,
            MediaKind::Video => Self::Video,
        }
    }
}

/// [MediaDeviceInfo.kind][1] representation.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-kind
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum MediaDeviceKind {
    /// Audio input device (for example, a microphone).
    AudioInput,

    /// Video input device (for example, a webcam).
    VideoInput,

    /// Audio output device (for example, a pair of headphones).
    AudioOutput,
}

impl From<media::MediaDeviceKind> for MediaDeviceKind {
    fn from(that: media::MediaDeviceKind) -> Self {
        use media::MediaDeviceKind as K;

        match that {
            K::AudioInput => Self::AudioInput,
            K::VideoInput => Self::VideoInput,
            K::AudioOutput => Self::AudioOutput,
        }
    }
}

/// Media source type.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum MediaSourceKind {
    /// Media is sourced from some media device (webcam or microphone).
    Device,

    /// Media is obtained via screen capturing.
    Display,
}

impl From<media::MediaSourceKind> for MediaSourceKind {
    fn from(that: media::MediaSourceKind) -> Self {
        match that {
            media::MediaSourceKind::Device => Self::Device,
            media::MediaSourceKind::Display => Self::Display,
        }
    }
}

impl From<MediaSourceKind> for media::MediaSourceKind {
    fn from(that: MediaSourceKind) -> Self {
        match that {
            MediaSourceKind::Device => Self::Device,
            MediaSourceKind::Display => Self::Display,
        }
    }
}

/// Describes directions that a camera can face, as seen from a user's
/// perspective. Representation of a [VideoFacingModeEnum][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-videofacingmodeenum
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum FacingMode {
    /// Facing towards a user (a self-view camera).
    User,

    /// Facing away from a user (viewing the environment).
    Environment,

    /// Facing to the left of a user.
    Left,

    /// Facing to the right of a user.
    Right,
}

impl From<media::FacingMode> for FacingMode {
    fn from(that: media::FacingMode) -> Self {
        use media::FacingMode as M;

        match that {
            M::User => Self::User,
            M::Environment => Self::Environment,
            M::Left => Self::Left,
            M::Right => Self::Right,
        }
    }
}

impl From<FacingMode> for media::FacingMode {
    fn from(val: FacingMode) -> Self {
        match val {
            FacingMode::User => Self::User,
            FacingMode::Environment => Self::Environment,
            FacingMode::Left => Self::Left,
            FacingMode::Right => Self::Right,
        }
    }
}

/// Media exchange direction of a `Track`.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum MediaDirection {
    /// `Track` is enabled on recv and send sides.
    SendRecv,

    /// `Track` is enabled on send side.
    SendOnly,

    /// `Track` is enabled on recv side.
    RecvOnly,

    /// `Track` is disabled on both sides.
    Inactive,
}

impl From<MediaDirection> for media::MediaDirection {
    fn from(val: MediaDirection) -> Self {
        match val {
            MediaDirection::SendRecv => Self::SendRecv,
            MediaDirection::SendOnly => Self::SendOnly,
            MediaDirection::RecvOnly => Self::RecvOnly,
            MediaDirection::Inactive => Self::Inactive,
        }
    }
}

impl From<media::MediaDirection> for MediaDirection {
    fn from(val: media::MediaDirection) -> Self {
        use media::MediaDirection as D;

        match val {
            D::SendRecv => Self::SendRecv,
            D::SendOnly => Self::SendOnly,
            D::RecvOnly => Self::RecvOnly,
            D::Inactive => Self::Inactive,
        }
    }
}

impl From<MediaDirection> for JsValue {
    fn from(val: MediaDirection) -> Self {
        JsValue::from(val as u8)
    }
}
