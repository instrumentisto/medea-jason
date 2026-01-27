//! Representation of a [MediaStreamTrack][0].
//!
//! [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack

use dart_sys::Dart_Handle;
use medea_macro::dart_bridge;

use crate::{
    media::{
        FacingMode, MediaKind, MediaSourceKind, NoiseSuppressionLevel,
        track::MediaStreamTrackState,
    },
    platform::{
        self,
        dart::utils::{
            NonNullDartValueArgExt as _, callback::Callback,
            dart_string_into_rust, handle::DartHandle,
        },
        utils::dart_future::FutureFromDart,
    },
};

#[dart_bridge("flutter/lib/src/native/platform/media_track.g.dart")]
mod media_stream_track {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::{api::DartValueArg, platform::Error};

    extern "C" {
        /// Returns [ID][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-id
        pub fn id(track: Dart_Handle) -> Result<ptr::NonNull<c_char>, Error>;

        /// Returns [device ID][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://w3.org/TR/mediacapture-streams#dfn-deviceid
        pub fn device_id(
            track: Dart_Handle,
        ) -> Result<ptr::NonNull<c_char>, Error>;

        /// Returns [kind][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-kind
        pub fn kind(track: Dart_Handle) -> Result<i64, Error>;

        /// Returns [facing mode][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#def-constraint-facingMode
        pub fn facing_mode(
            track: Dart_Handle,
        ) -> Result<ptr::NonNull<DartValueArg<Option<i64>>>, Error>;

        /// Returns [height][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-height
        pub fn height(
            track: Dart_Handle,
        ) -> Result<ptr::NonNull<DartValueArg<Option<u32>>>, Error>;

        /// Returns [width][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-width
        pub fn width(
            track: Dart_Handle,
        ) -> Result<ptr::NonNull<DartValueArg<Option<u32>>>, Error>;

        /// Returns [enabled][1] field of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-enabled
        pub fn enabled(track: Dart_Handle) -> Result<bool, Error>;

        /// Sets [enabled][1] field of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-enabled
        pub fn set_enabled(
            track: Dart_Handle,
            is_enabled: bool,
        ) -> Result<(), Error>;

        /// Returns [readiness state][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-readystate
        pub fn ready_state(track: Dart_Handle) -> Result<Dart_Handle, Error>;

        /// [Stops][1] the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-stop
        pub fn stop(track: Dart_Handle) -> Result<Dart_Handle, Error>;

        /// Sets [`onended`][1] event handler of the provided
        /// [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-onended
        pub fn on_ended(
            track: Dart_Handle,
            cb: Dart_Handle,
        ) -> Result<(), Error>;

        /// Creates a new instance of [MediaStreamTrack][0] depending on the
        /// same media source as the provided one has.
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn clone(track: Dart_Handle) -> Result<Dart_Handle, Error>;

        /// Disposes the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn dispose(track: Dart_Handle) -> Result<Dart_Handle, Error>;

        /// Indicates whether an `OnAudioLevelChangedCallback` is supported for
        /// this [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn is_on_audio_level_available(
            track: Dart_Handle,
        ) -> Result<bool, Error>;

        /// Sets the provided `OnAudioLevelChangedCallback` for the provided
        /// [MediaStreamTrack][0].
        ///
        /// It's called for live [MediaStreamTrack][0]s when their audio level
        /// changes.
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn on_audio_level_changed(
            track: Dart_Handle,
            cb: Dart_Handle,
        ) -> Result<(), Error>;

        /// Indicates whether the provided [MediaStreamTrack][0] supports audio
        /// processing functions.
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn is_audio_processing_available(
            track: Dart_Handle,
        ) -> Result<bool, Error>;

