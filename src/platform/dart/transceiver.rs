//! [`RtcRtpTransceiver`] wrapper.

use std::{cell::RefCell, future::Future, ptr, rc::Rc};

use dart_sys::Dart_Handle;
use futures::future::LocalBoxFuture;

use crate::{
    api::dart::{DartValue, DartValueArg},
    media::track::local,
    platform,
    platform::{
        dart::utils::{dart_future::DartFutureResolver, handle::DartHandle},
        TransceiverDirection,
    },
};
use std::convert::{TryFrom, TryInto};

/// Wrapper around `RTCRtpTransceiver`'s [`DartHandle`] which provides handy
/// methods for direction changes.
#[derive(Clone, Debug)]
pub struct Transceiver {
    transceiver: DartHandle,
    send_track: RefCell<Option<Rc<local::Track>>>,
}

impl From<DartHandle> for Transceiver {
    fn from(handle: DartHandle) -> Self {
        Self {
            transceiver: handle,
            send_track: RefCell::new(None),
        }
    }
}

/// Pointer to an extern function that request that returns current direction of
/// the provided [`Transceiver`].
type GetCurrentDirectionFunction = extern "C" fn(Dart_Handle) -> Dart_Handle;

// TODO: maybe remove it
type CurrentDirectionFunction = extern "C" fn(Dart_Handle) -> i32;

/// Pointer to an extern function that returns `Send` [`MediaStreamTrack`] of
/// the provided [`Transceiver`].
type GetSendTrackFunction =
    extern "C" fn(Dart_Handle) -> DartValueArg<Option<DartHandle>>;

/// Pointer to an extern function that replaces `Send` [`MediaStreamTrack`] of
/// the provided [`Transceiver`].
type ReplaceTrackFunction =
    extern "C" fn(Dart_Handle, Dart_Handle) -> Dart_Handle;

/// Pointer to an extern function that drops `Send` [`MediaStreamTrack`] of the
/// provided [`Transceiver`].
type DropSenderFunction = extern "C" fn(Dart_Handle);

/// Pointer to an extern function that returns stopped status of the provided
/// [`Transceiver`].
type IsStoppedFunction = extern "C" fn(Dart_Handle) -> bool;

/// Pointer to an extern function that sets `enabled` field of `Send`
/// [`MediaStreamTrack`] of the provided [`Transceiver`].
type SetSendTrackEnabledFunction = extern "C" fn(Dart_Handle, bool);

/// Pointer to an extern function that returns MID of the provided
/// [`Transceiver`].
type MidFunction = extern "C" fn(Dart_Handle) -> DartValueArg<Option<String>>;

/// Pointer to an extern function that returns `1` if provided [`Transceiver`]
/// has `Send` [`MediaStreamTrack`].
type HasSendTrackFunction = extern "C" fn(Dart_Handle) -> i8;

/// Pointer to an extern function that sets `direction` this [`Transceiver`].
type SetDirectionFunction = extern "C" fn(Dart_Handle, i32) -> Dart_Handle;

/// Stores pointer to the [`GetCurrentDirectionFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut GET_CURRENT_DIRECTION_FUNCTION: Option<GetCurrentDirectionFunction> =
    None;

/// Stores pointer to the [`CurrentDirectionFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut CURRENT_DIRECTION_FUNCTION: Option<CurrentDirectionFunction> = None;

/// Stores pointer to the [`GetSendTrackFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut GET_SEND_TRACK_FUNCTION: Option<GetSendTrackFunction> = None;

/// Stores pointer to the [`ReplaceTrackFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut REPLACE_TRACK_FUNCTION: Option<ReplaceTrackFunction> = None;

/// Stores pointer to the [`DropSenderFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut DROP_SENDER_FUNCTION: Option<DropSenderFunction> = None;

/// Stores pointer to the [`SetSendTrackEnabledFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut SET_SEND_TRACK_ENABLED_FUNCTION: Option<
    SetSendTrackEnabledFunction,
> = None;

/// Stores pointer to the [`IsStoppedFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut IS_STOPPED_FUNCTION: Option<IsStoppedFunction> = None;

