//! Adapters to [Media Capture and Streams API][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams

pub mod constraints;
mod manager;
pub mod track;

use derive_more::with_trait::Display;
use medea_client_api_proto::MediaType;

#[doc(inline)]
pub use self::{
    constraints::{
        AudioSource, DeviceAudioTrackConstraints, DeviceVideoTrackConstraints,
        DisplayAudioTrackConstraints, DisplayVideoTrackConstraints, FacingMode,
        LocalTracksConstraints, MediaStreamSettings, MediaTrackConstraints,
        MultiSourceTracksConstraints, NoiseSuppressionLevel, RecvConstraints,
        TrackConstraints, VideoSource,
    },
    manager::{
        EnumerateDevicesError, EnumerateDisplaysError, GetDisplayMediaError,
        GetUserMediaError, HandleDetachedError, InitLocalTracksError,
        InvalidOutputAudioDeviceIdError, MediaManager, MediaManagerHandleImpl,
        MicVolumeError,
    },
    track::{
        AudioLevelError, AudioProcessingError, MediaSourceKind,
        MediaStreamTrackState, remote::MediaDirection,
    },
};

/// [MediaStreamTrack.kind][1] representation.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-kind
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
#[repr(u8)]
pub enum MediaKind {
    /// Audio track.
    #[display("audio")]
    Audio = 0,

    /// Video track.
    #[display("video")]
    Video = 1,
}

impl MediaKind {
    /// Returns string representation of a [`MediaKind`].
    #[must_use]
    pub const fn as_str(self) -> &'static str {
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

impl From<&MediaType> for MediaKind {
    fn from(media_type: &MediaType) -> Self {
        match media_type {
            MediaType::Audio(_) => Self::Audio,
            MediaType::Video(_) => Self::Video,
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

/// Audio device kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum AudioDeviceKind {
    /// Built-in earpiece speaker.
    EarSpeaker = 0,

    /// Built-in loudspeaker.
    Speakerphone = 1,

    /// Wired headphones without microphone.
    WiredHeadphones = 2,

    /// Wired headset with a microphone.
    WiredHeadset = 3,

    /// USB headphones without microphone.
    UsbHeadphones = 4,

    /// USB headset with a microphone.
    UsbHeadset = 5,

    /// Bluetooth headphones profile (A2DP/BLE speaker).
    BluetoothHeadphones = 6,

    /// Bluetooth headset profile suitable for calls (SCO/BLE headset).
    BluetoothHeadset = 7,
}

impl TryFrom<i64> for AudioDeviceKind {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::EarSpeaker,
            1 => Self::Speakerphone,
            2 => Self::WiredHeadphones,
            3 => Self::WiredHeadset,
            4 => Self::UsbHeadphones,
            5 => Self::UsbHeadset,
            6 => Self::BluetoothHeadphones,
            7 => Self::BluetoothHeadset,
            _ => return Err(()),
        })
    }
}
