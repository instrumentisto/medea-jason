#![allow(
    clippy::unused_self,
    clippy::needless_pass_by_value,
    missing_copy_implementations
)]

use crate::{
    api,
    media::{
        track::remote::Track as CoreRemoteMediaTrack, MediaDirection,
        MediaKind, MediaSourceKind,
    },
    platform,
};

#[derive(Clone, Debug)]
pub struct RemoteMediaTrack(pub u8);

impl From<CoreRemoteMediaTrack> for RemoteMediaTrack {
    fn from(_: CoreRemoteMediaTrack) -> Self {
        Self(0)
    }
}

impl RemoteMediaTrack {
    #[must_use]
    pub const fn enabled(&self) -> bool {
        true
    }

    #[must_use]
    pub const fn kind(&self) -> MediaKind {
        MediaKind::Video
    }

    #[must_use]
    pub const fn media_source_kind(&self) -> MediaSourceKind {
        MediaSourceKind::Device
    }

    #[must_use]
    pub const fn muted(&self) -> bool {
        false
    }

    pub fn on_enabled(&self, cb: platform::Function<()>) {
        cb.call0();
    }

    pub fn on_disabled(&self, cb: platform::Function<()>) {
        cb.call0();
    }

    pub fn on_muted(&self, cb: platform::Function<()>) {
        cb.call0();
    }

    pub fn on_unmuted(&self, cb: platform::Function<()>) {
        cb.call0();
    }

    pub fn on_stopped(&self, cb: platform::Function<()>) {
        cb.call0();
    }

    #[allow(unused_qualifications)]
    pub fn on_media_direction_changed(
        &self,
        cb: platform::Function<api::MediaDirection>,
    ) {
        cb.call1(api::MediaDirection::SendRecv);
    }

    #[must_use]
    pub const fn media_direction(&self) -> MediaDirection {
        MediaDirection::SendRecv
    }

    #[must_use]
    pub fn get_track(&self) -> platform::MediaStreamTrack {
        unreachable!()
    }
}
