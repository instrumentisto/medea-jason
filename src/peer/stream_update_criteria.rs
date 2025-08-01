//! [`MediaKind`] + [`MediaSourceKind`] criteria for local stream updates.

use std::ops::BitOrAssign as _;

use medea_client_api_proto::{Direction, MediaSourceKind, MediaType, Track};

use crate::media::MediaKind;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Inner: u8 {
        const DEVICE_AUDIO =  0b0001;
        const DISPLAY_AUDIO = 0b0010;
        const DEVICE_VIDEO =  0b0100;
        const DISPLAY_VIDEO = 0b1000;
    }
}

/// Criteria, used for local stream updates, allowing to specify a set of
/// [`MediaKind`] + [`MediaSourceKind`] pairs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalStreamUpdateCriteria(Inner);

impl LocalStreamUpdateCriteria {
    /// Creates [`LocalStreamUpdateCriteria`] with all possible [`MediaKind`] +
    /// [`MediaSourceKind`] combinations.
    #[must_use]
    pub const fn all() -> Self {
        Self(Inner::all())
    }

    /// Creates empty [`LocalStreamUpdateCriteria`].
    #[must_use]
    pub const fn empty() -> Self {
        Self(Inner::empty())
    }

    /// Creates [`LocalStreamUpdateCriteria`] with the provided [`MediaKind`] +
    /// [`MediaSourceKind`] pair.
    ///
    /// [`None`] `source_kind` means both
    /// [`MediaSourceKind`]s.
    #[must_use]
    pub fn from_kinds(
        media_kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
    ) -> Self {
        use MediaKind as MK;
        use MediaSourceKind as SK;

        let inner = match (source_kind, media_kind) {
            (None, MK::Audio) => Inner::DEVICE_AUDIO | Inner::DISPLAY_AUDIO,
            (Some(SK::Device), MK::Audio) => Inner::DEVICE_AUDIO,
            (Some(SK::Display), MK::Audio) => Inner::DISPLAY_AUDIO,
            (None, MK::Video) => Inner::DEVICE_VIDEO | Inner::DISPLAY_VIDEO,
            (Some(SK::Device), MK::Video) => Inner::DEVICE_VIDEO,
            (Some(SK::Display), MK::Video) => Inner::DISPLAY_VIDEO,
        };
        Self(inner)
    }

    /// Builds [`LocalStreamUpdateCriteria`] from the provided `tracks`. Only
    /// [`Direction::Send`] [`Track`]s are taken into account.
    #[must_use]
    pub fn from_tracks(tracks: &[Track]) -> Self {
        let mut result = Self::empty();
        for track in tracks
            .iter()
            .filter(|t| matches!(t.direction, Direction::Send { .. }))
        {
            match &track.media_type {
                MediaType::Audio(audio) => {
                    result.add(MediaKind::Audio, audio.source_kind);
                }
                MediaType::Video(video) => {
                    result.add(MediaKind::Video, video.source_kind);
                }
            }
        }
        result
    }

    /// Adds the given [`MediaKind`] + [`MediaSourceKind`] pair to this
    /// [`LocalStreamUpdateCriteria`].
    pub fn add(&mut self, media_kind: MediaKind, source_kind: MediaSourceKind) {
        self.0.bitor_assign(Self::from_kinds(media_kind, Some(source_kind)).0);
    }

    /// Checks whether this [`LocalStreamUpdateCriteria`] contains the provided
    /// [`MediaKind`] + [`MediaSourceKind`] pair.
    #[must_use]
    pub fn has(
        self,
        media_kind: MediaKind,
        source_kind: MediaSourceKind,
    ) -> bool {
        self.0.contains(Self::from_kinds(media_kind, Some(source_kind)).0)
    }
}
