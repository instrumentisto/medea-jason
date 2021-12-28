//! [MediaDeviceInfo][1] related objects.
//!
//! [1]: https://w3.org/TR/mediacapture-streams/#device-info

use derive_more::Display;
use web_sys as sys;

use crate::media::MediaKind;

/// Errors that may occur when parsing [MediaDeviceInfo][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams/#device-info
#[derive(Clone, Copy, Debug, Display)]
pub enum Error {
    /// Occurs when kind of media device is not an input device.
    #[display(fmt = "Not an input device")]
    NotInputDevice,
}

/// Representation of [MediaDeviceInfo][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams/#device-info
#[derive(Debug)]
pub struct InputDeviceInfo {
    /// Kind of the represented media device.
    media_kind: MediaKind,

    /// Actual underlying [MediaDeviceInfo][1] object.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#device-info
    info: sys::MediaDeviceInfo,
}

impl TryFrom<sys::MediaDeviceKind> for MediaKind {
    type Error = Error;

    fn try_from(value: sys::MediaDeviceKind) -> Result<Self, Self::Error> {
        // False positive on `#[non_exhaustive]`.
        #[allow(clippy::wildcard_enum_match_arm)]
        match value {
            sys::MediaDeviceKind::Audioinput => Ok(Self::Audio),
            sys::MediaDeviceKind::Videoinput => Ok(Self::Video),
            _ => Err(Error::NotInputDevice),
        }
    }
}

impl InputDeviceInfo {
    /// Returns unique identifier for the represented device.
    #[must_use]
    pub fn device_id(&self) -> String {
        self.info.device_id()
    }

    /// Returns kind of the represented device.
    ///
    /// This representation of [MediaDeviceInfo][1] ONLY for input device.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams/#device-info
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        self.media_kind
    }

    /// Returns label describing the represented device (for example
    /// "External USB Webcam").
    /// If the device has no associated label, then returns an empty string.
    #[must_use]
    pub fn label(&self) -> String {
        self.info.label()
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
        Some(self.info.group_id())
    }
}

impl TryFrom<sys::MediaDeviceInfo> for InputDeviceInfo {
    type Error = Error;

    fn try_from(info: sys::MediaDeviceInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            media_kind: MediaKind::try_from(info.kind())?,
            info,
        })
    }
}
