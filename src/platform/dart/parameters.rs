//! Wrapper around [RTCRtpParameters][1].
//!
//! [1]: https://w3.org/TR/webrtc/#dom-rtcrtpparameters

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

    extern "C" {
        /// Gets [`SendEncodingParameters`] from the provided [`Parameters`].
        pub fn encodings(parameters: Dart_Handle) -> Dart_Handle;

        /// Sets the provided [`SendEncodingParameters`] in the provided
        /// [`Parameters`].
        pub fn set_encoding(
            parameters: Dart_Handle,
            encoding: Dart_Handle,
        ) -> Dart_Handle;
    }
}

/// Representation of [RTCRtpParameters][1].
///
/// [1]: https://w3.org/TR/webrtc/#dom-rtcrtpparameters
#[derive(Clone, Debug)]
pub struct Parameters(DartHandle);

impl From<DartHandle> for Parameters {
    fn from(from: DartHandle) -> Self {
        Self(from)
    }
}

impl Parameters {
    /// Returns [`SendEncodingParameters`] of this [`Parameters`].
    #[must_use]
    pub fn encodings(
        &self,
    ) -> LocalBoxFuture<'static, Result<Vec<SendEncodingParameters>, Error>>
    {
        let handle = self.0.get();

        Box::pin(async move {
            let fut = unsafe { parameters::encodings(handle) };
            let encodings =
                unsafe { FutureFromDart::execute::<DartHandle>(fut) }.await?;

            let encodings = Vec::from(DartList::from(encodings))
                .into_iter()
                .map(|encoding: DartHandle| {
                    SendEncodingParameters::from(encoding)
                })
                .collect();

            Ok(encodings)
        })
    }

    /// Sets the provided [`SendEncodingParameters`] in this [`Parameters`].
    #[must_use]
    pub fn set_encoding(
        &self,
        encoding: &SendEncodingParameters,
    ) -> LocalBoxFuture<'static, ()> {
        let handle = self.0.get();
        let enc_handle = encoding.handle();
        Box::pin(async move {
            let fut = unsafe { parameters::set_encoding(handle, enc_handle) };
            unsafe { FutureFromDart::execute::<()>(fut) }.await.unwrap();
        })
    }

    /// Returns the underlying [`Dart_Handle`] of this [`Parameters`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.0.get()
    }
}
