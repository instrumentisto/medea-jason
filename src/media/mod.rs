//! Adapters to [Media Capture and Streams API][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams

pub mod constraints;
mod manager;
pub mod track;

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
        GetUserMediaErrorKind, HandleDetachedError, InitLocalTracksError,
        InvalidOutputAudioDeviceIdError, MediaManager, MediaManagerHandle,
        MicrophoneVolumeError, MicrophoneVolumeIsAvailableError,
        SetMicrophoneVolumeError,
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

impl MediaKind {
    /// Returns string representation of a [`MediaKind`].
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Audio => "audio",
            Self::Video => "video",
        }
    }
}

impl From<&TrackConstraints> for MediaKind {
    fn from(media_type: &TrackConstraints) -> Self {
        match media_type {
            TrackConstraints::Audio(_) => Self::Audio,
            TrackConstraints::Video(_) => Self::Video,
        }
    }
}

/// [MediaDeviceInfo.kind][1] representation.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-kind
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum MediaDeviceKind {
    /// Audio input device (for example, a microphone).
    AudioInput = 0,

    /// Video input device (for example, a webcam).
    VideoInput = 1,

    /// Audio output device (for example, a pair of headphones).
    AudioOutput = 2,
}
