//! Wrapper around [RTCRtpParameters].
//!
//! [RTCRtpParameters]: https://w3.org/TR/webrtc#dom-rtcrtpparameters

use dart_sys::Dart_Handle;
use derive_more::From;
use medea_macro::dart_bridge;

use super::{
    send_encoding_parameters::SendEncodingParameters, utils::list::DartList,
};
use crate::platform::dart::utils::handle::DartHandle;

#[dart_bridge("flutter/lib/src/native/platform/send_parameters.g.dart")]
mod send_parameters {
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
    }
}

/// Representation of [RTCRtpSendParameters][0].
///
/// [0]: https://w3.org/TR/webrtc#dom-rtcrtpsendparameters
#[derive(Clone, Debug, From)]
pub struct SendParameters(DartHandle);

impl SendParameters {
    /// Returns [`SendEncodingParameters`] of these [`SendParameters`].
    #[must_use]
    pub fn encodings(&self) -> Box<[SendEncodingParameters]> {
        let encodings =
            unsafe { send_parameters::encodings(self.0.get()) }.unwrap();

        let encodings: Vec<_> =
            Vec::from(DartList::from(unsafe { DartHandle::new(encodings) }))
                .into_iter()
                .map(|encoding: DartHandle| {
                    SendEncodingParameters::from(encoding)
                })
                .collect();

        encodings.into_boxed_slice()
    }

    /// Returns the underlying [`Dart_Handle`] of these [`SendParameters`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.0.get()
    }
}
