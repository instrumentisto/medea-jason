//! [MediaDeviceInfo][1] related objects.
//!
//! [1]: https://w3.org/TR/mediacapture-streams/#device-info

use derive_more::From;
use web_sys as sys;

use crate::media::MediaDeviceKind;

/// Representation of [MediaDeviceInfo][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams/#device-info
#[derive(Debug, From)]
pub struct MediaDeviceInfo(sys::MediaDeviceInfo);

impl From<sys::MediaDeviceKind> for MediaDeviceKind {
    fn from(value: sys::MediaDeviceKind) -> Self {
        match value {
            sys::MediaDeviceKind::Audioinput => Self::AudioInput,
            sys::MediaDeviceKind::Videoinput => Self::VideoInput,
            sys::MediaDeviceKind::Audiooutput => Self::AudioOutput,
            sys::MediaDeviceKind::__Nonexhaustive => {
                unreachable!("Unknown MediaDeviceKind::{value:?}")
            }
        }
    }
}

impl MediaDeviceInfo {
    /// Returns unique identifier for the represented device.
    #[must_use]
    pub fn device_id(&self) -> String {
        self.0.device_id()
    }

    /// Returns kind of the represented device.
    ///
    /// This representation of [MediaDeviceInfo][1] ONLY for input device.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#device-info
    #[must_use]
    pub fn kind(&self) -> MediaDeviceKind {
        self.0.kind().into()
    }

    /// Returns label describing the represented device (for example
    /// "External USB Webcam").
    /// If the device has no associated label, then returns an empty string.
    #[must_use]
    pub fn label(&self) -> String {
        self.0.label()
    }

    /// Returns group identifier of the represented device.
    ///
    /// Two devices have the same group identifier if they belong to the same
    /// physical device. For example, the audio input and output devices
    /// representing the speaker and microphone of the same headset have the
    /// same [groupId][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediadeviceinfo-groupid
    #[must_use]
    pub fn group_id(&self) -> Option<String> {
        Some(self.0.group_id())
    }
}
