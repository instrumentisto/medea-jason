//! [RTCRtpTransceiver] wrapper.
//!
//! [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver

use std::{future::Future, rc::Rc};

use futures::future::LocalBoxFuture;
use medea_macro::dart_bridge;

use crate::{
    media::track::local,
    platform,
    platform::{
        dart::utils::{dart_future::FutureFromDart, handle::DartHandle},
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
        pub fn get_direction(transceiver: Dart_Handle) -> Dart_Handle;

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
        pub fn is_stopped(transceiver: Dart_Handle) -> bool;

        /// Returns MID of the provided [`Transceiver`].
        pub fn mid(
            transceiver: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        /// Changes the receive direction of the specified [`Transceiver`].
        pub fn set_recv(transceiver: Dart_Handle, active: bool) -> Dart_Handle;

        /// Changes the send direction of the specified [`Transceiver`].
        pub fn set_send(transceiver: Dart_Handle, active: bool) -> Dart_Handle;

        /// Disposes the provided [`Transceiver`].
        pub fn dispose(transceiver: Dart_Handle) -> Dart_Handle;
    }
}

/// Wrapper around [RTCRtpTransceiver] which provides handy methods for
/// direction changes.
///
/// [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver
#[derive(Clone, Debug)]
pub struct Transceiver(Rc<DartHandle>);

impl From<DartHandle> for Transceiver {
    fn from(from: DartHandle) -> Self {
        Self(Rc::new(from))
    }
}

impl Transceiver {
    /// Changes the receive direction of the specified [`Transceiver`].
    #[must_use]
    pub fn set_recv(&self, active: bool) -> LocalBoxFuture<'static, ()> {
        let handle = self.0.get();
        Box::pin(async move {
            let fut = unsafe { transceiver::set_recv(handle, active) };

            // TODO: Not supposed to error, but seems to. Log for further
            //       investigation.
            let res = unsafe { FutureFromDart::execute::<()>(fut) }.await;
            if let Err(e) = res {
                log::error!("Error in `Transceiver::set_recv`: {e}");
            }
        })
    }

    /// Changes the send direction of the specified [`Transceiver`].
    #[must_use]
    pub fn set_send(&self, active: bool) -> LocalBoxFuture<'static, ()> {
        let handle = self.0.get();
        Box::pin(async move {
            let fut = unsafe { transceiver::set_send(handle, active) };

            // TODO: Not supposed to error, but seems to. Log for further
            //       investigation.
            let res = unsafe { FutureFromDart::execute::<()>(fut) }.await;
            if let Err(e) = res {
                log::error!("Error in `Transceiver::set_send`: {e}");
            }
        })
    }

    /// Indicates whether the provided [`TransceiverDirection`] is enabled for
    /// this [`Transceiver`].
    pub async fn has_direction(&self, direction: TransceiverDirection) -> bool {
        self.direction().await.contains(direction)
    }

    /// Replaces [`TransceiverDirection::SEND`] [`local::Track`] of this
    /// [`Transceiver`].
    ///
    /// # Errors
    ///
    /// Errors with [`platform::Error`] if the underlying [`replaceTrack`][1]
    /// call fails.
    ///
    /// [`Error`]: platform::Error
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpsender-replacetrack
    pub async fn set_send_track(
        &self,
        new_track: Option<&Rc<local::Track>>,
    ) -> Result<(), platform::Error> {
        let fut = new_track.map_or_else(
            || unsafe { transceiver::drop_sender(self.0.get()) },
            |track| unsafe {
                transceiver::replace_track(
                    self.0.get(),
                    track.platform_track().handle(),
                )
            },
        );
        unsafe { FutureFromDart::execute::<()>(fut) }.await
    }

    /// Returns [`mid`] of this [`Transceiver`].
    ///
    /// [`mid`]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn mid(&self) -> Option<String> {
        let mid = unsafe { transceiver::mid(self.0.get()) };
        unsafe { (*Box::from_raw(mid.as_ptr())).try_into().unwrap() }
    }

    /// Indicates whether the underlying [RTCRtpTransceiver] is stopped.
    ///
    /// [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver
    #[must_use]
    pub fn is_stopped(&self) -> bool {
        unsafe { transceiver::is_stopped(self.0.get()) }
    }

    /// Returns current [`TransceiverDirection`] of this [`Transceiver`].
    fn direction(&self) -> impl Future<Output = TransceiverDirection> {
        let handle = self.0.get();
        async move {
            let fut = unsafe { transceiver::get_direction(handle) };
            unsafe { FutureFromDart::execute::<i32>(fut) }
                .await
                .unwrap()
                .into()
        }
    }
}

impl Drop for Transceiver {
    fn drop(&mut self) {
        if Rc::get_mut(&mut self.0).is_some() {
            let transceiver = Rc::clone(&self.0);
            platform::spawn(async move {
                let fut = unsafe { transceiver::dispose(transceiver.get()) };
                unsafe { FutureFromDart::execute::<()>(fut) }.await.unwrap();
            });
        }
    }
}
