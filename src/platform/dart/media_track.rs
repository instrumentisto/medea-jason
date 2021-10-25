//! Wrapper around [MediaStreamTrack][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack

use std::{os::raw::c_char, ptr};

use dart_sys::Dart_Handle;
use derive_more::From;

use crate::{
    api::dart::{utils::c_str_into_string, DartValueArg},
    media::{track::MediaStreamTrackState, FacingMode, MediaKind},
    platform::dart::utils::{callback_listener::Callback, handle::DartHandle},
};
use std::convert::{TryFrom, TryInto};

/// Pointer to an extern function that returns ID of the provided
/// [`MediaStreamTrack`].
type IdFunction = extern "C" fn(Dart_Handle) -> ptr::NonNull<c_char>;

/// Pointer to an extern function that returns device ID of the provided
/// [`MediaStreamTrack`].
type DeviceIdFunction =
    extern "C" fn(Dart_Handle) -> DartValueArg<Option<String>>;

/// Pointer to an extern function that returns facing mode of the provided
/// [`MediaStreamTrack`].
type FacingModeFunction =
    extern "C" fn(Dart_Handle) -> DartValueArg<Option<i32>>;

/// Pointer to an extern function that returns height of the provided
/// [`MediaStreamTrack`].
type HeightFunction = extern "C" fn(Dart_Handle) -> DartValueArg<Option<u32>>;

/// Pointer to an extern function that returns width of the provided
/// [`MediaStreamTrack`].
type WidthFunction = extern "C" fn(Dart_Handle) -> DartValueArg<Option<u32>>;

/// Pointer to an extern function that sets `enabled` field of the provided
/// [`MediaStreamTrack`] to the provided [`bool`].
type SetEnabledFunction = extern "C" fn(Dart_Handle, bool);

/// Pointer to an extern function that stops provided [`MediaStreamTrack`].
type StopFunction = extern "C" fn(Dart_Handle);

/// Pointer to an extern function that returns `enabled` field of the provided
/// [`MediaStreamTrack`].
type EnabledFunction = extern "C" fn(Dart_Handle) -> bool;

/// Pointer to an extern function that returns kind of the provided
/// [`MediaStreamTrack`].
type KindFunction = extern "C" fn(Dart_Handle) -> i32;

/// Pointer to an extern function that returns readiness state of the provided
/// [`MediaStreamTrack`].
type ReadyStateFunction = extern "C" fn(Dart_Handle) -> i32;

// Pointer to an extern function that sets `on_ended` callback of the provided
// [`MediaStreamTrack`].
type OnEndedFunction = extern "C" fn(Dart_Handle, Dart_Handle);

/// Stores pointer to the [`IdFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut ID_FUNCTION: Option<IdFunction> = None;

/// Stores pointer to the [`DeviceIdFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut DEVICE_ID_FUNCTION: Option<DeviceIdFunction> = None;

/// Stores pointer to the [`FacingModeFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut FACING_MODE_FUNCTION: Option<FacingModeFunction> = None;

/// Stores pointer to the [`HeightFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut HEIGHT_FUNCTION: Option<HeightFunction> = None;

/// Stores pointer to the [`WidthFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut WIDTH_FUNCTION: Option<WidthFunction> = None;

/// Stores pointer to the [`SetEnabledFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut SET_ENABLED_FUNCTION: Option<SetEnabledFunction> = None;

/// Stores pointer to the [`StopFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut STOP_FUNCTION: Option<StopFunction> = None;

/// Stores pointer to the [`EnabledFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut ENABLED_FUNCTION: Option<EnabledFunction> = None;

/// Stores pointer to the [`KindFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut KIND_FUNCTION: Option<KindFunction> = None;

/// Stores pointer to the [`ReadyStateFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut READY_STATE_FUNCTION: Option<ReadyStateFunction> = None;

/// Stores pointer to the [`OnEndedFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut ON_ENDED_FUNCTION: Option<OnEndedFunction> = None;

/// Wrapper around [MediaStreamTrack][1] received from a
/// [getUserMedia()][2]/[getDisplayMedia()][3] request.
///
/// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
/// [2]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [3]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
#[derive(Clone, From, Debug)]
pub struct MediaStreamTrack(DartHandle);

/// Registers the provided [`IdFunction`] as [`ID_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaStreamTrack__id(f: IdFunction) {
    ID_FUNCTION = Some(f);
}

/// Registers the provided [`DeviceIdFunction`] as [`DEVICE_ID_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaStreamTrack__device_id(
    f: DeviceIdFunction,
) {
    DEVICE_ID_FUNCTION = Some(f);
}

/// Registers the provided [`FacingModeFunction`] as [`FACING_MODE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaStreamTrack__facing_mode(
    f: FacingModeFunction,
) {
    FACING_MODE_FUNCTION = Some(f);
}