        /// Toggles noise suppression for the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn set_noise_suppression_enabled(
            track: Dart_Handle,
            enable: bool,
        ) -> Result<Dart_Handle, Error>;

        /// Configures a noise suppression level for the provided
        /// [MediaStreamTrack][0].
        ///
        /// __NOTE__: Only supported on desktop platforms.
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn set_noise_suppression_level(
            track: Dart_Handle,
            level: i64,
        ) -> Result<Dart_Handle, Error>;

        /// Toggles echo cancellation for the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn set_echo_cancellation_enabled(
            track: Dart_Handle,
            enable: bool,
        ) -> Result<Dart_Handle, Error>;

        /// Toggles auto gain control for the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn set_auto_gain_control_enabled(
            track: Dart_Handle,
            enable: bool,
        ) -> Result<Dart_Handle, Error>;

        /// Toggles high-pass filter for the provided [MediaStreamTrack][0].
        ///
        /// __NOTE__: Only supported on desktop platforms.
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn set_high_pass_filter_enabled(
            track: Dart_Handle,
            enable: bool,
        ) -> Result<Dart_Handle, Error>;

        /// Indicates whether noise suppression is enabled for the provided
        /// [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn is_noise_suppression_enabled(
            track: Dart_Handle,
        ) -> Result<Dart_Handle, Error>;

        /// Returns the current configured noise suppression level of the
        /// provided [MediaStreamTrack][0].
        ///
        /// __NOTE__: Only supported on desktop platforms.
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn get_noise_suppression_level(
            track: Dart_Handle,
        ) -> Result<Dart_Handle, Error>;

        /// Indicates whether auto gain control is enabled for the provided
        /// [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn is_auto_gain_control_enabled(
            track: Dart_Handle,
        ) -> Result<Dart_Handle, Error>;

        /// Indicates whether echo cancellation is enabled for the provided
        /// [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn is_echo_cancellation_enabled(
            track: Dart_Handle,
        ) -> Result<Dart_Handle, Error>;

        /// Indicates whether high-pass filter is enabled for the provided
        /// [MediaStreamTrack][0].
        ///
        /// __NOTE__: Only supported on desktop platforms.
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        pub fn is_high_pass_filter_enabled(
            track: Dart_Handle,
        ) -> Result<Dart_Handle, Error>;
    }
}

/// Representation of a [MediaStreamTrack][0] received from a
/// [getUserMedia()][1] or a [getDisplayMedia()][2] request.
///
/// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
#[derive(Clone, Debug)]
pub struct MediaStreamTrack {
    /// Pointer on the [MediaStreamTrack][0].
    ///
    /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    inner: DartHandle,

    /// Media source type of this [`MediaStreamTrack`].
    source_kind: Option<MediaSourceKind>,
}

impl MediaStreamTrack {
    /// Creates a new [`MediaStreamTrack`].
    #[must_use]
    pub const fn new(
        inner: DartHandle,
        source_kind: Option<MediaSourceKind>,
    ) -> Self {
        Self { inner, source_kind }
    }