/// Stores pointer to the [`MidFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut MID_FUNCTION: Option<MidFunction> = None;

/// Stores pointer to the [`HasSendTrackFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut HAS_SEND_TRACK_FUNCTION: Option<HasSendTrackFunction> = None;

/// Stores pointer to the [`SetDirectionFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut SET_DIRECTION_FUNCTION: Option<SetDirectionFunction> = None;

/// Registers the provided [`GetCurrentDirectionFunction`] as
/// [`GET_CURRENT_DIRECTION_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Transceiver__get_current_direction(
    f: GetCurrentDirectionFunction,
) {
    GET_CURRENT_DIRECTION_FUNCTION = Some(f);
}

/// Registers the provided [`CurrentDirectionFunction`] as
/// [`CURRENT_DIRECTION_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Transceiver__current_direction(
    f: CurrentDirectionFunction,
) {
    CURRENT_DIRECTION_FUNCTION = Some(f);
}

/// Registers the provided [`GetSendTrackFunction`] as
/// [`GET_SEND_TRACK_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Transceiver__get_send_track(
    f: GetSendTrackFunction,
) {
    GET_SEND_TRACK_FUNCTION = Some(f);
}

/// Registers the provided [`ReplaceTrackFunction`] as
/// [`REPLACE_TRACK_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Transceiver__replace_track(
    f: ReplaceTrackFunction,
) {
    REPLACE_TRACK_FUNCTION = Some(f);
}

/// Registers the provided [`SetSendTrackEnabledFunction`] as
/// [`SET_SEND_TRACK_ENABLED_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Transceiver__set_send_track_enabled(
    f: SetSendTrackEnabledFunction,
) {
    SET_SEND_TRACK_ENABLED_FUNCTION = Some(f);
}

/// Registers the provided [`DropSenderFunction`] as [`DROP_SENDER_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Transceiver__drop_sender(
    f: DropSenderFunction,
) {
    DROP_SENDER_FUNCTION = Some(f);
}

/// Registers the provided [`IsStoppedFunction`] as [`IS_STOPPED_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Transceiver__is_stopped(
    f: IsStoppedFunction,
) {
    IS_STOPPED_FUNCTION = Some(f);
}

/// Registers the provided [`MidFunction`] as [`MID_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Transceiver__mid(f: MidFunction) {
    MID_FUNCTION = Some(f);
}

/// Registers the provided [`HasSendTrackFunction`] as
/// [`HAS_SEND_TRACK_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Transceiver__has_send_track(
    f: HasSendTrackFunction,
) {
    HAS_SEND_TRACK_FUNCTION = Some(f);
}

/// Registers the provided [`SetDirectionFunction`] as
/// [`SET_DIRECTION_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Transceiver__set_direction(
    f: SetDirectionFunction,
) {
    SET_DIRECTION_FUNCTION = Some(f);
}

impl Transceiver {
    /// Returns current [`TransceiverDirection`] of this [`Transceiver`].
    pub fn current_direction(&self) -> TransceiverDirection {
        unsafe {
            CURRENT_DIRECTION_FUNCTION.unwrap()(self.transceiver.get()).into()
        }
    }

