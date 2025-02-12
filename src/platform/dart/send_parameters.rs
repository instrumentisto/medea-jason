//! Wrapper around [RTCRtpParameters].
//!
//! [RTCRtpParameters]: https://w3.org/TR/webrtc#dom-rtcrtpparameters

use dart_sys::Dart_Handle;
use futures::future::LocalBoxFuture;
use medea_macro::dart_bridge;

use crate::platform::{dart::utils::handle::DartHandle, Error};

use super::{
    send_encoding_parameters::SendEncodingParameters,
    utils::{dart_future::FutureFromDart, list::DartList},
};

#[dart_bridge("flutter/lib/src/native/platform/parameters.g.dart")]
mod parameters {
    use dart_sys::Dart_Handle;

    use crate::platform::Error;

    extern "C" {
        /// Returns [RTCRtpEncodingParameters][1] from the provided
        /// [RTCRtpParameters].
        ///
        /// [RTCRtpParameters]: https://w3.org/TR/webrtc#dom-rtcrtpparameters
        /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        pub fn encodings(parameters: Dart_Handle)
            -> Result<Dart_Handle, Error>;

        /// Sets the provided [RTCRtpEncodingParameters][1] into the provided
        /// [RTCRtpParameters].
        ///
        /// [RTCRtpParameters]: https://w3.org/TR/webrtc#dom-rtcrtpparameters
        /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
        pub fn set_encoding(
            parameters: Dart_Handle,
            encoding: Dart_Handle,
        ) -> Result<Dart_Handle, Error>;
    }
}

/// Representation of [RTCRtpSendParameters][0].
///
/// [0]: https://w3.org/TR/webrtc/#dom-rtcrtpsendparameters
#[derive(Clone, Debug)]
pub struct SendParameters(DartHandle);

impl From<DartHandle> for SendParameters {
    fn from(from: DartHandle) -> Self {
        Self(from)
    }
}

impl SendParameters {
    /// Returns [`SendEncodingParameters`] of these [`SendParameters`].
    #[must_use]
    pub async fn encodings(&self) -> Vec<SendEncodingParameters> {
        let fut = unsafe { parameters::encodings(self.0.get()) }.unwrap();
        let encodings = unsafe { FutureFromDart::execute::<DartHandle>(fut) }
            .await
            .unwrap();

        let encodings = Vec::from(DartList::from(encodings))
            .into_iter()
            .map(|encoding: DartHandle| SendEncodingParameters::from(encoding))
            .collect();

        Ok(encodings)
    }

    /// Sets the provided [`SendEncodingParameters`] into these
    /// [`SendParameters`].
    #[must_use]
    pub async fn set_encodings(&self, encoding: &SendEncodingParameters) {
        let fut = unsafe {
            parameters::set_encoding(self.0.get(), encoding.handle())
        }
        .unwrap();
        unsafe { FutureFromDart::execute::<()>(fut) }.await.unwrap();
    }

    /// Returns the underlying [`Dart_Handle`] of these [`SendParameters`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.0.get()
    }
}
