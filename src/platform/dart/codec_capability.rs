//! Wrapper around an [RTCRtpCodecCapability].
//!
//! [RTCRtpCodecCapability]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability

use std::collections::HashMap;

use dart_sys::Dart_Handle;
use medea_macro::dart_bridge;

use crate::{
    media::MediaKind,
    platform::{
        codec_capability::CodecCapabilityError as Error,
        dart::utils::{handle::DartHandle, NonNullDartValueArgExt as _},
    },
};

use super::utils::{
    dart_future::FutureFromDart, dart_string_into_rust, list::DartList,
};

#[dart_bridge("flutter/lib/src/native/platform/codec_capability.g.dart")]
mod codec_capability {
    use std::{ffi::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::{api::DartValueArg, platform::Error};

    extern "C" {
        /// Returns [RTCRtpSender]'s available [RTCRtpCodecCapability][1]s.
        ///
        /// [RTCRtpSender]: https://w3.org/TR/webrtc#dom-rtcrtpsender
        /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability
        pub fn get_sender_codec_capabilities(
            kind: i64,
        ) -> Result<Dart_Handle, Error>;

        pub fn get_receiver_codec_capabilities(
            kind: i64,
        ) -> Result<Dart_Handle, Error>;

        /// Returns [mimeType][2] of the provided [RTCRtpCodecCapability][1].
        ///
        /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability
        /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability-mimetype
        pub fn mime_type(
            codec_capability: Dart_Handle,
        ) -> Result<ptr::NonNull<c_char>, Error>;

        pub fn clock_rate(
            codec_capability: Dart_Handle,
        ) -> Result<ptr::NonNull<DartValueArg<Option<u32>>>, Error>;

        pub fn channels(
            codec_capability: Dart_Handle,
        ) -> Result<ptr::NonNull<DartValueArg<Option<u16>>>, Error>;

        /// Returns [mimeType][2] of the provided [RTCRtpCodecCapability][1].
        ///
        /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability
        /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability-mimetype
        pub fn parameters(
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

        #[expect(clippy::map_err_ignore, reason = "not useful")]
        let res: DartHandle = unsafe { FutureFromDart::execute(fut) }
            .await
            .map_err(|_| Error::FailedToGetCapabilities)?;

        Ok(Vec::from(DartList::from(res))
            .into_iter()
            .map(|caps: DartHandle| Self::from(caps))
            .collect())
    }

    /// Returns available [RTCRtpSender]'s [`CodecCapability`]s.
    ///
    /// # Errors
    ///
    /// With [`Error::FailedToGetCapabilities`] if fails to retrieve
    /// capabilities.
    ///
    /// [RTCRtpSender]: https://w3.org/TR/webrtc#dom-rtcrtpsender
    pub async fn get_receiver_codec_capabilities(
        kind: MediaKind,
    ) -> Result<Vec<Self>, Error> {
        let fut = unsafe {
            codec_capability::get_receiver_codec_capabilities(kind as i64)
        }
        .unwrap();

        #[expect(clippy::map_err_ignore, reason = "not useful")]
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
    /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability-mimetype
    #[must_use]
    pub fn mime_type(&self) -> String {
        let mime_type =
            unsafe { codec_capability::mime_type(self.0.get()) }.unwrap();
        unsafe { dart_string_into_rust(mime_type) }
    }

    /// Returns the codec clock rate expressed in Hertz.
    #[expect(clippy::unwrap_in_result, reason = "unrelated")]
    #[inline]
    #[must_use]
    pub fn clock_rate(&self) -> Option<u32> {
        let clock_rate =
            unsafe { codec_capability::clock_rate(self.0.get()) }.unwrap();
        Option::try_from(unsafe { clock_rate.unbox() }).unwrap()
    }

    /// Returns the maximum number of channels (mono=1, stereo=2).
    #[expect(clippy::unwrap_in_result, reason = "unrelated")]
    #[inline]
    #[must_use]
    pub fn channels(&self) -> Option<u16> {
        let channels =
            unsafe { codec_capability::channels(self.0.get()) }.unwrap();
        Option::try_from(unsafe { channels.unbox() }).unwrap()
    }

    /// Returns codec-specific parameters
    #[must_use]
    pub fn parameters(&self) -> HashMap<String, String> {
        let params_json_ptr =
            unsafe { codec_capability::parameters(self.0.get()) }.unwrap();
        let params_json = unsafe { dart_string_into_rust(params_json_ptr) };

        serde_json::from_str(&params_json).unwrap_or_else(|_| {
            log::error!("Failed to parse codec params: {params_json}");
            HashMap::new()
        })
    }

    /// Returns the underlying [`Dart_Handle`] of this [`CodecCapability`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.0.get()
    }
}
