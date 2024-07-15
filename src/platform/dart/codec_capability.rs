//! Wrapper around an [RTCRtpCodecCapability].
//!
//! [RTCRtpCodecCapability]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability

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

    use crate::platform::Error;

    extern "C" {
        /// Returns [RTCRtpSender]'s available [RTCRtpCodecCapability][1]s.
        ///
        /// [RTCRtpSender]: https://w3.org/TR/webrtc#dom-rtcrtpsender
        /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability
        pub fn get_sender_codec_capabilities(
            kind: i64,
        ) -> Result<Dart_Handle, Error>;

        /// Returns [mimeType][2] of the provided [RTCRtpCodecCapability][1].
        ///
        /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability
        /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability-mimetype
        pub fn mime_type(
            codec_capability: Dart_Handle,
        ) -> Result<ptr::NonNull<c_char>, Error>;
    }
}

/// Dart side representation of an [RTCRtpCodecCapability].
///
/// [RTCRtpCodecCapability]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability
#[derive(Clone, Debug)]
pub struct CodecCapability(DartHandle);

impl From<DartHandle> for CodecCapability {
    fn from(value: DartHandle) -> Self {
        Self(value)
    }
}

impl CodecCapability {
    /// Returns available [RTCRtpSender]'s [`CodecCapability`]s.
    ///
    /// # Errors
    ///
    /// With [`Error::FailedToGetCapabilities`] if fails to retrieve
    /// capabilities.
    ///
    /// [RTCRtpSender]: https://w3.org/TR/webrtc#dom-rtcrtpsender
    pub async fn get_sender_codec_capabilities(
        kind: MediaKind,
    ) -> Result<Vec<Self>, Error> {
        let fut = unsafe {
            codec_capability::get_sender_codec_capabilities(kind as i64)
        }
        .unwrap();

        #[allow(clippy::map_err_ignore)]
        let res: DartHandle = unsafe { FutureFromDart::execute(fut) }
            .await
            .map_err(|_| Error::FailedToGetCapabilities)?;

        Ok(Vec::from(DartList::from(res))
            .into_iter()
            .map(|caps: DartHandle| Self::from(caps))
            .collect())
    }

    /// Returns [MIME media type][2] of this [`CodecCapability`].
    ///
    /// # Errors
    ///
    /// Never errors, but [`Result`] is needed for consistency with WASM
    /// implementation.
    ///
    /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability-mimetype
    #[allow(clippy::unwrap_in_result)] // intentional
    pub fn mime_type(&self) -> Result<String, Error> {
        let mime_type =
            unsafe { codec_capability::mime_type(self.0.get()) }.unwrap();
        Ok(unsafe { dart_string_into_rust(mime_type) })
    }

    /// Returns the underlying [`Dart_Handle`] of this [`CodecCapability`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.0.get()
    }
}