/// Registers the provided [`HeightFunction`] as [`HEIGHT_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaStreamTrack__height(f: HeightFunction) {
    HEIGHT_FUNCTION = Some(f);
}

/// Registers the provided [`WidthFunction`] as [`WIDTH_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaStreamTrack__width(f: WidthFunction) {
    WIDTH_FUNCTION = Some(f);
}

/// Registers the provided [`SetEnabledFunction`] as [`SET_ENABLED_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaStreamTrack__set_enabled(
    f: SetEnabledFunction,
) {
    SET_ENABLED_FUNCTION = Some(f);
}

/// Registers the provided [`StopFunction`] as [`STOP_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaStreamTrack__stop(f: StopFunction) {
    STOP_FUNCTION = Some(f);
}

/// Registers the provided [`EnabledFunction`] as [`ENABLED_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaStreamTrack__enabled(
    f: EnabledFunction,
) {
    ENABLED_FUNCTION = Some(f);
}

/// Registers the provided [`KindFunction`] as [`KIND_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaStreamTrack__kind(f: KindFunction) {
    KIND_FUNCTION = Some(f);
}

/// Registers the provided [`ReadyStateFunction`] as [`READY_STATE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaStreamTrack__ready_state(
    f: ReadyStateFunction,
) {
    READY_STATE_FUNCTION = Some(f);
}

/// Registers the provided [`OnEndedFunction`] as [`ON_ENDED_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_MediaStreamTrack__on_ended(
    f: OnEndedFunction,
) {
    ON_ENDED_FUNCTION = Some(f);
}

impl MediaStreamTrack {
    /// Returns underlying [`Dart_Handle`] of this [`MediaStreamTrack`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.0.get()
    }

    /// Returns [`id`] of the underlying [MediaStreamTrack][2].
    ///
    /// [`id`]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-id
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn id(&self) -> String {
        unsafe { c_str_into_string(ID_FUNCTION.unwrap()(self.0.get())) }
    }

    /// Returns this [`MediaStreamTrack`]'s kind (audio/video).
    #[must_use]
    pub fn kind(&self) -> MediaKind {
        MediaKind::from(unsafe { KIND_FUNCTION.unwrap()(self.0.get()) })
    }

    /// Returns [MediaStreamTrackState][1] of the underlying
    /// [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrackstate
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[allow(clippy::unused_self)]
    #[must_use]
    pub fn ready_state(&self) -> MediaStreamTrackState {
        // TODO (evdokimovs): return real MediaStreamTrackState when
        // flutter_webrtc will be reworked
        MediaStreamTrackState::Live
    }

    /// Returns a [`deviceId`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-deviceid
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn device_id(&self) -> Option<String> {
        unsafe { DEVICE_ID_FUNCTION.unwrap()(self.0.get()) }
            .try_into()
            .unwrap()
    }

    /// Return a [`facingMode`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-facingmode
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn facing_mode(&self) -> Option<FacingMode> {
        Option::<i32>::try_from(unsafe {
            FACING_MODE_FUNCTION.unwrap()(self.0.get())
        })
        .unwrap()
        .map(FacingMode::from)
    }

    /// Returns a [`height`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-height
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn height(&self) -> Option<u32> {
        Option::try_from(unsafe { HEIGHT_FUNCTION.unwrap()(self.0.get()) })
            .unwrap()
    }

    /// Return a [`width`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediatracksettings-width
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn width(&self) -> Option<u32> {
        Option::try_from(unsafe { WIDTH_FUNCTION.unwrap()(self.0.get()) })
            .unwrap()
    }

    /// Changes an [`enabled`][1] attribute in the underlying
    /// [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            SET_ENABLED_FUNCTION.unwrap()(self.0.get(), enabled);
        }
    }

    /// Changes a [`readyState`][1] attribute in the underlying
    /// [MediaStreamTrack][2] to [`ended`][3].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-readystate
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    /// [3]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
    pub fn stop(&self) {
        unsafe {
            STOP_FUNCTION.unwrap()(self.0.get());
        }
    }

    /// Returns an [`enabled`][1] attribute of the underlying
    /// [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn enabled(&self) -> bool {
        unsafe { ENABLED_FUNCTION.unwrap()(self.0.get()) }
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
        // TODO (evdokimovs): add real implementation when flutter_webrtc will
        // be reworked
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
        self.clone()
    }

    /// Sets handler for the [`ended`][1] event on underlying
    /// [`web_sys::MediaStreamTrack`].
    ///
    /// [1]: https://tinyurl.com/w3-streams#event-mediastreamtrack-ended
    pub fn on_ended<F>(&self, f: Option<F>)
    where
        F: 'static + FnOnce(),
    {
        if let Some(cb) = f {
            let cb = Callback::callback(|_: ()| cb());
            unsafe { ON_ENDED_FUNCTION.unwrap()(self.0.get(), cb) };
        }
    }
}
