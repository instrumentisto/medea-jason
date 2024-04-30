//! [RTCRtpTransceiver] wrapper.
//!
//! [RTCRtpTransceiver]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver

use std::{future::Future, rc::Rc};

use dart_sys::_Dart_Handle;
use futures::future::LocalBoxFuture;
use medea_client_api_proto::EncodingParameters;
use medea_macro::dart_bridge;

use crate::{
    media::track::local,
    platform::{
        self,
        dart::utils::{
            dart_future::FutureFromDart, handle::DartHandle, list::DartList,
        },
        TransceiverDirection,
    },
};

use super::{
    codec_capability::CodecCapability, parameters::Parameters,
    send_encoding_parameters::SendEncodingParameters,
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

        /// Creates a new [`TransceiverInit`].
        pub fn create_transceiver_init(direction: i64) -> Dart_Handle;

        /// Adds [`SendEncodingParameters`] to the provided [`TransceiverInit`]
        pub fn add_sending_encodings(
            transceiver_init: Dart_Handle,
            encoding: Dart_Handle,
        );

        /// Gets [`Parameters`] of the underlying `sender`.
        pub fn get_send_parameters(transceiver: Dart_Handle) -> Dart_Handle;

        /// Sets [`Parameters`] into the underlying `sender`.
        pub fn set_send_parameters(
            transceiver: Dart_Handle,
            parameters: Dart_Handle,
        ) -> Dart_Handle;

        pub fn set_preferred_codec(
            transceiver: Dart_Handle,
            codec_capabilities: Dart_Handle,
        );
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

    /// Gets [`Parameters`] of the underlying `sender`.
    pub fn get_send_parameters(&self) -> impl Future<Output = Parameters> {
        let handle = self.0.get();
        async move {
            let fut = unsafe { transceiver::get_send_parameters(handle) };
            let params: DartHandle =
                unsafe { FutureFromDart::execute(fut) }.await.unwrap();
            Parameters::from(params)
        }
    }

    /// Sets [`Parameters`] into the underlying `sender`.
    ///
    /// # Errors
    ///
    /// Errors with [`platform::Error`] if the underlying [`setParameters`][1]
    /// call fails.
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-rtcrtpsender-setparameters
    pub fn set_send_parameters(
        &self,
        params: Parameters,
    ) -> impl Future<Output = Result<(), platform::Error>> {
        let handle = self.0.get();
        let params_handle = params.handle();
        async move {
            let fut = unsafe {
                transceiver::set_send_parameters(handle, params_handle)
            };

            unsafe { FutureFromDart::execute::<()>(fut) }.await?;

            Ok(())
        }
    }

    /// Sets preferred [`CodecCapability`] for this [`Transceiver`].
    pub fn set_preferred_codecs(&self, preferred_codecs: Vec<CodecCapability>) {
        let handle = self.0.get();
        let mut codecs_dart = DartList::new();
        for codec in preferred_codecs {
            let codec_handle = codec.handle();
            codecs_dart.add(codec_handle.into());
        }
        unsafe {
            transceiver::set_preferred_codec(handle, codecs_dart.as_handle());
        };
    }

    /// Updates parameters of encoding for underlying `sender`.
    ///
    /// # Errors
    ///
    /// Errors with [`platform::Error`] if the underlying [`setParameters`][1]
    /// call fails.
    ///
    /// [1]: https://w3.org/TR/webrtc/#dom-rtcrtpsender-setparameters
    pub async fn update_send_encodings(
        &self,
        encodings: Vec<EncodingParameters>,
    ) -> Result<(), platform::Error> {
        let params = self.get_send_parameters().await;

        let encs = params.encodings().await?;
        for mut enc in encs {
            let rid = enc.rid();

            let Some(encoding) = encodings.iter().find(|e| e.rid == rid) else {
                continue;
            };

            enc.set_active(encoding.active);
            if let Some(max_bitrate) = encoding.max_bitrate {
                enc.set_max_bitrate(max_bitrate.into());
            }
            if let Some(scale_resolution_down_by) =
                encoding.scale_resolution_down_by
            {
                enc.set_scale_resolution_down_by(
                    scale_resolution_down_by.into(),
                );
            }

            params.set_encoding(&enc).await;
        }

        self.set_send_parameters(params).await?;

        Ok(())
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

/// Dart side representation of [RTCRtpTransceiverInit].
///
/// [RTCRtpTransceiverInit]: https://tinyurl.com/mtdkabcj
#[derive(Debug)]
pub struct TransceiverInit(DartHandle);

impl TransceiverInit {
    /// Creates a new [`TransceiverInit`].
    #[must_use]
    pub fn new(direction: TransceiverDirection) -> Self {
        let handle =
            unsafe { transceiver::create_transceiver_init(direction.into()) };
        Self(unsafe { DartHandle::new(handle) })
    }

    /// Returns underlying [`_Dart_Handle`].
    #[must_use]
    pub fn handle(&self) -> *mut _Dart_Handle {
        self.0.get()
    }

    /// Adds provided [`SendEncodingParameters`] to this [`TransceiverInit`].
    pub fn sending_encodings(
        &mut self,
        encodings: Vec<SendEncodingParameters>,
    ) {
        for encoding in encodings {
            unsafe {
                transceiver::add_sending_encodings(
                    self.0.get(),
                    encoding.handle(),
                );
            }
        }
    }
}
