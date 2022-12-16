#![allow(clippy::unused_self, missing_copy_implementations)]

use crate::{
    media::{
        track::local::LocalMediaTrack as CoreLocalMediaTrack, MediaKind,
        MediaSourceKind,
    },
    platform,
};

#[derive(Debug)]
pub struct LocalMediaTrack(pub u8);

impl From<CoreLocalMediaTrack> for LocalMediaTrack {
    fn from(_: CoreLocalMediaTrack) -> Self {
        Self(0)
    }
}

impl LocalMediaTrack {
    #[must_use]
    pub const fn kind(&self) -> MediaKind {
        MediaKind::Video
    }

    #[must_use]
    pub const fn media_source_kind(&self) -> MediaSourceKind {
        MediaSourceKind::Display
    }

    #[must_use]
    pub fn get_track(&self) -> platform::MediaStreamTrack {
        unreachable!()
    }
}
