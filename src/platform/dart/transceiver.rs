//! [RTCRtpTransceiver] wrapper.
//!
//! [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver

use std::{future::Future, rc::Rc};

use derive_more::From;
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

        /// Sets `direction` of this [`Transceiver`].
        pub fn set_direction(
            transceiver: Dart_Handle,
            direction: i64,
        ) -> Dart_Handle;

        /// Sets receive of this [`Transceiver`].
        pub fn set_recv(transceiver: Dart_Handle, recv: bool) -> Dart_Handle;

        /// Sets send of this [`Transceiver`].
        pub fn set_send(transceiver: Dart_Handle, sens: bool) -> Dart_Handle;
    }
}

/// Wrapper around [RTCRtpTransceiver] which provides handy methods for
/// direction changes.
///
/// [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver
#[derive(Clone, Debug, From)]
pub struct Transceiver(DartHandle);

impl Transceiver {
    /// Sets this [`Transceiver`] receive to the `recv`.
    #[must_use]
    pub fn set_recv(&self, recv: bool) -> LocalBoxFuture<'static, ()> {
        let handle = self.0.get();
        Box::pin(async move {
            unsafe {
                FutureFromDart::execute::<()>(transceiver::set_recv(
                    handle, recv,
                ))
                .await
                .unwrap();
            }
        })
    }

    /// Sets this [`Transceiver`] send to the `send`.
    #[must_use]
    pub fn set_send(&self, send: bool) -> LocalBoxFuture<'static, ()> {
        let handle = self.0.get();
        Box::pin(async move {
            unsafe {
                FutureFromDart::execute::<()>(transceiver::set_send(
                    handle, send,
                ))
                .await
                .unwrap();
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
        if let Some(track) = new_track {
            unsafe {
                FutureFromDart::execute::<()>(transceiver::replace_track(
                    self.0.get(),
                    track.platform_track().handle(),
                ))
                .await
            }?;
        } else {
            unsafe {
                FutureFromDart::execute::<()>(transceiver::drop_sender(
                    self.0.get(),
                ))
                .await
            }?;
        }
        Ok(())
    }

    /// Returns [`mid`] of this [`Transceiver`].
    ///
    /// [`mid`]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn mid(&self) -> Option<String> {
        unsafe {
            let mid = transceiver::mid(self.0.get());
            (*Box::from_raw(mid.as_ptr())).try_into().unwrap()
        }
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
            unsafe {
                FutureFromDart::execute::<i32>(transceiver::get_direction(
                    handle,
                ))
                .await
            }
            .unwrap()
            .into()
        }
    }

    /// Sets this [`Transceiver`] to the provided [`TransceiverDirection`].
    #[allow(dead_code)]
    fn set_direction(
        &self,
        direction: TransceiverDirection,
    ) -> LocalBoxFuture<'static, ()> {
        let handle = self.0.get();
        Box::pin(async move {
            unsafe {
                FutureFromDart::execute::<()>(transceiver::set_direction(
                    handle,
                    direction.into(),
                ))
                .await
                .unwrap();
            }
        })
    }
}
