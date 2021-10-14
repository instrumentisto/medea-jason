//! Adapters to [Media Capture and Streams API][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams

pub mod constraints;
mod manager;
pub mod track;

use std::str::FromStr;

use derive_more::Display;

#[doc(inline)]
pub use self::{
    constraints::{
        AudioMediaTracksSettings, AudioTrackConstraints,
        DeviceVideoTrackConstraints, DisplayVideoTrackConstraints, FacingMode,
        LocalTracksConstraints, MediaStreamSettings,
        MultiSourceTracksConstraints, RecvConstraints, TrackConstraints,
        VideoSource, VideoTrackConstraints,
    },
    manager::{
        EnumerateDevicesError, GetDisplayMediaError, GetUserMediaError,
        InitLocalTracksError, MediaManager, MediaManagerHandle,
    },
    track::MediaSourceKind,
};

/// [MediaStreamTrack.kind][1] representation.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-kind
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
#[repr(u8)]
pub enum MediaKind {
    /// Audio track.
    #[display(fmt = "audio")]
    Audio = 0,

    /// Video track.
    #[display(fmt = "video")]
    Video = 1,
}

impl From<&TrackConstraints> for MediaKind {
    fn from(media_type: &TrackConstraints) -> Self {
        match media_type {
            TrackConstraints::Audio(_) => Self::Audio,
            TrackConstraints::Video(_) => Self::Video,
        }
    }
}

impl From<i32> for MediaKind {
    fn from(from: i32) -> Self {
        match from {
            0 => Self::Audio,
            1 => Self::Video,
            _ => unreachable!(),
        }
    }
}

impl FromStr for MediaKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "audio" => Ok(Self::Audio),
            "video" => Ok(Self::Video),
            _ => Err(()),
        }
    }
}

impl MediaKind {
    /// Returns string representation of a [`MediaKind`].
    #[inline]
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Audio => "audio",
            Self::Video => "video",
        }
    }
}
