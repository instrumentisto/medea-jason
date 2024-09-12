//! Wrapper around [MediaStreamTrack][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack

use std::{cell::RefCell, future::Future, rc::Rc};

use derive_more::AsRef;
use futures::future;

use crate::{
    media::{
        track::MediaStreamTrackState, FacingMode, MediaKind, MediaSourceKind,
    },
    platform::wasm::utils::EventListener,
};

/// Wrapper around [MediaStreamTrack][1] received from a
/// [getUserMedia()][2]/[getDisplayMedia()][3] request.
///
/// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
/// [2]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [3]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
#[derive(AsRef, Debug)]
pub struct MediaStreamTrack {
    /// Underlying [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[as_ref(forward)]
    sys_track: Rc<web_sys::MediaStreamTrack>,

    /// Kind of the underlying [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    kind: MediaKind,

    /// Media source kind of this [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    source_kind: Option<MediaSourceKind>,

    /// Listener for an [ended][1] event.
    ///
    /// [1]: https://tinyurl.com/w3-streams#event-mediastreamtrack-ended
    on_ended: RefCell<
        Option<EventListener<web_sys::MediaStreamTrack, web_sys::Event>>,
    >,
}

impl MediaStreamTrack {
    /// Creates a new [`MediaStreamTrack`].
    #[must_use]
    pub fn new<T>(sys_track: T, source_kind: Option<MediaSourceKind>) -> Self
    where
        web_sys::MediaStreamTrack: From<T>,
    {
        let sys_track = web_sys::MediaStreamTrack::from(sys_track);
        let kind = match sys_track.kind().as_ref() {
            "audio" => MediaKind::Audio,
            "video" => MediaKind::Video,
            _ => unreachable!(),
        };
        Self {
            sys_track: Rc::new(sys_track),
            source_kind,
            kind,
            on_ended: RefCell::new(None),
        }
    }

    /// Returns [`id`] of the underlying [MediaStreamTrack][2].
    ///
    /// [`id`]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-id
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn id(&self) -> String {
        self.sys_track.id()
    }

    /// Returns this [`MediaStreamTrack`]'s kind (audio/video).
    #[must_use]
    pub const fn kind(&self) -> MediaKind {
        self.kind
    }

    /// Returns [MediaStreamTrackState][1] of the underlying
    /// [MediaStreamTrack][2].
    ///
    /// # Panics
    ///
    /// If [`readyState`][3] property of underlying [MediaStreamTrack][2] is
    /// neither `live` nor `ended`.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrackstate
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    /// [3]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-readystate
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn ready_state(&self) -> MediaStreamTrackState {
        let state = self.sys_track.ready_state();
        match state {
            web_sys::MediaStreamTrackState::Live => MediaStreamTrackState::Live,
            web_sys::MediaStreamTrackState::Ended => {
                MediaStreamTrackState::Ended
            }
            _ => {
                unreachable!("unknown `MediaStreamTrackState`: {state:?}")
            }
        }
    }

    /// Returns a [`deviceId`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// # Panics
    ///
    /// If the underlying [MediaStreamTrack][2] doesn't have [`deviceId`][1].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-deviceid
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn device_id(&self) -> Option<String> {
        self.sys_track.get_settings().get_device_id()
    }

    /// Return a [`facingMode`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-facingmode
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn facing_mode(&self) -> Option<FacingMode> {
        let facing_mode = self.sys_track.get_settings().get_facing_mode();
        facing_mode.and_then(|fm| match fm.as_ref() {
            "user" => Some(FacingMode::User),
            "environment" => Some(FacingMode::Environment),
            "left" => Some(FacingMode::Left),
            "right" => Some(FacingMode::Right),
            _ => {
                log::error!("Unknown FacingMode: {fm}");
                None
            }
        })
    }

    /// Returns a [`height`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-height
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn height(&self) -> Option<u32> {
        self.sys_track
            .get_settings()
            .get_height()
            .and_then(|w| w.try_into().ok())
    }

    /// Return a [`width`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediatracksettings-width
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn width(&self) -> Option<u32> {
        self.sys_track
            .get_settings()
            .get_width()
            .and_then(|w| w.try_into().ok())
    }

    /// Changes an [`enabled`][1] attribute in the underlying
    /// [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    pub fn set_enabled(&self, enabled: bool) {
        self.sys_track.set_enabled(enabled);
    }

    /// Changes a [`readyState`][1] attribute in the underlying
    /// [MediaStreamTrack][2] to [`ended`][3].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-readystate
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    /// [3]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
    pub fn stop(&self) -> impl Future<Output = ()> + 'static {
        self.sys_track.stop();
        // For platform code uniformity.
        future::ready(())
    }

    /// Returns an [`enabled`][1] attribute of the underlying
    /// [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn enabled(&self) -> bool {
        self.sys_track.enabled()
    }

    /// Detects whether a video track captured from display searching
    /// [specific fields][1] in its settings.
    ///
    /// Only works in Chrome browser at the moment.
    ///
    /// [1]: https://w3.org/TR/screen-capture/#extensions-to-mediatracksettings
    #[must_use]
    pub fn guess_is_from_display(&self) -> bool {
        self.source_kind == Some(MediaSourceKind::Display)
    }

    /// Forks this [`MediaStreamTrack`].
    ///
    /// Creates a new [`MediaStreamTrack`] from this [`MediaStreamTrack`] using
    /// a [`clone()`][1] method. It won't clone current [`MediaStreamTrack`]'s
    /// callbacks.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-clone
    pub fn fork(&self) -> impl Future<Output = Self> + 'static {
        future::ready(Self {
            sys_track: Rc::new(web_sys::MediaStreamTrack::clone(
                &self.sys_track,
            )),
            kind: self.kind,
            source_kind: self.source_kind,
            on_ended: RefCell::new(None),
        })
    }

    /// Sets handler for the [`ended`][1] event on underlying
    /// [`web_sys::MediaStreamTrack`].
    ///
    /// # Panics
    ///
    /// If binding to the [`ended`][1] event fails. Not supposed to ever happen.
    ///
    /// [1]: https://tinyurl.com/w3-streams#event-mediastreamtrack-ended
    pub fn on_ended<F>(&self, f: Option<F>)
    where
        F: 'static + FnOnce(),
    {
        let mut on_ended = self.on_ended.borrow_mut();
        drop(match f {
            None => on_ended.take(),
            Some(f) => on_ended.replace(
                #[expect(clippy::unwrap_used, reason = "shouldn't error ever")]
                EventListener::new_once(
                    Rc::clone(&self.sys_track),
                    "ended",
                    move |_| {
                        f();
                    },
                )
                .unwrap(),
            ),
        });
    }

    /// Indicates whether an `OnAudioLevelChangedCallback` is supported for this
    /// [`MediaStreamTrack`].
    #[must_use]
    pub const fn is_on_audio_level_available(&self) -> bool {
        false
    }

    /// Sets the provided `OnAudioLevelChangedCallback` for this
    /// [`MediaStreamTrack`].
    ///
    /// It's called for live [`MediaStreamTrack`]s when their audio level
    /// changes.
    pub fn on_audio_level_changed<F>(&self, _callback: F)
    where
        F: 'static + FnMut(i32),
    {
    }
}
