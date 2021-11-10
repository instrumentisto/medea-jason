//! [RTCRtpTransceiver] wrapper.
//!
//! [RTCRtpTransceiver]: https://w3.org/TR/webrtc/#dom-rtcrtptransceiver

use std::{
    cell::RefCell,
    convert::{TryFrom, TryInto},
    future::Future,
    ptr,
    rc::Rc,
};

use dart_sys::Dart_Handle;
use futures::future::LocalBoxFuture;

use crate::{
    api::DartValueArg,
    media::track::local,
    platform,
    platform::{
        dart::utils::{dart_future::FutureFromDart, handle::DartHandle},
        TransceiverDirection,
    },
};

/// Pointer to an extern function returning current direction of the provided
/// [`Transceiver`].
type GetCurrentDirectionFunction = extern "C" fn(Dart_Handle) -> Dart_Handle;

/// Pointer to an extern function that returns `send` [`MediaStreamTrack`] of
/// the provided [`Transceiver`].
type GetSendTrackFunction =
    extern "C" fn(
        Dart_Handle,
    ) -> ptr::NonNull<DartValueArg<Option<DartHandle>>>;

/// Pointer to an extern function replacing `send` [`MediaStreamTrack`] of the
/// provided [`Transceiver`].
type ReplaceTrackFunction =
    extern "C" fn(Dart_Handle, Dart_Handle) -> Dart_Handle;

/// Pointer to an extern function dropping `send` [`MediaStreamTrack`] of the
/// provided [`Transceiver`].
type DropSenderFunction = extern "C" fn(Dart_Handle) -> Dart_Handle;

/// Pointer to an extern function returning stopped status of the provided
/// [`Transceiver`].
type IsStoppedFunction =
    extern "C" fn(Dart_Handle) -> ptr::NonNull<DartValueArg<i8>>;

/// Pointer to an extern function setting `enabled` field of `send`
/// [`MediaStreamTrack`] of the provided [`Transceiver`].
type SetSendTrackEnabledFunction = extern "C" fn(Dart_Handle, bool);

/// Pointer to an extern function returning MID of the provided [`Transceiver`].
type MidFunction =
    extern "C" fn(Dart_Handle) -> ptr::NonNull<DartValueArg<Option<String>>>;

/// Pointer to an extern function indicating whether the provided
/// [`Transceiver`] has `send` [`MediaStreamTrack`].
type HasSendTrackFunction = extern "C" fn(Dart_Handle) -> i8;

/// Pointer to an extern function setting `direction` of the provided
/// [`Transceiver`].
type SetDirectionFunction = extern "C" fn(Dart_Handle, i64) -> Dart_Handle;

/// Stores pointer to the [`GetCurrentDirectionFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut GET_CURRENT_DIRECTION_FUNCTION: Option<GetCurrentDirectionFunction> =
    None;

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

/// Wrapper around [RTCRtpTransceiver] which provides handy methods for
/// direction changes.
///
/// [RTCRtpTransceiver]: https://w3.org/TR/webrtc/#dom-rtcrtptransceiver
#[derive(Clone, Debug)]
pub struct Transceiver {
    transceiver: DartHandle,
    send_track: RefCell<Option<Rc<local::Track>>>,
}