    /// Returns the underlying [`Dart_Handle`] of this [`MediaStreamTrack`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.inner.get()
    }

    /// Returns [ID][1] of this [`MediaStreamTrack`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-id
    #[must_use]
    pub fn id(&self) -> String {
        let id = unsafe { media_stream_track::id(self.inner.get()) }.unwrap();
        unsafe { dart_string_into_rust(id) }
    }

    /// Returns [device ID][1] of this [`MediaStreamTrack`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dfn-deviceid
    #[inline]
    #[must_use]
    pub fn device_id(&self) -> Option<String> {
        let device_id =
            unsafe { media_stream_track::device_id(self.inner.get()) }.unwrap();
        Some(unsafe { dart_string_into_rust(device_id) })
    }

    /// Returns [kind][1] of this [`MediaStreamTrack`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-kind
    #[inline]
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        MediaKind::try_from(
            unsafe { media_stream_track::kind(self.inner.get()) }.unwrap(),
        )
        .unwrap()
    }

    /// Returns [facing mode][1] of this [`MediaStreamTrack`].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-facingmode
    #[must_use]
    pub fn facing_mode(&self) -> Option<FacingMode> {
        let facing_mode =
            unsafe { media_stream_track::facing_mode(self.inner.get()) }
                .unwrap();
        Option::<i64>::try_from(unsafe { facing_mode.unbox() })
            .unwrap()
            .map(FacingMode::try_from)
            .transpose()
            .unwrap()
    }

    /// Returns [height][1] of this [`MediaStreamTrack`].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-height
    #[must_use]
    pub fn height(&self) -> Option<u32> {
        let height =
            unsafe { media_stream_track::height(self.inner.get()) }.unwrap();
        Option::try_from(unsafe { height.unbox() }).unwrap()
    }

    /// Returns [width][1] of this [`MediaStreamTrack`].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-width
    #[must_use]
    pub fn width(&self) -> Option<u32> {
        let width =
            unsafe { media_stream_track::width(self.inner.get()) }.unwrap();
        Option::try_from(unsafe { width.unbox() }).unwrap()
    }

    /// Returns [enabled][1] field of this [`MediaStreamTrack`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    #[inline]
    #[must_use]
    pub fn enabled(&self) -> bool {
        unsafe { media_stream_track::enabled(self.inner.get()) }.unwrap()
    }

    /// Sets [enabled][1] field of this [`MediaStreamTrack`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    pub fn set_enabled(&self, enabled: bool) {
        unsafe { media_stream_track::set_enabled(self.inner.get(), enabled) }
            .unwrap();
    }

    /// Returns [readiness state][1] of this [`MediaStreamTrack`].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-readystate
    pub async fn ready_state(&self) -> MediaStreamTrackState {
        let handle = self.inner.get();
        let state = unsafe { media_stream_track::ready_state(handle) }.unwrap();
        let state =
            unsafe { FutureFromDart::execute::<i64>(state) }.await.unwrap();

        match state {
            0 => MediaStreamTrackState::Live,
            1 => MediaStreamTrackState::Ended,
            _ => unreachable!("Unknown `MediaStreamTrackState`: {state}"),
        }
    }

    /// [Stops][1] this [`MediaStreamTrack`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-stop
    #[inline]
    pub fn stop(&self) -> impl Future<Output = ()> + 'static + use<> {
        let inner = self.inner.clone();
        async move {
            let fut = unsafe { media_stream_track::stop(inner.get()) }.unwrap();
            unsafe { FutureFromDart::execute::<()>(fut) }.await.unwrap();
        }
    }

    /// Returns the [`MediaSourceKind`] of this [`MediaStreamTrack`].
    ///
    /// [`None`] for remote [`MediaStreamTrack`]s.
    #[must_use]
    pub const fn source_kind(&self) -> Option<MediaSourceKind> {
        self.source_kind
    }

    /// Detects whether this video [`MediaStreamTrack`] is captured from
    /// display, searching for [specific fields][1] in its settings.
    ///
    /// Only works in Chrome browser at the moment.
    ///
    /// [1]: https://w3.org/TR/screen-capture#extensions-to-mediatracksettings
    #[must_use]
    pub fn guess_is_from_display(&self) -> bool {
        self.source_kind == Some(MediaSourceKind::Display)
    }

    /// Forks this [`MediaStreamTrack`], by creating a new [`MediaStreamTrack`]
    /// from this [`MediaStreamTrack`] using a [`clone()`][1] method.
    ///
    /// __NOTE__: It won't clone [`MediaStreamTrack`]'s event handlers.
    ///
    /// # Naming
    ///
    /// The name of this method intentionally diverges from [the spec one][1] to
    /// not interfere with [`Clone`] trait.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-clone
    pub async fn fork(&self) -> Self {
        let fut =
            unsafe { media_stream_track::clone(self.inner.get()) }.unwrap();
        let new_track: DartHandle =
            unsafe { FutureFromDart::execute(fut) }.await.unwrap();
        Self::new(new_track, self.source_kind)
    }

    /// Sets [`onended`][1] event handler of this [`MediaStreamTrack`].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-onended
    pub fn on_ended<F>(&self, f: Option<F>)
    where
        F: 'static + FnOnce(),
    {
        if let Some(cb) = f {
            let cb = Callback::from_once(|(): ()| cb());
            unsafe {
                media_stream_track::on_ended(self.inner.get(), cb.into_dart())
            }
            .unwrap();
        }
    }

    /// Indicates whether an `OnAudioLevelChangedCallback` is supported for this
    /// [`MediaStreamTrack`].
    #[must_use]
    pub fn is_on_audio_level_available(&self) -> bool {
        unsafe {
            media_stream_track::is_on_audio_level_available(self.inner.get())
        }
        .unwrap()
    }

    /// Sets the provided `OnAudioLevelChangedCallback` for this
    /// [`MediaStreamTrack`].
    ///
    /// It's called for live [`MediaStreamTrack`]s when their audio level
    /// changes.
    ///
    /// # Errors
    ///
    /// Never errors.
    #[expect(clippy::unwrap_in_result, reason = "unrelated and intended")]
    pub fn on_audio_level_changed<F>(
        &self,
        mut f: F,
    ) -> Result<(), platform::Error>
    where
        F: 'static + FnMut(i32),
    {
        let cb = Callback::from_fn_mut(move |value: i32| f(value));

        unsafe {
            media_stream_track::on_audio_level_changed(
                self.inner.get(),
                cb.into_dart(),
            )
        }
        .unwrap();

        Ok(())
    }

    /// Indicates whether this [`MediaStreamTrack`] supports audio processing
    /// functions:
    /// - [`MediaStreamTrack::is_noise_suppression_enabled()`]
    /// - [`MediaStreamTrack::set_noise_suppression_enabled()`]
    /// - [`MediaStreamTrack::get_noise_suppression_level()`]
    /// - [`MediaStreamTrack::set_noise_suppression_level()`]
    /// - [`MediaStreamTrack::is_echo_cancellation_enabled()`]
    /// - [`MediaStreamTrack::set_echo_cancellation_enabled()`]
    /// - [`MediaStreamTrack::is_auto_gain_control_enabled()`]
    /// - [`MediaStreamTrack::set_auto_gain_control_enabled()`]
    /// - [`MediaStreamTrack::is_high_pass_filter_enabled()`]
    /// - [`MediaStreamTrack::set_high_pass_filter_enabled()`]
    #[must_use]
    pub fn is_audio_processing_available(&self) -> bool {
        unsafe {
            media_stream_track::is_audio_processing_available(self.inner.get())
        }
        .unwrap()
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
        let fut = unsafe {
            media_stream_track::set_noise_suppression_enabled(
                self.inner.get(),
                enabled,
            )
        }?;
        unsafe { FutureFromDart::execute::<()>(fut) }.await
    }

    /// Configures a [`NoiseSuppressionLevel`] for this [`MediaStreamTrack`].
    ///
    /// __NOTE__: Only supported on desktop platforms.
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    pub async fn set_noise_suppression_level(
        &self,
        level: NoiseSuppressionLevel,
    ) -> Result<(), platform::Error> {
        let fut = unsafe {
            media_stream_track::set_noise_suppression_level(
                self.inner.get(),
                level as i64,
            )
        }?;
        unsafe { FutureFromDart::execute::<()>(fut) }.await
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
        let fut = unsafe {
            media_stream_track::set_echo_cancellation_enabled(
                self.inner.get(),
                enabled,
            )
        }?;
        unsafe { FutureFromDart::execute::<()>(fut) }.await
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
        let fut = unsafe {
            media_stream_track::set_auto_gain_control_enabled(
                self.inner.get(),
                enabled,
            )
        }?;
        unsafe { FutureFromDart::execute::<()>(fut) }.await
    }

    /// Toggles high-pass filter for this [`MediaStreamTrack`].
    ///
    /// __NOTE__: Only supported on desktop platforms.
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    pub async fn set_high_pass_filter_enabled(
        &self,
        enabled: bool,
    ) -> Result<(), platform::Error> {
        let fut = unsafe {
            media_stream_track::set_high_pass_filter_enabled(
                self.inner.get(),
                enabled,
            )
        }?;
        unsafe { FutureFromDart::execute::<()>(fut) }.await
    }

    /// Indicates whether noise suppression is enabled for this
    /// [`MediaStreamTrack`].
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    pub async fn is_noise_suppression_enabled(
        &self,
    ) -> Result<bool, platform::Error> {
        let fut = unsafe {
            media_stream_track::is_noise_suppression_enabled(self.inner.get())
        }?;
        unsafe { FutureFromDart::execute::<bool>(fut) }.await
    }

    /// Returns the current configured [`NoiseSuppressionLevel`] of this
    /// [`MediaStreamTrack`].
    ///
    /// __NOTE__: Only supported on desktop platforms.
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    pub async fn get_noise_suppression_level(
        &self,
    ) -> Result<NoiseSuppressionLevel, platform::Error> {
        let fut = unsafe {
            media_stream_track::get_noise_suppression_level(self.inner.get())
        }?;
        let level = unsafe { FutureFromDart::execute::<i64>(fut) }.await?;

        Ok(NoiseSuppressionLevel::try_from(level).unwrap())
    }

    /// Indicates whether auto gain control is enabled for this
    /// [`MediaStreamTrack`].
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    pub async fn is_auto_gain_control_enabled(
        &self,
    ) -> Result<bool, platform::Error> {
        let fut = unsafe {
            media_stream_track::is_auto_gain_control_enabled(self.inner.get())
        }?;
        unsafe { FutureFromDart::execute::<bool>(fut) }.await
    }

    /// Indicates whether echo cancellation is enabled for this
    /// [`MediaStreamTrack`].
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    pub async fn is_echo_cancellation_enabled(
        &self,
    ) -> Result<bool, platform::Error> {
        let fut = unsafe {
            media_stream_track::is_echo_cancellation_enabled(self.inner.get())
        }?;
        unsafe { FutureFromDart::execute::<bool>(fut) }.await
    }

    /// Indicates whether high-pass filter is enabled for this
    /// [`MediaStreamTrack`].
    ///
    /// __NOTE__: Only supported on desktop platforms.
    ///
    /// # Errors
    ///
    /// With a [`platform::Error`] if platform call errors.
    pub async fn is_high_pass_filter_enabled(
        &self,
    ) -> Result<bool, platform::Error> {
        let fut = unsafe {
            media_stream_track::is_high_pass_filter_enabled(self.inner.get())
        }?;
        unsafe { FutureFromDart::execute::<bool>(fut) }.await
    }
}

impl Drop for MediaStreamTrack {
    fn drop(&mut self) {
        let track = self.inner.clone();
        platform::spawn(async move {
            let fut =
                unsafe { media_stream_track::dispose(track.get()) }.unwrap();
            unsafe { FutureFromDart::execute::<()>(fut) }.await.unwrap();
        });
    }
}
