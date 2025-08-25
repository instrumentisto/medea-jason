//! Wrapper around [MediaStreamTrack][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack

use std::{cell::RefCell, rc::Rc, time::Duration};

use derive_more::{Debug, with_trait::AsRef};
use futures::{StreamExt as _, future, stream::LocalBoxStream};
use js_sys::{Error as JsError, Reflect};
use medea_reactive::ObservableCell;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::{
    media::{
        FacingMode, MediaKind, MediaSourceKind, NoiseSuppressionLevel,
        track::MediaStreamTrackState,
    },
    platform::{self, wasm::utils::EventListener},
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
    /// [`None`] for remote tracks.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    source_kind: Option<MediaSourceKind>,

    /// Listener for an [ended][1] event.
    ///
    /// [1]: https://tinyurl.com/w3-streams#event-mediastreamtrack-ended
    on_ended: RefCell<
        Option<EventListener<web_sys::MediaStreamTrack, web_sys::Event>>,
    >,

    /// Listener of audio level [changes][1] in this [`MediaStreamTrack`] (if
    /// it's a local one).
    ///
    /// [1]: https://tinyurl.com/w3-streams#event-mediastreamtrack-ended
    #[expect(clippy::type_complexity, reason = "not really")]
    #[debug(skip)]
    on_audio_level: Rc<RefCell<Option<Box<dyn FnMut(i32)>>>>,

    /// [`AudioLevelWatcher`] of the underlying [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
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
            on_audio_level: Rc::new(RefCell::new(None)),
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
        let facing_mode = self.sys_track.get_settings().get_facing_mode()?;
        match facing_mode.as_ref() {
            "user" => Some(FacingMode::User),
            "environment" => Some(FacingMode::Environment),
            "left" => Some(FacingMode::Left),
            "right" => Some(FacingMode::Right),
            _ => {
                log::error!("Unknown `FacingMode`: {facing_mode}");
                None
            }
        }
    }

    /// Returns a [`height`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-height
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn height(&self) -> Option<u32> {
        let h = self.sys_track.get_settings().get_height()?;
        h.try_into().ok()
    }

    /// Return a [`width`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediatracksettings-width
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn width(&self) -> Option<u32> {
        let w = self.sys_track.get_settings().get_width()?;
        w.try_into().ok()
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

    /// Returns the media source kind of this [`MediaStreamTrack`].
    ///
    /// Returns [`None`] for remote tracks.
    #[must_use]
    pub const fn source_kind(&self) -> Option<MediaSourceKind> {
        self.source_kind
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
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn fork(&self) -> Self {
        Self {
            sys_track: Rc::new(web_sys::MediaStreamTrack::clone(
                &self.sys_track,
            )),
            kind: self.kind,
            source_kind: self.source_kind,
            on_ended: RefCell::new(None),
            on_audio_level: Rc::new(RefCell::new(None)),
            audio_level_watcher: Rc::clone(&self.audio_level_watcher),
        }
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
        self.kind == MediaKind::Audio && self.source_kind.is_some()
    }

    /// Sets the provided `OnAudioLevelChangedCallback` for this
    /// [`MediaStreamTrack`].
    ///
    /// It's called for live [`MediaStreamTrack`]s when their audio level
    /// changes.
    ///
    /// # Errors
    ///
    /// If platform call errors.
    pub fn on_audio_level_changed<F>(
        &self,
        cb: F,
    ) -> Result<(), platform::Error>
    where
        F: 'static + FnMut(i32),
    {
        if !self.is_on_audio_level_available() {
            return Ok(());
        }

        self.on_audio_level.borrow_mut().replace(Box::new(cb));
        let callback = Rc::clone(&self.on_audio_level);

        let mut sub = {
            let mut audio_level_watcher = self.audio_level_watcher.borrow_mut();
            if let Some(watcher) = audio_level_watcher.as_ref() {
                watcher.subscribe()
            } else {
                let watcher = AudioLevelWatcher::new(&self.sys_track)?;
                let sub = watcher.subscribe();
                *audio_level_watcher = Some(watcher);

                sub
            }
        };

        platform::spawn(async move {
            while let Some(level) = sub.next().await {
                if let Some(callback) = callback.borrow_mut().as_mut() {
                    callback(level);
                } else {
                    break;
                }
            }
        });

        Ok(())
    }

    /// Indicates whether this [`MediaStreamTrack`] supports audio processing
    /// functions:
    /// - [`MediaStreamTrack::is_noise_suppression_enabled()`]
    /// - [`MediaStreamTrack::set_noise_suppression_enabled()`]
    /// - [`MediaStreamTrack::is_echo_cancellation_enabled()`]
    /// - [`MediaStreamTrack::set_echo_cancellation_enabled()`]
    /// - [`MediaStreamTrack::is_auto_gain_control_enabled()`]
    /// - [`MediaStreamTrack::set_auto_gain_control_enabled()`]
    ///
    /// These functions are unavailable and always error:
    /// - [`MediaStreamTrack::get_noise_suppression_level()`]
    /// - [`MediaStreamTrack::set_noise_suppression_level()`]
    /// - [`MediaStreamTrack::is_high_pass_filter_enabled()`]
    /// - [`MediaStreamTrack::set_high_pass_filter_enabled()`]
    pub fn is_audio_processing_available(&self) -> bool {
        if Reflect::get(&self.sys_track, &JsValue::from_str("getCapabilities"))
            .map_or(None, |val| (!val.is_undefined()).then_some(val))
            .is_none()
        {
            return false;
        }
        if Reflect::get(&self.sys_track, &JsValue::from_str("applyConstraints"))
            .map_or(None, |val| (!val.is_undefined()).then_some(val))
            .is_none()
        {
            return false;
        }
        if Reflect::get(&self.sys_track, &JsValue::from_str("getSettings"))
            .map_or(None, |val| (!val.is_undefined()).then_some(val))
            .is_none()
        {
            return false;
        }

        let caps = self.sys_track.get_capabilities();
        caps.get_echo_cancellation().is_some()
            && caps.get_noise_suppression().is_some()
            && caps.get_auto_gain_control().is_some()
    }

    /// Toggles noise suppression for this [`MediaStreamTrack`].
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    pub async fn set_noise_suppression_enabled(
        &self,
        enabled: bool,
    ) -> Result<(), platform::Error> {
        let caps = self.sys_track.get_constraints();
        caps.set_noise_suppression(&JsValue::from(enabled));

        let fut = self
            .sys_track
            .apply_constraints_with_constraints(&caps)
            .map_err(platform::Error::from)?;
        JsFuture::from(fut).await.map_err(platform::Error::from)?;

        if self.is_noise_suppression_enabled().await? != enabled {
            return Err(JsError::new("not supported").into());
        }

        Ok(())
    }

    /// Toggles auto gain control for this [`MediaStreamTrack`].
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    pub async fn set_auto_gain_control_enabled(
        &self,
        enabled: bool,
    ) -> Result<(), platform::Error> {
        let caps = self.sys_track.get_constraints();
        caps.set_auto_gain_control(&JsValue::from(enabled));

        let fut = self
            .sys_track
            .apply_constraints_with_constraints(&caps)
            .map_err(platform::Error::from)?;
        JsFuture::from(fut).await.map_err(platform::Error::from)?;

        // This might not have worked, so we are checking to make sure
        if self.is_auto_gain_control_enabled().await? != enabled {
            return Err(JsError::new("not supported").into());
        }

        Ok(())
    }

    /// Toggles acoustic echo cancellation for this [`MediaStreamTrack`].
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    pub async fn set_echo_cancellation_enabled(
        &self,
        enabled: bool,
    ) -> Result<(), platform::Error> {
        let caps = self.sys_track.get_constraints();
        caps.set_echo_cancellation(&JsValue::from(enabled));

        let fut = self
            .sys_track
            .apply_constraints_with_constraints(&caps)
            .map_err(platform::Error::from)?;
        JsFuture::from(fut).await.map_err(platform::Error::from)?;

        // This might not have worked, so we are checking to make sure
        if self.is_echo_cancellation_enabled().await? != enabled {
            return Err(JsError::new("not supported").into());
        }

        Ok(())
    }

    /// Configures a [`NoiseSuppressionLevel`] for this [`MediaStreamTrack`].
    ///
    /// __NOTE__: Does nothing, as is not supported.
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn set_noise_suppression_level(
        &self,
        _: NoiseSuppressionLevel,
    ) -> Result<(), platform::Error> {
        log::error!("Changing noise suppression level is not available on web");

        Ok(())
    }

    /// Toggles high-pass filter for this [`MediaStreamTrack`].
    ///
    /// __NOTE__: Does nothing, as is not supported.
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn set_high_pass_filter_enabled(
        &self,
        _: bool,
    ) -> Result<(), platform::Error> {
        log::error!("Changing high-pass filter is not available on web");

        Ok(())
    }

    /// Indicates whether noise suppression is enabled for this
    /// [`MediaStreamTrack`].
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn is_noise_suppression_enabled(
        &self,
    ) -> Result<bool, platform::Error> {
        let settings = self.sys_track.get_settings();

        Ok(settings.get_noise_suppression().unwrap_or_default())
    }

    /// Indicates whether echo cancellation is enabled for this
    /// [`MediaStreamTrack`].
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn is_echo_cancellation_enabled(
        &self,
    ) -> Result<bool, platform::Error> {
        let settings = self.sys_track.get_settings();

        Ok(settings.get_echo_cancellation().unwrap_or_default())
    }

    /// Indicates whether auto gain control is enabled for this
    /// [`MediaStreamTrack`].
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn is_auto_gain_control_enabled(
        &self,
    ) -> Result<bool, platform::Error> {
        let settings = self.sys_track.get_settings();

        Ok(settings.get_auto_gain_control().unwrap_or_default())
    }

    /// Returns the current configured [`NoiseSuppressionLevel`] of this
    /// [`MediaStreamTrack`].
    ///
    /// __NOTE__: Panics, as is not supported.
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    ///
    /// # Panics
    ///
    /// Always panics as [`unimplemented`].
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn get_noise_suppression_level(
        &self,
    ) -> Result<NoiseSuppressionLevel, platform::Error> {
        unimplemented!(
            "getting noise suppression level is not available on web",
        )
    }

    /// Indicates whether high-pass filter is enabled for this
    /// [`MediaStreamTrack`].
    ///
    /// __NOTE__: Panics, as is not supported.
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    ///
    /// # Panics
    ///
    /// Always panics as [`unimplemented`].
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn is_high_pass_filter_enabled(
        &self,
    ) -> Result<bool, platform::Error> {
        unimplemented!("getting high-pass filter is not available on web")
    }
}

/// Analyzer of audio track raw data producing audio level ([RMS] loudness).
///
/// [RMS]: https://en.wikipedia.org/wiki/Root_mean_square
#[derive(Debug)]
struct AudioLevelWatcher {
    /// [`web_sys::AudioContext`] holding this [`AudioLevelWatcher`] audio
    /// processing pipeline.
    audio_ctx: web_sys::AudioContext,

    /// Latest audio level value in the `[0;100]` range.
    level: Rc<ObservableCell<i32>>,

    /// [`web_sys::AudioSourceNode`] wrapping the [`MediaStreamTrack`] being
    /// watched.
    src: web_sys::MediaStreamAudioSourceNode,

    /// [`web_sys::AnalyserNode`] processing audio data.
    analyzer: web_sys::AnalyserNode,
}

impl AudioLevelWatcher {
    /// Creates a new [`AudioLevelWatcher`] for the provided
    /// [`web_sys::MediaStreamTrack`].
    #[expect(clippy::unwrap_in_result, reason = "unrelated and intended")]
    fn new(track: &web_sys::MediaStreamTrack) -> Result<Self, platform::Error> {
        /// [`web_sys::AnalyserNode`] FFT size.
        ///
        /// Must be a power of two in the `[32..32768]` range.
        const FFT_SIZE: u32 = 256;

        let audio_ctx = web_sys::AudioContext::new()?;
        let level = Rc::new(ObservableCell::new(0));

        let stream = {
            let stream = web_sys::MediaStream::new()?;
            stream.add_track(track);
            stream
        };
        // TODO: Use `createMediaStreamTrackSource` once available.
        let src = audio_ctx.create_media_stream_source(&stream)?;
        let analyzer = audio_ctx.create_analyser()?;
        analyzer.set_fft_size(FFT_SIZE);
        #[expect(clippy::unwrap_used, reason = "always in bounds")]
        let mut audio_buf = vec![0.0f32; usize::try_from(FFT_SIZE).unwrap()];

        src.connect_with_audio_node(&analyzer)?;
        platform::spawn({
            let level = Rc::clone(&level);
            let analyzer = analyzer.clone();
            let audio_ctx = audio_ctx.clone();
            async move {
                loop {
                    if audio_ctx.state() == web_sys::AudioContextState::Closed {
                        break;
                    }

                    analyzer.get_float_time_domain_data(&mut audio_buf);
                    let mut sum = 0.0;
                    for b in &audio_buf {
                        sum += b * b;
                    }

                    #[expect( // no better way
                        clippy::as_conversions,
                        clippy::cast_precision_loss,
                        reason = "no better way"
                    )]
                    let lvl = (sum / FFT_SIZE as f32).sqrt() * 1000.0;
                    #[expect( // no better way
                        clippy::as_conversions,
                        clippy::cast_possible_truncation,
                        reason = "no better way"
                    )]
                    level.set(lvl.round().clamp(0.0, 100.0) as i32);

                    // Measure every 50 milliseconds.
                    platform::delay_for(Duration::from_millis(50)).await;
                }
            }
        });

        Ok(Self { audio_ctx, level, src, analyzer })
    }

    /// Subscribes to audio level changes of this [`AudioLevelWatcher`].
    fn subscribe(&self) -> LocalBoxStream<'static, i32> {
        self.level.subscribe()
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
