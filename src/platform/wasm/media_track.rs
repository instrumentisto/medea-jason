//! Wrapper around [MediaStreamTrack][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack

use std::{cell::RefCell, rc::Rc};

use derive_more::with_trait::AsRef;
use futures::future;
use wasm_bindgen_futures::JsFuture;

use crate::{
    media::{
        FacingMode, MediaKind, MediaSourceKind, track::MediaStreamTrackState,
    },
    platform,
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
    /// Equals `None` for remote tracks.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    source_kind: Option<MediaSourceKind>,

    /// Listener for an [ended][1] event.
    ///
    /// [1]: https://tinyurl.com/w3-streams#event-mediastreamtrack-ended
    on_ended: RefCell<
        Option<EventListener<web_sys::MediaStreamTrack, web_sys::Event>>,
    >,

    // TODO: forking?
    audio_level_watcher: Rc<RefCell<Option<AudioLevelWatcher>>>,
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
            audio_level_watcher: Rc::new(RefCell::new(None)),
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
    pub fn stop(&self) -> impl Future<Output = ()> + 'static + use<> {
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
    pub fn fork(&self) -> impl Future<Output = Self> + 'static + use<> {
        future::ready(Self {
            sys_track: Rc::new(web_sys::MediaStreamTrack::clone(
                &self.sys_track,
            )),
            kind: self.kind,
            source_kind: self.source_kind,
            on_ended: RefCell::new(None),
            audio_level_watcher: Rc::clone(&self.audio_level_watcher),
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
    pub fn is_on_audio_level_available(&self) -> bool {
        // Only local audio tracks.
        self.kind == MediaKind::Audio && self.source_kind.is_none()
    }

    /// Sets the provided `OnAudioLevelChangedCallback` for this
    /// [`MediaStreamTrack`].
    ///
    /// It's called for live [`MediaStreamTrack`]s when their audio level
    /// changes.
    pub fn on_audio_level_changed<F>(&self, cb: F) -> Result<(), js_sys::Error>
    where
        F: 'static + FnMut(i32),
    {
        let mut watcher = self.audio_level_watcher.borrow_mut();
        if let Some(v) = &*watcher {
            drop(v.cb.replace(Box::new(cb)));
        } else {
            *watcher = Some(AudioLevelWatcher::new(&self.sys_track, cb)?);
        }

        Ok(())
    }
}

struct AudioLevelWatcher {
    /// Callback that audio level updates are provided to.
    cb: Rc<RefCell<Box<dyn FnMut(i32)>>>,

    /// [`web_sys::AudioContext`] holding this [`AudioLevelWatcher`] audio
    /// processing pipeline.
    audio_ctx: web_sys::AudioContext,

    /// [`web_sys::AudioSourceNode`] that wraps the [`MediaStreamTrack`] being
    /// watched.
    src: web_sys::MediaStreamAudioSourceNode,

    /// [`web_sys::AnalyserNode`] that is processing audio data.
    analyzer: web_sys::AnalyserNode,
}

impl AudioLevelWatcher {
    fn new<F>(
        track: &web_sys::MediaStreamTrack,
        cb: F,
    ) -> Result<Self, js_sys::Error>
    where
        F: 'static + FnMut(i32),
    {
        /// [`web_sys::AnalyserNode`] FFT size.
        ///
        /// Must be a power of two in the [32..32768] range.
        const FFT_SIZE: u32 = 128;

        let audio_ctx = web_sys::AudioContext::new()?;
        let cb: Box<dyn FnMut(i32)> = Box::new(cb);
        let cb = Rc::new(RefCell::new(cb));

        let stream = {
            let stream = web_sys::MediaStream::new()?;
            stream.add_track(track);
            stream
        };
        // TODO: Use createMediaStreamTrackSource when available
        let src = audio_ctx.create_media_stream_source(&stream)?;
        let analyzer = audio_ctx.create_analyser()?;
        analyzer.set_fft_size(FFT_SIZE);
        // u32 as usize is safe
        // frequency_bin_count = Half the FFT size.
        let mut audio_buf =
            vec![0u8; usize::try_from(analyzer.frequency_bin_count()).unwrap()];
        let audio_buf_len = audio_buf.len() as i32;

        src.connect_with_audio_node(&analyzer)?;
        platform::spawn({
            let cb = Rc::clone(&cb);
            let analyzer = analyzer.clone();
            let audio_ctx = audio_ctx.clone();
            async move {
                loop {
                    platform::delay_for(std::time::Duration::from_millis(100))
                        .await;
                    if audio_ctx.state() == web_sys::AudioContextState::Closed {
                        break;
                    }

                    analyzer.get_byte_frequency_data(&mut audio_buf);
                    let level: i32 =
                        audio_buf.iter().map(|a| i32::from(*a)).sum::<i32>()
                            / audio_buf_len;
                    cb.borrow_mut()(level);
                }
            }
        });

        Ok(Self { cb, src, analyzer, audio_ctx })
    }
}

impl std::fmt::Debug for AudioLevelWatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioLevelWatcher").finish_non_exhaustive()
    }
}

impl Drop for AudioLevelWatcher {
    fn drop(&mut self) {
        drop(self.src.disconnect());
        drop(self.analyzer.disconnect());
        if let Ok(close) = self.audio_ctx.close() {
            platform::spawn(async {
                drop(JsFuture::from(close).await);
            });
        }
    }
}
