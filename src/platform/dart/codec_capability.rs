//! [SendEncodingParameters] wrapper.
//!
//! [SendEncodingParameters]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver

use dart_sys::Dart_Handle;
use medea_macro::dart_bridge;

use crate::{
    media::MediaKind,
    platform::{
        codec_capability::CodecCapabilityError as Error,
        dart::utils::handle::DartHandle,
    },
};

use super::utils::{
    dart_future::FutureFromDart, dart_string_into_rust, list::DartList,
};

#[dart_bridge("flutter/lib/src/native/platform/codec_capability.g.dart")]
mod codec_capability {
    use std::{ffi::c_char, ptr};

    use dart_sys::Dart_Handle;

    extern "C" {
        /// Gets [`RTCRtpSender`]'s available [`RTCRtpCodecCapability`]s.
        ///
        /// [`RTCRtpSender`]: https://w3c.github.io/webrtc-pc/#dom-rtcrtpsender
        /// [`RTCRtpCodecCapability`]: https://tinyurl.com/4jcp8m4s
        pub fn get_sender_codec_capabilities(kind: i64) -> Dart_Handle;

        /// Gets [`RTCRtpCodecCapability.mimeType`] of the provided
        /// [`RTCRtpCodecCapability`].
        ///
        /// [`RTCRtpCodecCapability.mimeType`]: https://tinyurl.com/yv38zr3a
        /// [`RTCRtpCodecCapability`]: https://tinyurl.com/4jcp8m4s
        pub fn mime_type(codec_capability: Dart_Handle)
            -> ptr::NonNull<c_char>;
    }
}

/// Dart side representation of [`RTCRtpCodecCapability`].
///
/// [`RTCRtpCodecCapability`]: https://tinyurl.com/4jcp8m4s
#[derive(Clone, Debug)]
pub struct CodecCapability(DartHandle);

impl From<DartHandle> for CodecCapability {
    fn from(value: DartHandle) -> Self {
        Self(value)
    }
}

impl CodecCapability {
    /// Gets available [`RTCRtpSender`]'s [`CodecCapability`]s.
    ///
    /// # Errors
    ///
    /// Errors with [`Error::FailedToGetCapabilities`] if fails to get
    /// capabilities.
    ///
    /// [`RTCRtpSender`]: https://w3c.github.io/webrtc-pc/#dom-rtcrtpsender
    pub async fn get_sender_codec_capabilities(
        kind: MediaKind,
    ) -> Result<Vec<Self>, Error> {
        let fut = unsafe {
            codec_capability::get_sender_codec_capabilities(kind as i64)
        };

        #[allow(clippy::map_err_ignore)]
        let res: DartHandle = unsafe { FutureFromDart::execute(fut) }
            .await
            .map_err(|_| Error::FailedToGetCapabilities)?;

        Ok(Vec::from(DartList::from(res))
            .into_iter()
            .map(|caps: DartHandle| Self::from(caps))
            .collect())
    }

    /// Gets `mime_type` of this [`CodecCapability`]s.
    ///
    /// # Errors
    ///
    /// Never errors, but [`Result`] is needed for consistency with WASM
    /// implementation.
    pub fn mime_type(&self) -> Result<String, Error> {
        let mime_type = unsafe { codec_capability::mime_type(self.0.get()) };
        Ok(unsafe { dart_string_into_rust(mime_type) })
    }

    /// Returns the underlying [`Dart_Handle`] of this [`CodecCapability`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.0.get()
    }
}
