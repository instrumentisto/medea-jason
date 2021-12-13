//! [RTCRtpTransceiver] wrapper.
//!
//! [RTCRtpTransceiver]: https://w3.org/TR/webrtc/#dom-rtcrtptransceiver

use std::{
    cell::RefCell,
    convert::{TryFrom, TryInto},
    future::Future,
    rc::Rc,
};

use futures::future::LocalBoxFuture;
use medea_macro::dart_bridge;

use crate::{
    media::track::local,
    platform,
    platform::{
        dart::utils::{
            dart_future::FutureFromDart, handle::DartHandle,
            NonNullDartValueArgExt,
        },
        TransceiverDirection,
    },
};

#[dart_bridge("flutter/lib/src/native/platform/transceiver.g.dart")]
mod transceiver {
    use std::ptr;

    use dart_sys::Dart_Handle;

    use crate::{api::DartValueArg, platform::dart::utils::handle::DartHandle};

    extern "C" {
        /// Returns current direction of the provided [`Transceiver`].
        pub fn get_current_direction(transceiver: Dart_Handle) -> Dart_Handle;

        /// Returns `Send` [`MediaStreamTrack`] of the provided [`Transceiver`].
        pub fn get_send_track(
            transceiver: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<DartHandle>>>;

        /// Replaces `Send` [`MediaStreamTrack`] of the provided
        /// [`Transceiver`].
        pub fn replace_track(
            transceiver: Dart_Handle,
            track: Dart_Handle,
        ) -> Dart_Handle;

        /// Drops `Send` [`MediaStreamTrack`] of the provided [`Transceiver`].
        pub fn drop_sender(transceiver: Dart_Handle) -> Dart_Handle;

        /// Returns stopped status of the provided [`Transceiver`].
        pub fn is_stopped(
            transceiver: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<i8>>;

        /// Sets `enabled` field of `Send` [`MediaStreamTrack`] of the provided
        /// [`Transceiver`].
        pub fn set_send_track_enabled(transceiver: Dart_Handle, enabled: i32);

        /// Returns MID of the provided [`Transceiver`].
        pub fn mid(
            transceiver: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        /// Returns `1` if the provided [`Transceiver`] has `Send`
        /// [`MediaStreamTrack`].
        pub fn has_send_track(transceiver: Dart_Handle) -> i8;

        /// Sets `direction` of this [`Transceiver`].
        pub fn set_direction(
            transceiver: Dart_Handle,
            direction: i64,
        ) -> Dart_Handle;
    }
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
            transceiver::replace_track(
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
                transceiver::drop_sender(transceiver)
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
            let mid = transceiver::mid(self.transceiver.get());
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
        unsafe { transceiver::has_send_track(self.transceiver.get()) == 1 }
    }

    /// Sets the underlying [`local::Track`]'s `enabled` field to the provided
    /// value, if any.
    pub fn set_send_track_enabled(&self, enabled: bool) {
        unsafe {
            if let Some(sender) =
                Option::<DartHandle>::try_from(*Box::from_raw(
                    transceiver::get_send_track(self.transceiver.get())
                        .as_ptr(),
                ))
                .unwrap()
            {
                transceiver::set_send_track_enabled(
                    sender.get(),
                    enabled as i32,
                );
            }
        }
    }

    /// Indicates whether the underlying [RTCRtpTransceiver] is stopped.
    #[must_use]
    pub fn is_stopped(&self) -> bool {
        let val =
            unsafe { transceiver::is_stopped(self.transceiver.get()).unbox() };
        i8::try_from(val).unwrap() == 1
    }

    /// Returns current [`TransceiverDirection`] of this [`Transceiver`].
    fn current_direction(&self) -> impl Future<Output = TransceiverDirection> {
        let handle = self.transceiver.get();
        async move {
            FutureFromDart::execute::<i32>(unsafe {
                transceiver::get_current_direction(handle)
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
                transceiver::set_direction(handle, direction.into())
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
