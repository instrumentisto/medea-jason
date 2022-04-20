//! Representation of a [MediaStreamTrack][0].
//!
//! [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack

use std::future::Future;

use dart_sys::Dart_Handle;
use medea_macro::dart_bridge;

use crate::{
    api::c_str_into_string,
    media::{
        track::MediaStreamTrackState, FacingMode, MediaKind, MediaSourceKind,
    },
    platform::{
        dart::utils::{
            callback::Callback, handle::DartHandle, NonNullDartValueArgExt as _,
        },
        utils::dart_future::FutureFromDart,
    },
};

#[dart_bridge("flutter/lib/src/native/platform/media_track.g.dart")]
mod media_stream_track {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::api::DartValueArg;

    extern "C" {
        /// Returns [ID][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-id
        pub fn id(track: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns [device ID][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://w3.org/TR/mediacapture-streams#dfn-deviceid
        pub fn device_id(track: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns [kind][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-kind
        pub fn kind(track: Dart_Handle) -> i64;

        /// Returns [facing mode][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#def-constraint-facingMode
        pub fn facing_mode(
            track: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<i64>>>;

        /// Returns [height][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-height
        pub fn height(
            track: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns [width][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-width
        pub fn width(
            track: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u32>>>;

        /// Returns [enabled][1] field of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-enabled
        pub fn enabled(track: Dart_Handle) -> bool;

        /// Sets [enabled][1] field of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-enabled
        pub fn set_enabled(track: Dart_Handle, is_enabled: bool);

        /// Returns [readiness state][1] of the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-readystate
        pub fn ready_state(track: Dart_Handle) -> i64;

        /// [Stops][1] the provided [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-stop
        pub fn stop(track: Dart_Handle);

        /// Sets [`onended`][1] event handler of the provided
        /// [MediaStreamTrack][0].
        ///
        /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
        /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-onended
        pub fn on_ended(track: Dart_Handle, cb: Dart_Handle);

        /// Creates a new instance of [`MediaStreamTrack`] depending on the same
        /// media source as this [`MediaStreamTrack`].
        pub fn clone(track: Dart_Handle) -> Dart_Handle;
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
    /// Pointer on the [MediaStreamTrack][0]
    ///
    /// [0]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    inner: DartHandle,

    /// Media source type of this [`MediaStreamTrack`].
    source_kind: Option<MediaSourceKind>,
}

impl MediaStreamTrack {
    /// Creates a new [`MediaStreamTrack`].
    #[must_use]
    pub fn new(
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
        unsafe { c_str_into_string(media_stream_track::id(self.inner.get())) }
    }

    /// Returns [device ID][1] of this [`MediaStreamTrack`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dfn-deviceid
    #[inline]
    #[must_use]
    pub fn device_id(&self) -> String {
        unsafe {
            c_str_into_string(media_stream_track::device_id(self.inner.get()))
        }
    }

    /// Returns [kind][1] of this [`MediaStreamTrack`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-kind
    #[inline]
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        MediaKind::try_from(unsafe {
            media_stream_track::kind(self.inner.get())
        })
        .unwrap()
    }

    /// Returns [facing mode][1] of this [`MediaStreamTrack`].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-facingmode
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn facing_mode(&self) -> Option<FacingMode> {
        Option::<i64>::try_from(unsafe {
            media_stream_track::facing_mode(self.inner.get()).unbox()
        })
        .unwrap()
        .map(FacingMode::try_from)
        .transpose()
        .unwrap()
    }

    /// Returns [height][1] of this [`MediaStreamTrack`].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-height
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn height(&self) -> Option<u32> {
        Option::try_from(unsafe {
            media_stream_track::height(self.inner.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [width][1] of this [`MediaStreamTrack`].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-width
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn width(&self) -> Option<u32> {
        Option::try_from(unsafe {
            media_stream_track::width(self.inner.get()).unbox()
        })
        .unwrap()
    }

    /// Returns [enabled][1] field of this [`MediaStreamTrack`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    #[inline]
    #[must_use]
    pub fn enabled(&self) -> bool {
        unsafe { media_stream_track::enabled(self.inner.get()) }
    }

    /// Sets [enabled][1] field of this [`MediaStreamTrack`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            media_stream_track::set_enabled(self.inner.get(), enabled);
        }
    }

    /// Returns [readiness state][1] of this [`MediaStreamTrack`].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-readystate
    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn ready_state(&self) -> MediaStreamTrackState {
        // TODO: Correct implementation requires `flutter_webrtc`-side fixes.
        MediaStreamTrackState::Live
    }

    /// [Stops][1] this [`MediaStreamTrack`].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-stop
    #[inline]
    pub fn stop(&self) {
        unsafe {
            media_stream_track::stop(self.inner.get());
        }
    }

    /// Detects whether this video [`MediaStreamTrack`] is captured from
    /// display, searching for [specific fields][1] in its settings.
    ///
    /// Only works in Chrome browser at the moment.
    ///
    /// [1]: https://w3.org/TR/screen-capture#extensions-to-mediatracksettings
    #[allow(clippy::unused_self)]
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
    pub fn fork(&self) -> impl Future<Output = Self> + 'static {
        unsafe {
            let handle = self.inner.get();
            let source_kind = self.source_kind;
            async move {
                let new_track: DartHandle =
                    FutureFromDart::execute(media_stream_track::clone(handle))
                        .await
                        .unwrap();
                Self::new(new_track, source_kind)
            }
        }
    }

    /// Sets [`onended`][1] event handler of this [`MediaStreamTrack`].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-onended
    pub fn on_ended<F>(&self, f: Option<F>)
    where
        F: 'static + FnOnce(),
    {
        if let Some(cb) = f {
            let cb = Callback::from_once(|_: ()| cb());
            unsafe {
                media_stream_track::on_ended(self.inner.get(), cb.into_dart());
            };
        }
    }
}
