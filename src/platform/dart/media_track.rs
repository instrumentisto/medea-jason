//! Wrapper around [MediaStreamTrack][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack

use std::convert::TryFrom;

use dart_sys::Dart_Handle;
use derive_more::From;
use medea_macro::dart_bridge;

use crate::{
    api::c_str_into_string,
    media::{track::MediaStreamTrackState, FacingMode, MediaKind},
    platform::dart::utils::{
        callback::Callback, handle::DartHandle, NonNullDartValueArgExt as _,
    },
};

#[dart_bridge("flutter/lib/src/native/platform/media_track.g.dart")]
mod media_stream_track {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::api::DartValueArg;

    extern "C" {
        /// Returns ID of the provided [`MediaStreamTrack`].
        pub fn id(track: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns device ID of the provided [`MediaStreamTrack`].
        pub fn device_id(track: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns facing mode of the provided [`MediaStreamTrack`].
        pub fn facing_mode(
            track: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<i64>>>;

        /// Returns height of the provided [`MediaStreamTrack`].
        pub fn height(
            track: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns width of the provided [`MediaStreamTrack`].
        pub fn width(
            track: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Sets `enabled` field of the provided [`MediaStreamTrack`] to the
        /// provided [`bool`].
        pub fn set_enabled(track: Dart_Handle, is_enabled: bool);

        /// Stops provided [`MediaStreamTrack`].
        pub fn stop(track: Dart_Handle);

        /// Returns `enabled` field of the provided [`MediaStreamTrack`].
        pub fn enabled(track: Dart_Handle) -> bool;

        /// Returns kind of the provided [`MediaStreamTrack`].
        pub fn kind(track: Dart_Handle) -> i64;

        /// Returns readiness state of the provided [`MediaStreamTrack`].
        pub fn ready_state(track: Dart_Handle) -> i64;

        /// Sets `on_ended` callback of the provided [`MediaStreamTrack`].
        pub fn on_ended(track: Dart_Handle, cb: Dart_Handle);
    }
}

/// Wrapper around [MediaStreamTrack][1] received from a
/// [getUserMedia()][2]/[getDisplayMedia()][3] request.
///
/// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
/// [2]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [3]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
#[derive(Clone, From, Debug)]
pub struct MediaStreamTrack(DartHandle);

impl MediaStreamTrack {
    /// Returns the underlying [`Dart_Handle`] of this [`MediaStreamTrack`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.0.get()
    }

    /// Returns [`id`] of the underlying [MediaStreamTrack][2].
    ///
    /// [`id`]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-id
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[inline]
    #[must_use]
    pub fn id(&self) -> String {
        unsafe { c_str_into_string(media_stream_track::id(self.0.get())) }
    }

    /// Returns this [`MediaStreamTrack`]'s kind (audio/video).
    #[inline]
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        MediaKind::try_from(unsafe { media_stream_track::kind(self.0.get()) })
            .unwrap()
    }

    /// Returns [MediaStreamTrackState][1] of the underlying
    /// [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrackstate
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn ready_state(&self) -> MediaStreamTrackState {
        // TODO: Correct implementation requires `flutter_webrtc`-side fixes.
        MediaStreamTrackState::Live
    }

    /// Returns a [`deviceId`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-deviceid
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[inline]
    #[must_use]
    pub fn device_id(&self) -> String {
        unsafe {
            c_str_into_string(media_stream_track::device_id(self.0.get()))
        }
    }

    /// Return a [`facingMode`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-facingmode
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn facing_mode(&self) -> Option<FacingMode> {
        Option::<i64>::try_from(unsafe {
            media_stream_track::facing_mode(self.0.get()).unbox()
        })
        .unwrap()
        .map(FacingMode::try_from)
        .transpose()
        .unwrap()
    }

    /// Returns a [`height`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-height
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[inline]
    #[must_use]
    pub fn height(&self) -> Option<u32> {
        Option::try_from(unsafe {
            media_stream_track::height(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Return a [`width`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediatracksettings-width
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[inline]
    #[must_use]
    pub fn width(&self) -> Option<u32> {
        Option::try_from(unsafe {
            media_stream_track::width(self.0.get()).unbox()
        })
        .unwrap()
    }

    /// Changes an [`enabled`][1] attribute in the underlying
    /// [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[inline]
    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            media_stream_track::set_enabled(self.0.get(), enabled);
        }
    }

    /// Changes a [`readyState`][1] attribute in the underlying
    /// [MediaStreamTrack][2] to [`ended`][3].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-readystate
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    /// [3]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
    #[inline]
    pub fn stop(&self) {
        unsafe {
            media_stream_track::stop(self.0.get());
        }
    }

    /// Returns an [`enabled`][1] attribute of the underlying
    /// [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[inline]
    #[must_use]
    pub fn enabled(&self) -> bool {
        unsafe { media_stream_track::enabled(self.0.get()) }
    }

    /// Detects whether a video track captured from display searching
    /// [specific fields][1] in its settings.
    ///
    /// Only works in Chrome browser at the moment.
    ///
    /// [1]: https://w3.org/TR/screen-capture/#extensions-to-mediatracksettings
    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn guess_is_from_display(&self) -> bool {
        // TODO: Correct implementation requires `flutter_webrtc`-side fixes.
        false
    }

    /// Forks this [`MediaStreamTrack`].
    ///
    /// Creates a new [`MediaStreamTrack`] from this [`MediaStreamTrack`] using
    /// a [`clone()`][1] method. It won't clone current [`MediaStreamTrack`]'s
    /// callbacks.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-clone
    #[must_use]
    pub fn fork(&self) -> Self {
        // TODO: Correct implementation requires `flutter_webrtc`-side fixes.
        self.clone()
    }

    /// Sets handler for the [`ended`][1] event on the underlying
    /// [MediaStreamTrack][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#event-mediastreamtrack-ended
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    pub fn on_ended<F>(&self, f: Option<F>)
    where
        F: 'static + FnOnce(),
    {
        if let Some(cb) = f {
            let cb = Callback::from_once(|_: ()| cb());
            unsafe {
                media_stream_track::on_ended(self.0.get(), cb.into_dart());
            };
        }
    }
}
