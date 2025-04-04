//! [RTCRtpTransceiver] wrapper.
//!
//! [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver

use std::rc::Rc;

use dart_sys::Dart_Handle;
use futures::future::LocalBoxFuture;
use medea_macro::dart_bridge;

use super::{
    codec_capability::CodecCapability,
    send_encoding_parameters::SendEncodingParameters,
    send_parameters::SendParameters,
};
use crate::{
    media::track::local,
    platform::{
        self, TransceiverDirection,
        dart::utils::{
            dart_future::FutureFromDart, handle::DartHandle, list::DartList,
        },
    },
};

#[dart_bridge("flutter/lib/src/native/platform/transceiver.g.dart")]
mod transceiver {
    use std::ptr;

    use dart_sys::Dart_Handle;

    use crate::{api::DartValueArg, platform::Error};

    extern "C" {
        /// Returns current direction of the provided [`Transceiver`].
        pub fn get_direction(
            transceiver: Dart_Handle,
        ) -> Result<Dart_Handle, Error>;

        /// Replaces `Send` [`MediaStreamTrack`] of the provided
        /// [`Transceiver`].
        pub fn replace_track(
            transceiver: Dart_Handle,
            track: Dart_Handle,
        ) -> Result<Dart_Handle, Error>;

        /// Drops `Send` [`MediaStreamTrack`] of the provided [`Transceiver`].
        pub fn drop_sender(
            transceiver: Dart_Handle,
        ) -> Result<Dart_Handle, Error>;

        /// Returns stopped status of the provided [`Transceiver`].
        pub fn is_stopped(transceiver: Dart_Handle) -> Result<bool, Error>;

        /// Returns MID of the provided [`Transceiver`].
        pub fn mid(
            transceiver: Dart_Handle,
        ) -> Result<ptr::NonNull<DartValueArg<Option<String>>>, Error>;

        /// Changes the receive direction of the specified [`Transceiver`].
        pub fn set_recv(
            transceiver: Dart_Handle,
            active: bool,
        ) -> Result<Dart_Handle, Error>;

        /// Changes the send direction of the specified [`Transceiver`].
        pub fn set_send(
            transceiver: Dart_Handle,
            active: bool,
        ) -> Result<Dart_Handle, Error>;

        /// Disposes the provided [`Transceiver`].
        pub fn dispose(transceiver: Dart_Handle) -> Result<Dart_Handle, Error>;

        /// Creates a new [`TransceiverInit`].
        pub fn create_transceiver_init(
            direction: i64,
        ) -> Result<Dart_Handle, Error>;

        /// Adds [`SendEncodingParameters`] to the provided [`TransceiverInit`].
        pub fn add_sending_encodings(
            transceiver_init: Dart_Handle,
            encoding: Dart_Handle,
        ) -> Result<(), Error>;

        /// Returns [`Parameters`] of the underlying [RTCRtpSender].
        ///
        /// [RTCRtpSender]: https://w3.org/TR/webrtc#rtcrtpsender-interface
        pub fn get_send_parameters(
            transceiver: Dart_Handle,
        ) -> Result<Dart_Handle, Error>;

        /// Sets [`Parameters`] into the underlying [RTCRtpSender].
        ///
        /// [RTCRtpSender]: https://w3.org/TR/webrtc#rtcrtpsender-interface
        pub fn set_send_parameters(
            transceiver: Dart_Handle,
            parameters: Dart_Handle,
        ) -> Result<Dart_Handle, Error>;

        /// Overrides the default receive codec preferences, used by the user
        /// agent for the provided [`Transceiver`].
        pub fn set_codec_preferences(
            transceiver: Dart_Handle,
            codec_capabilities: Dart_Handle,
        ) -> Result<(), Error>;
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
            let fut = unsafe { transceiver::set_recv(handle, active) }.unwrap();

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
            let fut = unsafe { transceiver::set_send(handle, active) }.unwrap();

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
            || unsafe { transceiver::drop_sender(self.0.get()) }.unwrap(),
            |track| {
                unsafe {
                    transceiver::replace_track(
                        self.0.get(),
                        track.platform_track().handle(),
                    )
                }
                .unwrap()
            },
        );
        unsafe { FutureFromDart::execute::<()>(fut) }.await
    }

    /// Returns [`mid`] of this [`Transceiver`].
    ///
    /// [`mid`]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    #[expect(clippy::unwrap_in_result, reason = "unrelated")]
    #[must_use]
    pub fn mid(&self) -> Option<String> {
        let mid = unsafe { transceiver::mid(self.0.get()) }.unwrap();
        unsafe { (*Box::from_raw(mid.as_ptr())).try_into().unwrap() }
    }

    /// Indicates whether the underlying [RTCRtpTransceiver] is stopped.
    ///
    /// [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver
    #[must_use]
    pub fn is_stopped(&self) -> bool {
        unsafe { transceiver::is_stopped(self.0.get()) }.unwrap()
    }

    /// Returns current [`TransceiverDirection`] of this [`Transceiver`].
    async fn direction(&self) -> TransceiverDirection {
        let fut = unsafe { transceiver::get_direction(self.0.get()) }.unwrap();
        unsafe { FutureFromDart::execute::<i32>(fut) }.await.unwrap().into()
    }

    /// Returns [`SendParameters`] of the underlying [RTCRtpSender].
    ///
    /// [RTCRtpSender]: https://w3.org/TR/webrtc#rtcrtpsender-interface
    pub async fn get_send_parameters(&self) -> SendParameters {
        let fut =
            unsafe { transceiver::get_send_parameters(self.0.get()) }.unwrap();
        let params: DartHandle =
            unsafe { FutureFromDart::execute(fut) }.await.unwrap();

        SendParameters::from(params)
    }

    /// Sets [`SendParameters`] into the underlying [RTCRtpSender].
    ///
    /// # Errors
    ///
    /// With [`platform::Error`] if the underlying [setParameters()][1] call
    /// fails.
    ///
    /// [RTCRtpSender]: https://w3.org/TR/webrtc#rtcrtpsender-interface
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpsender-setparameters
    pub async fn set_send_parameters(
        &self,
        params: SendParameters,
    ) -> Result<(), platform::Error> {
        let handle = self.0.get();
        let params_handle = params.handle();
        let fut =
            unsafe { transceiver::set_send_parameters(handle, params_handle) }
                .unwrap();

        unsafe { FutureFromDart::execute::<()>(fut) }.await
    }

    /// Sets preferred [`CodecCapability`] for this [`Transceiver`].
    pub fn set_codec_preferences(
        &self,
        preferred_codecs: Vec<CodecCapability>,
    ) {
        let handle = self.0.get();
        let mut codecs_dart = DartList::new();
        for codec in preferred_codecs {
            let codec_handle = codec.handle();
            codecs_dart.add(codec_handle.into());
        }
        unsafe {
            transceiver::set_codec_preferences(handle, codecs_dart.handle())
        }
        .unwrap();
    }
}

