#![allow(missing_copy_implementations, clippy::unused_self)]

use crate::media::MediaDeviceKind;

#[derive(Debug)]
pub struct MediaDeviceInfo(pub u8);

impl MediaDeviceInfo {
    #[must_use]
    pub fn device_id(&self) -> String {
        String::from("MediaDeviceInfo.device_id")
    }

    #[must_use]
    pub const fn kind(&self) -> MediaDeviceKind {
        MediaDeviceKind::AudioInput
    }

    #[must_use]
    pub fn label(&self) -> String {
        String::from("MediaDeviceInfo.label")
    }

    #[must_use]
    pub fn group_id(&self) -> Option<String> {
        Some(String::from("MediaDeviceInfo.group_id"))
    }
}
