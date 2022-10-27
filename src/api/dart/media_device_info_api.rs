use flutter_rust_bridge::{Opaque, SyncReturn};

#[cfg(feature = "mockable")]
pub use self::mock::MediaDeviceInfo;
#[cfg(not(feature = "mockable"))]
pub use crate::platform::MediaDeviceInfo;

/// Returns unique identifier of the represented device.
pub fn media_device_info_device_id(
    media_device: Opaque<MediaDeviceInfo>,
) -> SyncReturn<String> {
    SyncReturn(media_device.device_id())
}

/// Returns kind of the represented device.
///
/// This representation of [MediaDeviceInfo][1] ONLY for input device.
///
/// [1]: https://w3.org/TR/mediacapture-streams/#device-info
pub fn media_device_info_kind(
    media_device: Opaque<MediaDeviceInfo>,
) -> SyncReturn<u8> {
    SyncReturn(media_device.kind() as u8)
}

/// Returns label describing the represented device (for example "External USB
/// Webcam").
///
/// If the device has no associated label, then returns an empty string.
pub fn media_device_info_label(
    media_device: Opaque<MediaDeviceInfo>,
) -> SyncReturn<String> {
    SyncReturn(media_device.label())
}

/// Returns group identifier of the represented device.
///
/// Two devices have the same group identifier if they belong to the same
/// physical device. For example, the audio input and output devices
/// representing the speaker and microphone of the same headset have the
/// same [groupId][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediadeviceinfo-groupid
pub fn media_device_info_group_id(
    media_device: Opaque<MediaDeviceInfo>,
) -> SyncReturn<Option<String>> {
    SyncReturn(media_device.group_id())
}

#[cfg(feature = "mockable")]
mod mock {
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
        pub fn kind(&self) -> MediaDeviceKind {
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
}