impl Drop for Transceiver {
    fn drop(&mut self) {
        if Rc::get_mut(&mut self.0).is_some() {
            let transceiver = Rc::clone(&self.0);
            platform::spawn(async move {
                let fut =
                    unsafe { transceiver::dispose(transceiver.get()) }.unwrap();
                unsafe { FutureFromDart::execute::<()>(fut) }.await.unwrap();
            });
        }
    }
}

/// Dart side representation of an [RTCRtpTransceiverInit].
///
/// [RTCRtpTransceiverInit]: https://w3.org/TR/webrtc#dom-rtcrtptransceiverinit
#[derive(Debug)]
pub struct TransceiverInit(DartHandle);

impl TransceiverInit {
    /// Creates a new [`TransceiverInit`].
    #[must_use]
    pub fn new(direction: TransceiverDirection) -> Self {
        let handle =
            unsafe { transceiver::create_transceiver_init(direction.into()) }
                .unwrap();
        Self(unsafe { DartHandle::new(handle) })
    }

    /// Returns the underlying [`Dart_Handle`] of this [`TransceiverInit`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.0.get()
    }

    /// Adds the provided [`SendEncodingParameters`] to this
    /// [`TransceiverInit`].
    pub fn set_send_encodings(&self, encodings: Vec<SendEncodingParameters>) {
        for encoding in encodings {
            unsafe {
                transceiver::add_sending_encodings(
                    self.0.get(),
                    encoding.handle(),
                )
            }
            .unwrap();
        }
    }
}