    fn set_direction(
        &self,
        direction: TransceiverDirection,
    ) -> LocalBoxFuture<'static, ()> {
        let fut = DartFutureResolver::execute::<()>(unsafe {
            SET_DIRECTION_FUNCTION.unwrap()(
                self.transceiver.get(),
                direction.into(),
            )
        });
        Box::pin(async move {
            fut.await;
        })
    }

    /// Disables provided [`TransceiverDirection`] of this [`Transceiver`].
    pub fn sub_direction(
        &self,
        disabled_direction: TransceiverDirection,
    ) -> LocalBoxFuture<'static, ()> {
        let this = self.clone();
        Box::pin(async move {
            this.set_direction(
                this.get_current_direction().await - disabled_direction,
            )
            .await;
        })
    }

    /// Enables provided [`TransceiverDirection`] of this [`Transceiver`].
    pub fn add_direction(
        &self,
        enabled_direction: TransceiverDirection,
    ) -> LocalBoxFuture<'static, ()> {
        let this = self.clone();
        Box::pin(async move {
            this.set_direction(
                this.get_current_direction().await | enabled_direction,
            )
            .await;
        })
    }

    /// Indicates whether the provided [`TransceiverDirection`] is enabled for
    /// this [`Transceiver`].
    pub fn has_direction(&self, direction: TransceiverDirection) -> bool {
        self.current_direction().contains(direction)
    }

    /// Replaces [`TransceiverDirection::SEND`] [`local::Track`] of this
    /// [`Transceiver`].
    ///
    /// # Errors
    ///
    /// Errors with JS error if the underlying [`replaceTrack`][1] call fails.
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-rtcrtpsender-replacetrack
    pub async fn set_send_track(
        &self,
        new_sender: Rc<local::Track>,
    ) -> Result<(), platform::Error> {
        unsafe {
            DartFutureResolver::execute::<()>(REPLACE_TRACK_FUNCTION.unwrap()(
                self.transceiver.get(),
                new_sender.platform_track().handle(),
            ))
            .await;
        }
        self.send_track.replace(Some(new_sender));
        Ok(())
    }

    /// Sets a [`TransceiverDirection::SEND`] [`local::Track`] of this
    /// [`Transceiver`] to [`None`].
    ///
    /// # Panics
    ///
    /// If [`local::Track`] replacement with [`None`] fails on JS side, but
    /// basing on [WebAPI docs] it should never happen.
    ///
    /// [WebAPI docs]: https://tinyurl.com/7pnszaa8
    pub fn drop_send_track(&self) -> impl Future<Output = ()> {
        unsafe {
            if let Some(sender) = Option::<DartHandle>::try_from(
                GET_SEND_TRACK_FUNCTION.unwrap()(self.transceiver.get()),
            )
            .unwrap()
            {
                DROP_SENDER_FUNCTION.unwrap()(sender.get());
            }
        }
        async {}
    }

    /// Sets the underlying [`local::Track`]'s `enabled` field to the provided
    /// value, if any.
    pub fn set_send_track_enabled(&self, enabled: bool) {
        unsafe {
            if let Some(sender) = Option::<DartHandle>::try_from(
                GET_SEND_TRACK_FUNCTION.unwrap()(self.transceiver.get()),
            )
            .unwrap()
            {
                SET_SEND_TRACK_ENABLED_FUNCTION.unwrap()(sender.get(), enabled);
            }
        }
    }

    /// Returns current [`TransceiverDirection`] of this [`Transceiver`].
    pub fn get_current_direction(
        &self,
    ) -> impl Future<Output = TransceiverDirection> {
        let handle = self.transceiver.get();
        unsafe {
            async move {
                DartFutureResolver::execute::<i32>(
                    GET_CURRENT_DIRECTION_FUNCTION.unwrap()(handle),
                )
                .await
                .into()
            }
        }
    }

    /// Indicates whether the underlying [`RtcRtpTransceiver`] is stopped.
    pub fn is_stopped(&self) -> bool {
        unsafe { IS_STOPPED_FUNCTION.unwrap()(self.transceiver.get()) }
    }

    /// Returns [`mid`] of this [`Transceiver`].
    ///
    /// [`mid`]: https://w3.org/TR/webrtc/#dom-rtptransceiver-mid
    pub fn mid(&self) -> Option<String> {
        unsafe {
            MID_FUNCTION.unwrap()(self.transceiver.get())
                .try_into()
                .unwrap()
        }
    }

    /// Returns [`local::Track`] that is being send to remote, if any.
    pub fn send_track(&self) -> Option<Rc<local::Track>> {
        self.send_track.borrow().as_ref().cloned()
    }

    /// Indicates whether this [`Transceiver`] has [`local::Track`].
    pub fn has_send_track(&self) -> bool {
        unsafe { HAS_SEND_TRACK_FUNCTION.unwrap()(self.transceiver.get()) == 1 }
    }
}
