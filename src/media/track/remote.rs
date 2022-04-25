//! Wrapper around a received remote [`platform::MediaStreamTrack`].

use std::{cell::Cell, rc::Rc};

use futures::StreamExt as _;
use medea_client_api_proto as proto;
use medea_reactive::ObservableCell;

use crate::{
    api,
    media::{track::MediaStreamTrackState, MediaKind, MediaSourceKind},
    platform,
};

/// Inner reference-counted data of a [`Track`].
#[derive(Debug)]
struct Inner {
    /// Underlying platform-specific [`platform::MediaStreamTrack`].
    track: platform::MediaStreamTrack,

    /// Underlying [`platform::MediaStreamTrack`] source kind.
    media_source_kind: proto::MediaSourceKind,

    /// Callback to be invoked when this [`Track`] is muted.
    on_muted: platform::Callback<()>,

    /// Callback to be invoked when this [`Track`] is unmuted.
    on_unmuted: platform::Callback<()>,

    /// Callback to be invoked when this [`Track`] is stopped.
    on_stopped: platform::Callback<()>,

    /// Callback to be invoked when this [`Track`]'s general media exchange
    /// direction is changed.
    #[allow(unused_qualifications)]
    on_media_direction_changed: platform::Callback<api::MediaDirection>,

    /// Current [`MediaDirection`] of this [`Track`].
    media_direction: Cell<MediaDirection>,

    /// Indicates whether this track is muted.
    ///
    /// Updating this value fires `on_muted` or `on_unmuted` callback and
    /// changes [`muted`][1] property of the underlying
    /// [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-muted
    /// [2]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
    muted: ObservableCell<bool>,
}

/// Wrapper around a received remote [MediaStreamTrack][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
#[derive(Clone, Debug)]
pub struct Track(Rc<Inner>);

impl Track {
    /// Creates a new [`Track`] spawning a listener for its [`enabled`][1] and
    /// [`muted`][2] properties changes.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    /// [2]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-muted
    #[allow(clippy::mut_mut)]
    #[must_use]
    pub fn new<T>(
        track: T,
        media_source_kind: proto::MediaSourceKind,
        muted: bool,
        media_direction: MediaDirection,
    ) -> Self
    where
        platform::MediaStreamTrack: From<T>,
    {
        let track = platform::MediaStreamTrack::from(track);
        let track = Self(Rc::new(Inner {
            track,
            media_source_kind,
            muted: ObservableCell::new(muted),
            on_media_direction_changed: platform::Callback::default(),
            media_direction: Cell::new(media_direction),
            on_stopped: platform::Callback::default(),
            on_muted: platform::Callback::default(),
            on_unmuted: platform::Callback::default(),
        }));

        track.0.track.on_ended({
            let weak_inner = Rc::downgrade(&track.0);
            Some(move || {
                if let Some(inner) = weak_inner.upgrade() {
                    inner.on_stopped.call0();
                }
            })
        });

        let mut muted_changes = track.0.muted.subscribe().skip(1).fuse();
        platform::spawn({
            let weak_inner = Rc::downgrade(&track.0);
            async move {
                while let Some(is_muted) = muted_changes.next().await {
                    if let Some(inner) = weak_inner.upgrade() {
                        if is_muted {
                            inner.on_muted.call0();
                        } else {
                            inner.on_unmuted.call0();
                        }
                    }
                }
            }
        });

        track
    }

    /// Sets general media exchange direction of this [`Track`].
    pub fn set_media_direction(&self, direction: MediaDirection) {
        self.0.media_direction.set(direction);
        self.0.on_media_direction_changed.call1(direction);
    }

    /// Sets `muted` property on this [`Track`].
    ///
    /// Calls `on_muted` or `on_unmuted` callback respectively.
    ///
    /// Updates [`muted`][1] property in the underlying
    /// [`platform::MediaStreamTrack`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-muted
    pub fn set_muted(&self, muted: bool) {
        self.0.muted.set(muted);
    }

    /// Returns [`id`][1] of the underlying [`platform::MediaStreamTrack`] of
    /// this [`Track`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-id
    #[must_use]
    pub fn id(&self) -> String {
        self.0.track.id()
    }

    /// Returns this [`Track`]'s kind (audio/video).
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        self.0.track.kind()
    }

    /// Returns this [`Track`]'s media source kind.
    #[must_use]
    pub fn media_source_kind(&self) -> MediaSourceKind {
        self.0.media_source_kind.into()
    }

    /// Stops this [`Track`] invoking an `on_stopped` callback if it's in a
    /// [`MediaStreamTrackState::Live`] state.
    pub fn stop(self) {
        if self.0.track.ready_state() == MediaStreamTrackState::Live {
            self.0.on_stopped.call0();
        }
    }

    /// Returns the underlying [`platform::MediaStreamTrack`] of this [`Track`].
    #[must_use]
    pub fn get_track(&self) -> &platform::MediaStreamTrack {
        &self.0.track
    }

    /// Indicate whether this [`Track`] is muted.
    #[must_use]
    pub fn muted(&self) -> bool {
        self.0.muted.get()
    }

    /// Sets callback to invoke when this [`Track`] is muted.
    pub fn on_muted(&self, callback: platform::Function<()>) {
        self.0.on_muted.set_func(callback);
    }

    /// Sets callback to invoke when this [`Track`] is unmuted.
    pub fn on_unmuted(&self, callback: platform::Function<()>) {
        self.0.on_unmuted.set_func(callback);
    }

    /// Sets callback to invoke when this [`Track`] is stopped.
    pub fn on_stopped(&self, callback: platform::Function<()>) {
        self.0.on_stopped.set_func(callback);
    }

    /// Sets callback to invoke when this [`Track`]'s general media exchange
    /// direction is changed.
    #[allow(unused_qualifications)]
    pub fn on_media_direction_changed(
        &self,
        callback: platform::Function<api::MediaDirection>,
    ) {
        self.0.on_media_direction_changed.set_func(callback);
    }

    /// Returns current general media exchange direction of this [`Track`].
    #[must_use]
    pub fn media_direction(&self) -> MediaDirection {
        self.0.media_direction.get()
    }
}

/// Media exchange direction of a [`platform::MediaStreamTrack`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum MediaDirection {
    /// `Track` is enabled on recv and send sides.
    SendRecv,

    /// `Track` is enabled on send side.
    SendOnly,

    /// `Track` is enabled on recv side.
    RecvOnly,

    /// `Track` is disabled on both sides.
    Inactive,
}

impl From<MediaDirection> for proto::MediaDirection {
    fn from(val: MediaDirection) -> Self {
        match val {
            MediaDirection::SendRecv => Self::SendRecv,
            MediaDirection::SendOnly => Self::SendOnly,
            MediaDirection::RecvOnly => Self::RecvOnly,
            MediaDirection::Inactive => Self::Inactive,
        }
    }
}

impl From<proto::MediaDirection> for MediaDirection {
    fn from(val: proto::MediaDirection) -> Self {
        match val {
            proto::MediaDirection::SendRecv => Self::SendRecv,
            proto::MediaDirection::SendOnly => Self::SendOnly,
            proto::MediaDirection::RecvOnly => Self::RecvOnly,
            proto::MediaDirection::Inactive => Self::Inactive,
        }
    }
}
