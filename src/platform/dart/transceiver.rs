//! [RTCRtpTransceiver] wrapper.
//!
//! [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver

use std::{cell::RefCell, future::Future, rc::Rc};

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

        /// Sets `enabled` field of `Send` [`MediaStreamTrack`] of the provided
        /// [`Transceiver`].
        pub fn set_send_track_enabled(transceiver: Dart_Handle, enabled: bool);

        /// Returns MID of the provided [`Transceiver`].
        pub fn mid(
            transceiver: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        /// Returns `1` if the provided [`Transceiver`] has `Send`
        /// [`MediaStreamTrack`].
        pub fn has_send_track(transceiver: Dart_Handle) -> bool;

        /// Sets `direction` of this [`Transceiver`].
        pub fn set_direction(
            transceiver: Dart_Handle,
            direction: i64,
        ) -> Dart_Handle;

        /// todo
        pub fn add_direction(
            transceiver: Dart_Handle,
            direction: i64,
        ) -> Dart_Handle;

        /// todo
        pub fn sub_direction(
            transceiver: Dart_Handle,
            direction: i64,
        ) -> Dart_Handle;
    }
}

/// Wrapper around [RTCRtpTransceiver] which provides handy methods for
/// direction changes.
///
/// [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver
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
    ) -> LocalBoxFuture<'static, Result<(), platform::dart::error::Error>> {
        let this = self.clone();
        Box::pin(async move {
            unsafe {
                FutureFromDart::execute::<()>(transceiver::sub_direction(
                    this.transceiver.get(),
                    disabled_direction.into(),
                ))
                .await
            }
        })
    }

    /// Enables provided [`TransceiverDirection`] of this [`Transceiver`].
    pub fn add_direction(
        &self,
        enabled_direction: TransceiverDirection,
    ) -> LocalBoxFuture<'static, ()> {
        let this = self.clone();
        Box::pin(async move {
            unsafe {
                let res = FutureFromDart::execute::<()>(transceiver::add_direction(
                    this.transceiver.get(),
                    enabled_direction.into(),
                ))
                .await.unwrap();
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
        new_sender: Rc<local::Track>,
    ) -> Result<(), platform::Error> {
        unsafe {
            FutureFromDart::execute::<()>(transceiver::replace_track(
                self.transceiver.get(),
                new_sender.platform_track().handle(),
            ))
            .await
        }
        .unwrap();
        drop(self.send_track.replace(Some(new_sender)));
        Ok(())
    }

    /// Sets a [`TransceiverDirection::SEND`] [`local::Track`] of this
    /// [`Transceiver`] to [`None`].
    pub fn drop_send_track(&self) -> impl Future<Output = ()> {
        drop(self.send_track.borrow_mut().take());
        let transceiver = self.transceiver.get();
        async move {
            unsafe {
                FutureFromDart::execute::<()>(transceiver::drop_sender(
                    transceiver,
                ))
                .await
            }
            .unwrap();
        }
    }

    /// Returns [`mid`] of this [`Transceiver`].
    ///
    /// [`mid`]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    #[allow(clippy::unwrap_in_result)]
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
        unsafe { transceiver::has_send_track(self.transceiver.get()) }
    }

    /// Sets the underlying [`local::Track`]'s `enabled` field to the provided
    /// value, if any.
    pub fn set_send_track_enabled(&self, enabled: bool) {
        unsafe {
            transceiver::set_send_track_enabled(
                self.transceiver.get(),
                enabled,
            );
        }
    }

    /// Indicates whether the underlying [RTCRtpTransceiver] is stopped.
    ///
    /// [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver
    #[must_use]
    pub fn is_stopped(&self) -> bool {
        unsafe { transceiver::is_stopped(self.transceiver.get()) }
    }

    /// Returns current [`TransceiverDirection`] of this [`Transceiver`].
    fn direction(&self) -> impl Future<Output = TransceiverDirection> {
        let handle = self.transceiver.get();
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
    ) -> LocalBoxFuture<'static, Result<(), platform::dart::error::Error>> {
        let handle = self.transceiver.get();
        Box::pin(async move {
            unsafe {
                FutureFromDart::execute::<()>(transceiver::set_direction(
                    handle,
                    direction.into(),
                ))
                .await
            }
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
