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
    }
}

/// Wrapper around [RTCRtpTransceiver] which provides handy methods for
/// direction changes.
///
/// [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver
#[derive(Clone, Debug, From)]
pub struct Transceiver(DartHandle);

impl Transceiver {
    /// Disables provided [`TransceiverDirection`] of this [`Transceiver`].
    #[must_use]
    pub fn sub_direction(
        &self,
        disabled_direction: TransceiverDirection,
    ) -> LocalBoxFuture<'static, ()> {
        let this = self.clone();
        Box::pin(async move {
            this.set_direction(this.direction().await - disabled_direction)
                .await;
        })
    }

    /// Enables provided [`TransceiverDirection`] of this [`Transceiver`].
    #[must_use]
    pub fn add_direction(
        &self,
        enabled_direction: TransceiverDirection,
    ) -> LocalBoxFuture<'static, ()> {
        let this = self.clone();
        Box::pin(async move {
            this.set_direction(this.direction().await | enabled_direction)
                .await;
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
            }
            .unwrap();
        } else {
            unsafe {
                FutureFromDart::execute::<()>(transceiver::drop_sender(
                    self.0.get(),
                ))
                .await
            }
            .unwrap();
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
            }
            .unwrap();
        })
    }

    /// TODO: add docs
    #[must_use]
    pub fn get_recv_track(&self) -> platform::MediaStreamTrack {
        todo!();
        // platform::MediaStreamTrack::new(self.0.receiver().track(), None)
    }
}
