use flutter_rust_bridge::{Opaque, SyncReturn};

#[cfg(feature = "mockable")]
pub use self::mock::MediaDisplayInfo;
#[cfg(not(feature = "mockable"))]
pub use crate::platform::MediaDisplayInfo;

/// Returns a unique identifier of the represented display.
pub fn media_display_info_device_id(
    media_display: Opaque<MediaDisplayInfo>,
) -> SyncReturn<String> {
    SyncReturn(media_display.device_id())
}

/// Returns a title describing the represented display.
pub fn media_display_info_title(
    media_display: Opaque<MediaDisplayInfo>,
) -> SyncReturn<Option<String>> {
    SyncReturn(media_display.title())
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(missing_copy_implementations, clippy::unused_self)]

    #[derive(Debug)]
    pub struct MediaDisplayInfo(pub u8);

    impl MediaDisplayInfo {
        #[must_use]
        pub fn device_id(&self) -> String {
            String::from("device_id")
        }

        #[must_use]
        pub fn title(&self) -> Option<String> {
            Some(String::from("title"))
        }
    }
}