impl Transceiver {
    /// Disables provided [`TransceiverDirection`] of this [`Transceiver`].
    pub fn sub_direction(
        &self,
        disabled_direction: TransceiverDirection,
    ) -> LocalBoxFuture<'static, ()> {
        let this = self.clone();
        Box::pin(async move {
            this.set_direction(
                this.current_direction().await - disabled_direction,
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
                this.current_direction().await | enabled_direction,
            )
            .await;
        })
    }

    /// Indicates whether the provided [`TransceiverDirection`] is enabled for
    /// this [`Transceiver`].
    pub async fn has_direction(&self, direction: TransceiverDirection) -> bool {
        self.current_direction().await.contains(direction)
    }

    /// Replaces [`TransceiverDirection::SEND`] [`local::Track`] of this
    /// [`Transceiver`].
    ///
    /// # Errors
    ///
    /// Errors with [`Error`] if the underlying [`replaceTrack`][1] call fails.
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-rtcrtpsender-replacetrack
    pub async fn set_send_track(
        &self,
        new_sender: Rc<local::Track>,
    ) -> Result<(), platform::Error> {
        FutureFromDart::execute::<()>(unsafe {
            REPLACE_TRACK_FUNCTION.unwrap()(
                self.transceiver.get(),
                new_sender.platform_track().handle(),
            )
        })
        .await
        .unwrap();
        self.send_track.replace(Some(new_sender));
        Ok(())
    }

    /// Sets a [`TransceiverDirection::SEND`] [`local::Track`] of this
    /// [`Transceiver`] to [`None`].
    pub fn drop_send_track(&self) -> impl Future<Output = ()> {
        drop(self.send_track.borrow_mut().take());
        let transceiver = self.transceiver.get();
        async move {
            FutureFromDart::execute::<()>(unsafe {
                DROP_SENDER_FUNCTION.unwrap()(transceiver)
            })
            .await
            .unwrap();
        }
    }

    /// Returns [`mid`] of this [`Transceiver`].
    ///
    /// [`mid`]: https://w3.org/TR/webrtc/#dom-rtptransceiver-mid
    #[must_use]
    pub fn mid(&self) -> Option<String> {
        unsafe {
            let mid = MID_FUNCTION.unwrap()(self.transceiver.get());
            (*Box::from_raw(mid.as_ptr())).try_into().unwrap()
        }
    }

    /// Returns [`local::Track`] that is being send to remote, if any.
    #[must_use]
    pub fn send_track(&self) -> Option<Rc<local::Track>> {
        self.send_track.borrow().as_ref().cloned()
    }

    /// Indicates whether this [`Transceiver`] has [`local::Track`].
    #[must_use]
    pub fn has_send_track(&self) -> bool {
        unsafe { HAS_SEND_TRACK_FUNCTION.unwrap()(self.transceiver.get()) == 1 }
    }

    /// Sets the underlying [`local::Track`]'s `enabled` field to the provided
    /// value, if any.
    pub fn set_send_track_enabled(&self, enabled: bool) {
        unsafe {
            if let Some(sender) =
                Option::<DartHandle>::try_from(*Box::from_raw(
                    GET_SEND_TRACK_FUNCTION.unwrap()(self.transceiver.get())
                        .as_ptr(),
                ))
                .unwrap()
            {
                SET_SEND_TRACK_ENABLED_FUNCTION.unwrap()(sender.get(), enabled);
            }
        }
    }

    /// Indicates whether the underlying [RTCRtpTransceiver] is stopped.
    #[must_use]
    pub fn is_stopped(&self) -> bool {
        let val = unsafe {
            let p = IS_STOPPED_FUNCTION.unwrap()(self.transceiver.get());
            *Box::from_raw(p.as_ptr())
        };
        i8::try_from(val).unwrap() == 1
    }

    /// Returns current [`TransceiverDirection`] of this [`Transceiver`].
    fn current_direction(&self) -> impl Future<Output = TransceiverDirection> {
        let handle = self.transceiver.get();
        async move {
            FutureFromDart::execute::<i32>(unsafe {
                GET_CURRENT_DIRECTION_FUNCTION.unwrap()(handle)
            })
            .await
            .unwrap()
            .into()
        }
    }

    /// Sets this [`Transceiver`] to the provided [`TransceiverDirection`].
    fn set_direction(
        &self,
        direction: TransceiverDirection,
    ) -> LocalBoxFuture<'static, ()> {
        let handle = self.transceiver.get();
        Box::pin(async move {
            FutureFromDart::execute::<()>(unsafe {
                SET_DIRECTION_FUNCTION.unwrap()(handle, direction.into())
            })
            .await
            .unwrap();
        })
    }
}

impl From<DartHandle> for Transceiver {
    fn from(handle: DartHandle) -> Self {
        Self {
            transceiver: handle,
            send_track: RefCell::new(None),
        }
    }
}
